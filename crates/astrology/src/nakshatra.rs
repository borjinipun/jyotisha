//! 27 lunar mansions (sidereal moon longitude ÷ 13°20′) with pada.

pub const NAKSHATRAS: [&str; 27] = [
    "Ashwini", "Bharani", "Krittika", "Rohini", "Mrigashira", "Ardra",
    "Punarvasu", "Pushya", "Ashlesha", "Magha", "Purva Phalguni",
    "Uttara Phalguni", "Hasta", "Chitra", "Swati", "Vishakha", "Anuradha",
    "Jyeshtha", "Mula", "Purva Ashadha", "Uttara Ashadha", "Shravana",
    "Dhanishta", "Shatabhisha", "Purva Bhadrapada", "Uttara Bhadrapada",
    "Revati",
];

const SEGMENT_DEG: f64 = 360.0 / 27.0;

/// Sidereal-moon mansion: (index 0..=26, pada 1..=4).
pub fn of_sidereal(sidereal_moon_longitude: f64) -> crate::Nakshatra {
    // Normalize into [0, 360) then divide.
    let mut v = sidereal_moon_longitude % 360.0;
    if v < 0.0 {
        v += 360.0;
    }
    let seg = v / SEGMENT_DEG;
    let idx = usize::min((seg.floor()) as usize, 26);
    let pada = f64::min(((seg * 4.0) % 4.0).floor() + 1.0, 4.0) as u8;
    crate::Nakshatra {
        index: idx as u8,
        name: NAKSHATRAS[idx].to_string(),
        pada,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundaries() {
        assert_eq!(of_sidereal(0.0).index, 0);
        assert_eq!(of_sidereal(0.0).name, "Ashwini");
        assert_eq!(of_sidereal(13.0).name, "Ashwini");
        assert_eq!(of_sidereal(13.4).name, "Bharani");
        assert_eq!(of_sidereal(355.0).name, "Revati");
        assert_eq!(of_sidereal(359.999).name, "Revati");
    }

    #[test]
    fn pada() {
        assert_eq!(of_sidereal(0.3).pada, 1);
        assert_eq!(of_sidereal(3.4).pada, 2);
        assert_eq!(of_sidereal(6.7).pada, 3);
        assert_eq!(of_sidereal(10.0).pada, 4);
        assert_eq!(of_sidereal(14.0).pada, 1);
    }
}
