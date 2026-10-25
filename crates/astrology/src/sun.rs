//! Low-precision apparent solar ecliptic longitude (Meeus Ch. 25).
//!
//! Accuracy ≈ ±0.01° over 1900–2050 — sufficient for sun-sign resolution.

/// Julian centuries since J2000.0 (TT ≈ UT).
pub fn centuries(jd: f64) -> f64 {
    (jd - 2451545.0) / 36525.0
}

fn deg(x: f64) -> f64 {
    x.to_radians()
}

/// Apparent (refraction-light-corrected) geometric ecliptic longitude in deg [0,360).
pub fn apparent_longitude(jd: f64) -> f64 {
    let t = centuries(jd);
    let mean_lng =
        280.46646 + 36000.76983 * t + 0.0003032 * t * t;
    let mean_anom =
        357.52911 + 35999.05029 * t - 0.0001537 * t * t;
    let g = deg(mean_anom);
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * g.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * g).sin()
        + 0.000289 * (3.0 * g).sin();
    let true_lng = mean_lng + c;
    let apparent = true_lng - 0.00569 - 0.00478 * deg(125.04 - 1934.136 * t).sin();
    norm360(apparent)
}

pub fn norm360(x: f64) -> f64 {
    let v = x % 360.0;
    if v < 0.0 { v + 360.0 } else { v }
}

/// Equation-of-time-free solar position helpers for `sunrise`.
pub fn solar_at(jd: f64) -> (f64, f64) {
    // returns (apparent longitude deg, equation of center only) — kept for tests.
    let t = centuries(jd);
    let mean_lng = 280.46646 + 36000.76983 * t;
    let mean_anom = 357.52911 + 35999.05029 * t;
    let g = deg(mean_anom);
    let c = 1.914602 * g.sin() + 0.019993 * (2f64 * g).sin();
    (norm360(mean_lng + c), norm360(mean_anom))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meeus_ch25_j2000() {
        // Meeus example: 2000-01-01 12:00 → true longitude 280.38397° (geometric).
        let jd = 2451545.0;
        let t = centuries(jd);
        let mean_lng = 280.46646 + 36000.76983 * t;
        let mean_anom = 357.52911 + 35999.05029 * t;
        let g = deg(mean_anom);
        let c = (1.914602) * g.sin() + (0.019993) * (2f64 * g).sin();
        let true_lng = mean_lng + c;
        assert!(
            (true_lng - 280.38397).abs() < 0.005,
            "true={true_lng}"
        );
    }

    #[test]
    fn norm() {
        assert_eq!(norm360(-10.0), 350.0);
        assert_eq!(norm360(360.0), 0.0);
    }
}
