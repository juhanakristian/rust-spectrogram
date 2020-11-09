use num::complex::Complex;
use std;
use std::f32;

fn fft(buffer: Vec<u16>) -> Vec<u16> {
    let N = buffer.len();
    if N == 1 {
        return buffer;
    }

    let r: f32 = -2.00 * f32::consts::PI / (N as f32);
    let root = Complex::new(r.cos(), r.sin());

    let even: &mut Vec<Complex<f32>> = &mut vec![];
    let odd: &mut Vec<Complex<f32>> = &mut vec![];

    return vec![0];
}

fn main() {
    println!("Hello, world!");
}
