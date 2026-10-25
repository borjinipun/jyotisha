//! Date/time parsing helpers (F-VAL-001).

use astrology::julian::julian_day_from_local;
use astrology::julian::InvalidDate;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct CivilInstant {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub tzone: f64,
}

impl CivilInstant {
    /// UTC Julian Day for this local instant.
    pub fn utc_jd(&self) -> Result<f64, InvalidDate> {
        let (jd, _) = julian_day_from_local(
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.tzone,
        )?;
        Ok(jd)
    }
}

/// Strict `YYYY-MM-DD`.
pub fn parse_date(s: &str) -> Option<(i32, u32, u32)> {
    let p: Vec<&str> = s.split('-').collect();
    if p.len() != 3 {
        return None;
    }
    let y = i32::from_str(p[0]).ok()?;
    let m = u32::from_str(p[1]).ok()?;
    let d = u32::from_str(p[2]).ok()?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) || p[1].len() != 2 || p[2].len() != 2 {
        return None;
    }
    if !((1900..=2600).contains(&y)) {
        return None;
    }
    Some((y, m, d))
}

/// Strict `HH:MM`.
pub fn parse_time(s: &str) -> Option<(u32, u32)> {
    let p: Vec<&str> = s.split(':').collect();
    if p.len() != 2 {
        return None;
    }
    let h = u32::from_str(p[0]).ok()?;
    let mi = u32::from_str(p[1]).ok()?;
    if h > 23 || mi > 59 {
        return None;
    }
    Some((h, mi))
}

/// `+5.5`, `-5`, `5.5`, `0` … clamped by engine later.
pub fn parse_tzone(s: &str) -> Option<f64> {
    let v = f64::from_str(s.trim().trim_start_matches('+')).ok()?;
    if !(-12.0..=14.0).contains(&v) {
        return None;
    }
    Some(v)
}

/// Add one day to (y,m,d) via Julian Day round trip.
pub fn next_day(y: i32, m: u32, d: u32) -> Option<(i32, u32, u32)> {
    let jd = julian_day_from_local(y, m, d, 12, 0, 0.0).ok()?;
    let next = jd.0 + 1.0;
    civil_from_jd_noon(next)
}

/// (y,m,d) of the JD whose value is at noon (frac≈0.5).
pub fn civil_from_jd_noon(jd: f64) -> Option<(i32, u32, u32)> {
    // Meeus inverse (Ch. 7): Z = INT(JD + 0.5), F fractional remainder.
    let z = (jd + 0.5).floor();
    let f = jd + 0.5 - z;
    let alpha = ((z - 1867216.25) / 36524.25).floor();
    let a = z + 1.0 + alpha - (alpha / 4.0).floor();
    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let dd = (365.25 * c).floor();
    let e = ((b - dd) / 30.6001).floor();
    let day = (b - dd - (30.6001 * e).floor()).floor();
    let month = if e < 14.0 { e - 1.0 } else { e - 13.0 };
    let year = if month > 2.0 { c - 4716.0 } else { c - 4715.0 };
    if !(1.0..=12.0).contains(&month) || day < 1.0 || day > 31.0 || !(-9999.0..=9999.0).contains(&year) {
        return None;
    }
    let _ = f;
    Some((year as i32, month as u32, day as u32))
}
