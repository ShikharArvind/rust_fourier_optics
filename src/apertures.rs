use ndrustfft::*;

// Implement simple fftshift for 1d using slices and concatenation
pub fn rect_1d(x: f64) -> Complex<f64> {
    if x.abs() <= 0.5 {
        Complex::new(1.0, 0.0)
    } else {
        Complex::new(0.0, 0.0)
    }
}