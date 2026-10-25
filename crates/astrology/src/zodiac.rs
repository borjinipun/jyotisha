//! Zodiac: tropical + sidereal sign taxonomy (12 × 30°).

use crate::sun::norm360;

/// Canonical order and names (0=Aries … 11=Pisces).
pub const SIGNS: [&str; 12] = [
    "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
    "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

/// Lowercase URL/handle form.
pub const SIGN_SLUGS: [&str; 12] = [
    "aries", "taurus", "gemini", "cancer", "leo", "virgo",
    "libra", "scorpio", "sagittarius", "capricorn", "aquarius", "pisces",
];

pub fn sign_index_of(longitude: f64) -> u8 {
    (norm360(longitude) / 30.0).floor() as u8
}

pub fn sign_name(idx: u8) -> &'static str {
    SIGNS[(idx % 12) as usize]
}

pub fn sidereal(longitude: f64, ayanamsa_deg: f64) -> f64 {
    norm360(longitude - ayanamsa_deg)
}

pub fn slug_lookup(slug: &str) -> Option<u8> {
    SIGN_SLUGS.iter().position(|s| *s == slug).map(|i| i as u8)
}

/// Rough tropical sun-sign boundaries for parameter-free pages.
pub fn tropical_sun_sign_of_ymd(y: i32, m: u32, d: u32) -> &'static str {
    let key = m * 100 + d;
    // (sign name chosen by the ingress date of the *middle* of the window)
    const CUTS: [(u32, u8); 12] = [
        (120, 9),  // Capricorn until ~Jan 20
        (219, 10), // Aquarius until ~Feb 19
        (321, 11), // Pisces
        (420, 0),  // Aries
        (521, 1),  // Taurus
        (621, 2),  // Gemini
        (723, 3),  // Cancer
        (823, 4),  // Leo
        (923, 5),  // Virgo
        (1023, 6), // Libra
        (1122, 7), // Scorpio
        (1222, 8), // Sagittarius
    ];
    let mut sign = 9u8; // before Jan 20 → Capricorn
    for (cut, s) in CUTS {
        if key >= cut {
            sign = s;
        }
    }
    let _ = y;
    SIGNS[sign as usize]
}

/// Public struct for chart members.
pub fn bodies(lng: f64, ay: f64, _label: &str) -> crate::LuminBody {
    let sid = sidereal(lng, ay);
    let t_i = sign_index_of(lng);
    let s_i = sign_index_of(sid);
    crate::LuminBody {
        longitude_deg: norm360(lng),
        tropical_sign: t_i,
        tropical_sign_name: sign_name(t_i).to_string(),
        sidereal_sign: s_i,
        sidereal_sign_name: sign_name(s_i).to_string(),
    }
}

/// Deterministic moon-phase label from the tithi index (1..=30).
pub fn moon_phase_label(tithi_index: u8) -> &'static str {
    match tithi_index.max(1).min(30) {
        1 => "New Moon",
        2..=7 => "Waxing Crescent",
        8..=9 => "First Quarter",
        10..=14 => "Waxing Gibbous",
        15 => "Full Moon",
        16..=22 => "Waning Gibbous",
        23..=24 => "Last Quarter",
        _ => "Waning Crescent",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugs() {
        assert_eq!(slug_lookup("aries"), Some(0));
        assert_eq!(slug_lookup("pisces"), Some(11));
        assert_eq!(slug_lookup("nope"), None);
    }

    #[test]
    fn tropical_index() {
        // 280° → Capricorn (9)
        assert_eq!(sign_index_of(280.0), 9);
        assert_eq!(sign_name(sign_index_of(280.0)), "Capricorn");
        assert_eq!(sign_name(sign_index_of(45.0)), "Taurus");
    }

    #[test]
    fn phases() {
        assert_eq!(moon_phase_label(1), "New Moon");
        assert_eq!(moon_phase_label(15), "Full Moon");
        assert_eq!(moon_phase_label(30), "Waning Crescent");
    }
}
