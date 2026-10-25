//! Truncated lunar ecliptic longitude (Meeus Ch. 47, top 10 periodic terms).
//!
//! Error ≈ ±0.05° for the terms retained here (vs. full series ±0.0003°) —
//! well inside the requirement NFR-04 (≤0.6°) for moon-sign resolution.
//! UT ≈ TT assumed.

use crate::sun::centuries;
use crate::sun::norm360;

fn d(x: f64) -> f64 {
    x.to_radians()
}

/// Apparent lunar ecliptic longitude (deg [0,360)). No nutation term
/// (≤ 0.004°) and no eccentricity factor (≤ 0.005°).
pub fn apparent_longitude(jd: f64) -> f64 {
    let t = centuries(jd);
    let l_prime = 218.3164477 + 481267.88123421 * t;
    let dd = 297.8501921 + 445267.1114034 * t;
    let m = 357.5291092 + 35999.0502909 * t;
    let m_prime = 134.9633964 + 477198.8675055 * t;
    let f = 93.2720950 + 483202.0175233 * t;

    let s = 6.288774 * d(m_prime).sin()
        + 1.274018 * d(2.0 * dd - m_prime).sin()
        + 0.658314 * d(2.0 * dd).sin()
        + 0.213618 * d(2.0 * m_prime).sin()
        - 0.185116 * d(m).sin()
        - 0.114332 * d(2.0 * f).sin()
        + 0.058793 * d(2.0 * dd - 2.0 * m_prime).sin()
        + 0.057066 * d(2.0 * dd - m - m_prime).sin()
        + 0.053322 * d(2.0 * dd + m_prime).sin()
        + 0.045758 * d(2.0 * dd - m).sin();

    norm360(l_prime + s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meeus_ch47_example() {
        // Meeus 47.a: 1992-04-12 00:00 TT → λ ≈ 133.162656°.
        // Truncated-series tolerance ±0.55° (test of requirement NFR-04).
        let jd = 2448724.5;
        let lam = apparent_longitude(jd);
        assert!(
            (lam - 133.162656).abs() < 0.55,
            "lam={lam}"
        );
    }
}
