use num::complex::Complex;
use std;
use std::f32;

fn fft(buffer: &mut Vec<Complex<f32>>) -> &mut Vec<Complex<f32>> {
    let N = buffer.len();
    if N == 1 {
        return buffer;
    }

    let r: f32 = -2.00 * f32::consts::PI / (N as f32);
    let root = Complex::new(r.cos(), r.sin());

    let p_even = &mut vec![];
    let p_odd = &mut vec![];

    for i in (0..N).step_by(2) {
        p_even.push(buffer[i]);
    }

    for i in (1..N).step_by(2) {
        p_odd.push(buffer[i]);
    }

    let even = fft(p_even);
    let odd = fft(p_odd);

    let mut r: Complex<f32> = Complex::new(1.00, 0.00);
    for i in 0..N / 2 {
        buffer[i] = even[i] + r * odd[i];
        buffer[N / 2 + i] = even[i] - r * odd[i];
        r *= root;
    }

    return buffer;
}

fn main() {
    println!("Hello, world!");
}
