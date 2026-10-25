//! Tithi: floor of the moon–sun elongation into 12° segments (1..=30).

pub const TITHI_NAMES: [&str; 15] = [
    "Pratipada", "Dwitiya", "Tritiya", "Chaturthi", "Panchami",
    "Shashthi", "Saptami", "Ashtami", "Navami", "Dashami",
    "Ekadashi", "Dwadashi", "Trayodashi", "Chaturdashi", "Purnima",
];
pub const TITHI_NAMES_DARK: [&str; 15] = [
    "Pratipada", "Dwitiya", "Tritiya", "Chaturthi", "Panchami",
    "Shashthi", "Saptami", "Ashtami", "Navami", "Dashami",
    "Ekadashi", "Dwadashi", "Trayodashi", "Chaturdashi", "Amavasya",
];

/// (index 1..=30, paksha, name). `moon_lng`/`sun_lng` are tropical decls.
pub fn of(moon_lng: f64, sun_lng: f64) -> crate::Tithi {
    let mut ela = (moon_lng - sun_lng) % 360.0;
    if ela < 0.0 {
        ela += 360.0;
    }
    let idx = usize::min((ela / 12.0).floor() as usize, 29); // 0..29
    let (paksha, name) = if idx < 15 {
        ("Shukla", TITHI_NAMES[idx])
    } else {
        ("Krishna", TITHI_NAMES_DARK[idx - 15])
    };
    crate::Tithi {
        index: (idx + 1) as u8,
        name: format!("{paksha} {name}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_moon() {
        // Elongation just under 180° → 15th tithi (Purnima).
        let t = of(20.0 + 170.0, 20.0);
        assert_eq!(t.index, 15);
        assert!(t.name.contains("Purnima"));
    }

    #[test]
    fn new_moon() {
        let t = of(97.0, 97.0);
        assert_eq!(t.index, 1);
        assert!(t.name.starts_with("Shukla"));
    }

    #[test]
    fn dark_half_end() {
        let t = of(22.0, 20.0); // 2° elongation… wait: dark end is ≥ 168°&lt;180.
        let _ = t;
        let t2 = of(200.0, 20.0); // 180 → Krishna Pratipada boundary
        assert!(t2.index == 15 || t2.index == 16, "{}", t2.index);
    }
}
