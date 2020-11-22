use num::complex::Complex;
use std::{self, fs::File, path::Path};

use hound;
use png;
use std::io::BufWriter;

use std::env;
use std::f32;

fn fft(buffer: &mut Vec<Complex<f32>>) -> &mut Vec<Complex<f32>> {
    let n = buffer.len();
    if n == 1 {
        return buffer;
    }

    let r: f32 = -2.00 * f32::consts::PI / (n as f32);
    let root = Complex::new(r.cos(), r.sin());

    let p_even = &mut vec![];
    let p_odd = &mut vec![];

    for i in (0..n).step_by(2) {
        p_even.push(buffer[i]);
    }

    for i in (1..n).step_by(2) {
        p_odd.push(buffer[i]);
    }

    let even = fft(p_even);
    let odd = fft(p_odd);

    let mut r: Complex<f32> = Complex::new(1.00, 0.00);
    for i in 0..n / 2 {
        buffer[i] = even[i] + r * odd[i];
        buffer[n / 2 + i] = even[i] - r * odd[i];
        r *= root;
    }

    return buffer;
}

fn dft(input: &mut Vec<f32>) -> Vec<Complex<f32>> {
    let n = input.len();
    let i = Complex::new(0.0, 1.0);
    let mut ans: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n {
        for j in 0..n {
            let kf32 = k as f32;
            let jf32 = j as f32;
            let nf32 = n as f32;
            ans[k] += input[j] * (-2.0 * f32::consts::PI * kf32 * jf32 / nf32 * i).exp();
        }
    }
    return ans;
}

fn hann_window(v: Vec<i32>) -> Vec<f32> {
    let mut v2 = v.iter().map(|v| *v as f32).collect::<Vec<_>>();
    for i in 0..v.len() {
        let t = i as f32 / v.len() as f32 - 1.0;
        v2[i] *= (t * f32::consts::PI).sin().powf(2.0);
    }

    return v2;
}

fn main() -> std::io::Result<()> {
    let file_path = env::args().nth(1);

    if file_path == None {
        println!("Please give a path to a wav file");
        return Ok(());
    }

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let nrows = 256;
    let ncols = 512;

    let window_size = nrows as usize;
    let db_ceil: f32 = 8.0;
    let db_floor: f32 = 1.0;

    let mut reader = hound::WavReader::open(file_path.unwrap()).unwrap();

    let mut encoder = png::Encoder::new(w, ncols, nrows);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut image_data: Vec<i32> = vec![0; (nrows * ncols) as usize];

    let mut max_amp: f32 = 0.0;
    for column in 0..ncols {
        let window = reader
            .samples::<i32>()
            .take(window_size)
            .flatten()
            .collect::<Vec<_>>();

        let t = reader
            .samples::<i32>()
            .take(10)
            .flatten()
            .collect::<Vec<_>>();

        let dft_result = dft(&mut hann_window(window));

        let amp_result = dft_result
            .into_iter()
            .map(|v| (v.norm() / window_size as f32).log10())
            .collect::<Vec<_>>();

        for j in 0..window_size {
            let mut amp = amp_result[j].max(db_floor).min(db_ceil);
            amp -= db_floor;
            amp /= db_ceil - db_floor;

            let index = j + column as usize * window_size as usize;

            image_data[index] = (amp * 255.0) as i32;
            max_amp = max_amp.max(amp);
        }
    }

    // let mut max_amplitude = result.iter().max_by(|a, b| a.)
    let mut image: Vec<u8> = vec![];
    for i in 0..nrows {
        for j in 0..ncols {
            let index = (i + j * nrows) as usize;
            let red = image_data[index] as u8 / max_amp as u8;
            image.push(red);
            image.push(0); // green
            image.push(0); // blue
            image.push(255); // alpha
        }
    }

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&image[..]);
    //TODO: add green,blue, alpha and write image

    return Ok(());
}
