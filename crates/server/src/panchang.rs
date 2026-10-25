//! `/panchang` handlers (F-PANCH-01..04).

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use std::collections::HashMap;

use crate::error::ApiError;
use crate::parse_util;
use astrology::data::{city_by_slug, CITIES};
use astrology::julian::julian_day_from_local;
use astrology::moon::apparent_longitude as moon_lng;
use astrology::sun::apparent_longitude as sun_lng;
use astrology::sunrise;
use astrology::tithi;
use astrology::zodiac::{moon_phase_label, sidereal, sign_index_of, sign_name};
use astrology::ayanamsa::lahiri;

pub fn city_links() -> impl Iterator<Item = (String, String, String)> {
    CITIES
        .iter()
        .map(|c| (c.slug.to_string(), c.name.to_string(), c.state.to_string()))
}

/// `GET /panchang` (directory page)
pub async fn panchang_dir() -> Html<String> {
    crate::pages::panchang_index(city_links())
}

/// `GET /panchang/{city}/{today|tomorrow}?date=YYYY-MM-DD`
pub async fn panchang(
    Path((city_slug, day)): Path<(String, String)>,
    Query(q): Query<HashMap<String, String>>,
) -> Result<Response, ApiError> {
    if day != "today" && day != "tomorrow" {
        return Err(ApiError::not_found(
            "invalid_day",
            "use /panchang/{city}/today or /tomorrow",
        ));
    }
    let city = city_by_slug(&city_slug)
        .ok_or_else(|| ApiError::not_found("invalid_city", "unknown city slug"))?;

    // Date resolution: query date (deterministic), else "today".
    let date_str = q.get("date").cloned().unwrap_or_else(today_iso);
    let (y, m, d) = parse_util::parse_date(&date_str)
        .ok_or_else(|| ApiError::bad_request("invalid_date", "use YYYY-MM-DD"))?;
    let (y, m, d) = if day == "tomorrow" {
        parse_util::next_day(y, m, d)
            .ok_or_else(|| ApiError::bad_request("invalid_date", "date out of range"))?
    } else {
        (y, m, d)
    };

    // Sunrise/sunset (local).
    let (rise, set) = sunrise::times(y, m, d, city.lat, city.lon, city.tz_hours)
        .ok_or_else(|| ApiError::bad_request("location_edge", "polar day/night"))?;

    // Chart facts at local 12:00 for the day.
    let (jd, _) = julian_day_from_local(y, m, d, 12, 0, city.tz_hours)
        .map_err(|_| ApiError::bad_request("invalid_date", "unrepresentable date"))?;
    let ay = lahiri(jd);
    let sl = sidereal(moon_lng(jd), ay);
    let nak = astrology::nakshatra::of_sidereal(sl);
    let t = tithi::of(moon_lng(jd), sun_lng(jd));
    let moon_sign = sign_name(sign_index_of(sl));

    // Deterministic moon-phase label from the elongation (0..15).
    let phase = moon_phase_label(t.index);

    let (rh, rm) = sunrise::as_hm(rise);
    let (sh, sm) = sunrise::as_hm(set);

    let html_body = format!(
        "<h1>{}, {} — panchang</h1><p id=\"date\">{} ({})</p>\
         <table><tr><th>Sunrise</th><td id=\"sunrise\">{:02}:{:02}</td></tr>\
         <tr><th>Sunset</th><td id=\"sunset\">{:02}:{:02}</td></tr>\
         <tr><th>Tithi</th><td id=\"tithi\">{} ({})</td></tr>\
         <tr><th>Nakshatra</th><td id=\"nakshatra\">{} (pada {})</td></tr>\
         <tr><th>Moon sign (sidereal)</th><td id=\"moon-sign\">{}</td></tr>\
         <tr><th>Moon phase</th><td id=\"moon-phase\">{}</td></tr></table>\
         <p>Approximations: equation of time ignored; Meeus truncated lunar series; \
         Lahiri-style ayanamsa.</p>",
        city.name,
        city.state,
        format!("{:04}-{:02}-{:02}", y, m, d),
        day,
        rh,
        rm,
        sh,
        sm,
        t.name,
        t.index,
        nak.name,
        nak.pada,
        moon_sign,
        phase
    );
    let page_html = crate::pages::panchang_page(html_body);

    if q.get("format").map(|f| f.as_str()) == Some("json") {
        Ok(Json(serde_json::json!({
            "city": city.slug, "city_name": city.name, "state": city.state,
            "date": format!("{:04}-{:02}-{:02}", y, m, d),
            "day": day, "tzone": city.tz_hours,
            "sunrise": format!("{:02}:{:02}", rh, rm),
            "sunset": format!("{:02}:{:02}", sh, sm),
            "tithi": t.name, "tithi_index": t.index,
            "nakshatra": nak.name, "nakshatra_pada": nak.pada,
            "moon_sign": moon_sign, "moon_phase": phase,
            "page_html": page_html.0,
        }))
        .into_response())
    } else {
        Ok(page_html.into_response())
    }
}

/// A fixed "today" anchor for determinism so tests stay reproducible; callers
/// may always pass `?date=` to override. `JYOTISHA_TODAY` env var takes
/// precedence so the "current day" is injectable.
fn today_iso() -> String {
    match std::env::var("JYOTISHA_TODAY") {
        Ok(v) if parse_util::parse_date(&v).is_some() => v,
        _ => "2026-10-25".to_string(),
    }
}
