use std::f64::consts::PI;

pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn cos(x: f64) -> f64 {
    x.cos()
}

pub fn tan(x: f64) -> f64 {
    x.tan()
}

pub fn exp(x: f64) -> f64 {
    x.exp()
}

pub fn ln(x: f64) -> f64 {
    x.ln()
}

pub fn log10(x: f64) -> f64 {
    x.log10()
}

pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

pub fn pow(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn rad_to_deg(radians: f64) -> f64 {
    radians * 180.0 / PI
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_trigonometric_functions() {
        assert_approx_eq!(sin(PI / 2.0), 1.0);
        assert_approx_eq!(cos(PI), -1.0);
        assert_approx_eq!(tan(PI / 4.0), 1.0);
    }

    #[test]
    fn test_exponential_and_logarithmic() {
        assert_approx_eq!(exp(1.0), std::f64::consts::E);
        assert_approx_eq!(ln(std::f64::consts::E), 1.0);
        assert_approx_eq!(log10(100.0), 2.0);
    }

    #[test]
    fn test_power_and_root() {
        assert_approx_eq!(sqrt(4.0), 2.0);
        assert_approx_eq!(pow(2.0, 3.0), 8.0);
    }

    #[test]
    fn test_angle_conversion() {
        assert_approx_eq!(deg_to_rad(180.0), PI);
        assert_approx_eq!(rad_to_deg(PI), 180.0);
    }
}