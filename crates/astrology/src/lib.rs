//! astrology — pure computation engine for the clean-room reconstruction.
//!
//! All astronomical formulas are public-domain algorithms from Jean Meeus,
//! *Astronomical Algorithms* (2nd ed.): low-precision solar longitude
//! (Ch. 25) and a truncated lunar series (Ch. 47, top 10 periodic terms,
//! error ≈ ±0.05° — requirement: ≤ 0.6°, see NFR-04). UT ≈ TT is assumed
//! (ΔT ignored; negligible at sign/tithi resolution).
//!
//! No I/O, no third-party crates beyond `serde` types for shared payloads.

pub mod ayanamsa;
pub mod data;
pub mod horoscope;
pub mod julian;
pub mod moon;
pub mod nakshatra;
pub mod sun;
pub mod sunrise;
pub mod tithi;
pub mod zodiac;

/// Chart summary returned by the engine (see requirements F-CHART-02).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ChartSummary {
    /// Name as typed by the caller.
    pub name: String,
    /// Local input echoed back.
    pub local: String,
    /// UTC ISO-8601 instant after offset conversion.
    pub utc: String,
    /// Julian Day (UT) of the instant.
    pub julian_day: f64,
    /// Lahiri-style ayanamsa applied (deg).
    pub ayanamsa_deg: f64,
    /// Sun facts.
    pub sun: LuminBody,
    /// Moon facts.
    pub moon: LuminBody,
    /// 27-lunar-mansion placement of the sidereal moon.
    pub nakshatra: Nakshatra,
    /// Moon–Sun elongation tithi (1..=30).
    pub tithi: Tithi,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct LuminBody {
    /// Tropical ecliptic longitude (deg).
    pub longitude_deg: f64,
    /// Tropical sign (0=Aries … 11=Pisces).
    pub tropical_sign: u8,
    pub tropical_sign_name: String,
    /// Sidereal sign (Lahiri-style ayanamsa).
    pub sidereal_sign: u8,
    pub sidereal_sign_name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Nakshatra {
    /// 0..=26
    pub index: u8,
    pub name: String,
    /// 1..=4
    pub pada: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Tithi {
    /// 1..=30
    pub index: u8,
    pub name: String,
}

/// Compute a full chart summary for a local civil instant.
pub fn chart_summary(
    name: &str,
    local: &str,
    utc: &str,
    jd: f64,
) -> ChartSummary {
    let sun_lng = sun::apparent_longitude(jd);
    let moon_lng = moon::apparent_longitude(jd);
    let ay = ayanamsa::lahiri(jd);
    let sun = zodiac::bodies(
        sun_lng,
        ay,
        "Sun",
    );
    let moon = zodiac::bodies(
        moon_lng,
        ay,
        "Moon",
    );
    let nak = nakshatra::of_sidereal(
        zodiac::sidereal(moon_lng, ay),
    );
    let titi = tithi::of(moon_lng, sun_lng);
    ChartSummary {
        name: name.to_string(),
        local: local.to_string(),
        utc: utc.to_string(),
        julian_day: jd,
        ayanamsa_deg: ay,
        sun,
        moon,
        nakshatra: nak,
        tithi: titi,
    }
}
