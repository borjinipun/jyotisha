//! Calendar ⇄ Julian Day conversions (proleptic Gregorian).
//!
//! JD convention: JD = 2451545.0 exactly at 2000-01-01 12:00 UT (J2000.0).

/// Error for invalid civil dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidDate;

/// Julian Day from proleptic-Gregorian civil date and fractional UTC day.
/// `frac` ∈ [0,1): 0 = midnight. Example: 12:00 UT → frac = 0.5.
pub fn julian_day(year: i32, month: u32, day: u32, frac: f64) -> Result<f64, InvalidDate> {
    if month == 0 || month > 12 || day == 0 || day > 31 {
        return Err(InvalidDate);
    }
    let mut y = year as i64;
    let mut m = month as i64;
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    // Gregorian leap correction (Meeus Ch. 7): A = INT(Y/100), B = 2 − A + INT(A/4).
    let a = y.div_euclid(100);
    let b = 2 - a + a.div_euclid(4);
    let jd0 = (365.25 * (y + 4716).to_f64().unwrap_or(0.0)).floor()
        + (30.6001 * (m + 1).to_f64().unwrap_or(0.0)).floor()
        + day as f64
        + b as f64
        - 1524.5;
    if !(0.0..1.0).contains(&frac) {
        return Err(InvalidDate);
    }
    Ok(jd0 + frac)
}

// i64 → f64 helper without importing num traits.
#[allow(non_snake_case)]
trait ToF64 {
    fn to_f64(self) -> Option<f64>;
}
impl ToF64 for i64 {
    fn to_f64(self) -> Option<f64> {
        Some(self as f64)
    }
}

/// Split a JD into integer JD at noon + fractional day, for display.
pub fn jd_parts(jd: f64) -> (f64, f64) {
    let int = jd.floor();
    let frac = jd - int;
    (int, frac)
}

/// Convert `(y, m, d, h, min)` + hour-offset timezone to UTC JD.
/// `tzone_hours` is signed (e.g. 5.5 for IST, -5.0 for US Eastern).
pub fn julian_day_from_local(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    tzone_hours: f64,
) -> Result<(f64, f64), InvalidDate> {
    if !(hour <= 23 && minute <= 59) {
        return Err(InvalidDate);
    }
    if !(-12.0..=14.0).contains(&tzone_hours) {
        return Err(InvalidDate);
    }
    let local_frac = (hour as f64 + minute as f64 / 60.0 + 1e-9) / 24.0;
    let jd_local = julian_day(year, month, day, local_frac)?;
    let utc = jd_local - tzone_hours / 24.0;
    Ok((utc, jd_local))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn j2000_noon() {
        let jd = julian_day(2000, 1, 1, 0.5).unwrap();
        assert!((jd - 2451545.0).abs() < 1e-4, "jd={jd}");
    }

    #[test]
    fn meeus_ch7() {
        // Meeus Ch.7: 1957-10-04 19:26:24 → 2436116.31 (converted to UT)
        let jd = julian_day(1957, 10, 4, 19.0 / 24.0 + 26.0 / 1440.0 + 24.0 / 86400.0).unwrap();
        assert!((jd - 2436116.31).abs() < 0.001, "jd={jd}");
    }

    #[test]
    fn roundtrip_local() {
        // 2026-10-25 09:00 local (UTC+5.5): JD(2026-10-25 00:00 UT) is 2461338.5.
        let (utc, jd_local) = julian_day_from_local(2026, 10, 25, 9, 0, 5.5).unwrap();
        let expected_local = 2461338.5 + 9.0 / 24.0;
        assert!((jd_local - expected_local).abs() < 1e-3, "jd_local={jd_local}");
        assert!((utc - (jd_local - 5.5 / 24.0)).abs() < 1e-6);
        // UTC shift → 03:30 UT (9.0 − 5.5 h).
        let expected_utc = expected_local - 5.5 / 24.0;
        assert!((utc - expected_utc).abs() < 1e-3);
    }

    #[test]
    fn rejects_bad() {
        assert!(julian_day_from_local(2026, 13, 25, 9, 0, 5.5).is_err());
        assert!(julian_day_from_local(2026, 10, 25, 24, 0, 5.5).is_err());
        assert!(julian_day_from_local(2026, 10, 25, 9, 0, 20.0).is_err());
    }
}
