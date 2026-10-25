//! Standard sunrise/sunset equation (zenith 90.833°: refraction + solar disk).
//!
//! Pure geometric approximation — equation of time is ignored (bounded by
//! ≈ ±2 min), intended for panchang-lite pages only. Per-city constants
//! live in [`crate::data::CITIES`].

use std::f64::consts::PI;

use crate::sun::norm360;

/// Approximate solar declination (deg) at JD, low precision.
fn declination_deg(jd: f64) -> f64 {
    let n = jd - 2451545.0;
    let lam = norm360(280.460 + 0.9856474 * n); // mean sun longitude
    let omega = 2.1429 - 0.0010394594 * n; // mean obliquity correction scale
    let eps = 23.439 - 0.0000004 * n;
    let l = lam.to_radians();
    let o = omega.to_radians();
    let lng = norm360(280.460 + 0.9856474 * n);
    // geometric solar longitude (mean + equation of centre ~ 1.915 sin g)
    let g = norm360(357.528 + 0.9856003 * n).to_radians();
    let tl = lng + 1.915 * g.sin() + 0.020 * (2.0 * g).sin();
    let _ = l;
    let _ = o;
    let tl_r = tl.to_radians();
    (eps.to_radians().sin() * tl_r.sin()).asin().to_degrees()
}

/// Sunrise/sunset local civil times (decimal hours) for a calendar day.
///
/// `lat`/`lon` in degrees (east-positive), `tz_hours` offset from UTC.
pub fn times(
    year: i32,
    month: u32,
    day: u32,
    lat: f64,
    lon: f64,
    tz_hours: f64,
) -> Option<(f64, f64)> {
    let jd_mid = crate::julian::julian_day(year, month, day, 0.0).ok()?;
    // Evaluate the sun at local solar noon: UTC noon minus (lon/15 − tz).
    let noon_jd = jd_mid + 0.5 - (tz_hours) / 24.0 + lon / 360.0;
    let decl = declination_deg(noon_jd);
    let lat_r = lat.to_radians();
    let const_r = (90.833f64).to_radians(); // zenith for rise/set
    let decl_r = decl.to_radians();
    let cos_h = (const_r.cos() - (lat_r.sin() * decl_r.sin()))
        / (lat_r.cos() * decl_r.cos());
    if !(-1.0..=1.0).contains(&cos_h) {
        return None; // polar day/night
    }
    let h = cos_h.acos(); // radians
    let solar_noon_utc = /* local solar noon in UTC hours inside the day */
        12.0 - lon / 15.0; // (approx; jd anchor above chose the same solar day)
    let rise_utc = solar_noon_utc - h.to_degrees() / 15.0;
    let set_utc = solar_noon_utc + h.to_degrees() / 15.0;
    let rise_local = rise_utc + tz_hours;
    let set_local = set_utc + tz_hours;
    let _ = PI;
    Some((norm_local(rise_local), norm_local(set_local)))
}

fn norm_local(h: f64) -> f64 {
    let v = h % 24.0;
    if v < 0.0 { v + 24.0 } else { v }
}

/// Decimal hours → (h, m) with minute rounding.
pub fn as_hm(hours: f64) -> (u32, u32) {
    let total = (hours * 60.0).round() as i64;
    let t = (total + 1440) % 1440;
    ((t / 60) as u32, (t % 60) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mumbai_solstice() {
        // Mumbai 2026-06-21: sunrise ≈ 06:03, sunset ≈ 19:18 IST
        let (r, s) = times(2026, 6, 21, 19.0760, 72.8777, 5.5).unwrap();
        let (rh, rm) = as_hm(r);
        let (sh, sm) = as_hm(s);
        let rise_min = rh * 60 + rm;
        let set_min = sh * 60 + sm;
        assert!(
            (rise_min >= 340 && rise_min <= 400),
            "rise {rh:02}:{rm:02}"
        );
        assert!(
            (set_min >= 1100 && set_min <= 1160),
            "set {sh:02}:{sm:02}"
        );
    }

    #[test]
    fn delhi_winter() {
        // Delhi 2026-12-21: sunrise ≈ 07:09, sunset ≈ 17:28 IST
        let (r, s) = times(2026, 12, 21, 28.6139, 77.2090, 5.5).unwrap();
        let (rh, rm) = as_hm(r);
        let (sh, sm) = as_hm(s);
        let rise_min = rh * 60 + rm;
        let set_min = sh * 60 + sm;
        assert!(rise_min >= 400 && rise_min <= 460, "rise {rh:02}:{rm:02}");
        assert!(set_min >= 1000 && set_min <= 1070, "set {sh:02}:{sm:02}");
    }
}
