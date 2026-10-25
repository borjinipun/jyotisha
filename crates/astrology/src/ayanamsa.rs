//! Lahiri-style ayanamsa (linear precession approximation, public domain).
//!
//! Anchored: 23°51'11" (23°51.183′ … commonly quoted 23.8529°) at
//! J2000.0, rate 50.29″/Julian year. This is a textbook approximation
//! (published formula; error vs. tabulated Lahiri values is ≲ 0.03°
//! over 1900–2050).

/// Ayanamsa in degrees for a given Julian Day (UT ≈ TT).
pub fn lahiri(jd: f64) -> f64 {
    const J2000: f64 = 2451544.5; // 2000-01-01 00:00 UT
    const RATE_ARCSEC_PER_YEAR: f64 = 50.29;
    let years = (jd - J2000) / 365.2425;
    let ay = 23.853056 + RATE_ARCSEC_PER_YEAR / 3600.0 * years;
    ay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchored() {
        assert!((lahiri(2451544.5) - 23.853056).abs() < 1e-6);
    }

    #[test]
    fn plausible_2026() {
        // 2026-10-25: Lahiri ayanamsa ≈ 24.23-24.25°.
        let jd = 2461339.0; // ≈ 2026-10-25
        let ay = lahiri(jd);
        assert!((23.9..24.6).contains(&ay), "ay={ay}");
    }
}
