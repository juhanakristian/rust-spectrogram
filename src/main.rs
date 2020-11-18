use num::complex::Complex;
use std::{self, fs::File, path::Path};

use hound;
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

fn main() -> std::io::Result<()> {
    let file_path = env::args().nth(1);

    if file_path == None {
        println!("Please give a path to a wav file");
        return Ok(());
    }

    let window_size = 256;

    let mut reader = hound::WavReader::open(file_path.unwrap()).unwrap();
    let sample_count = reader.len();
    for n in (0..sample_count).step_by(window_size) {
        let mut window = reader
            .samples::<f32>()
            .take(window_size)
            .flatten()
            .collect::<Vec<_>>();

        let result = dft(&mut window);
    }

    return Ok(());
}
