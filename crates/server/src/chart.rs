//! `/chart` handlers (F-CHART-01..05).

use axum::extract::Query;
use axum::Json;
use serde_json::Value;
use std::collections::HashMap;

use crate::error::ApiError;
use crate::parse_util::{parse_date, parse_time, parse_tzone, CivilInstant};

/// Common query/JSON parsing → `CivilInstant`.
pub(crate) fn instant_from_parts(
    name: Option<String>,
    date: Option<&str>,
    time: Option<&str>,
    tzone: Option<f64>,
) -> Result<(String, CivilInstant), ApiError> {
    let date = date.ok_or_else(|| ApiError::bad_request("invalid_date", "date (YYYY-MM-DD) is required"))?;
    let time = time.ok_or_else(|| ApiError::bad_request("invalid_time", "time (HH:MM) is required"))?;
    let tz = tzone.ok_or_else(|| ApiError::bad_request("invalid_tzone", "tzone offset (hours from UTC, e.g. 5.5) is required"))?;
    let (y, m, d) = parse_date(date).ok_or_else(|| ApiError::bad_request("invalid_date", "use YYYY-MM-DD in 1900..=2600"))?;
    let (h, mi) = parse_time(time).ok_or_else(|| ApiError::bad_request("invalid_time", "use strict 24h HH:MM"))?;
    let ci = CivilInstant { year: y, month: m, day: d, hour: h, minute: mi, tzone: tz };
    ci.utc_jd().map_err(|_| ApiError::bad_request("invalid_date", "the instant is not representable"))?;
    let name = match name {
        Some(n) if !n.trim().is_empty() => n.trim().to_string(),
        _ => "unnamed".to_string(),
    };
    Ok((name, ci))
}

/// `GET /chart?…` — query form.
pub async fn get_chart(Query(q): Query<HashMap<String, String>>) -> Result<Json<Value>, ApiError> {
    let (name, ci) = instant_from_parts(
        q.get("name")
            .map(|s| s.to_string()),
        q.get("date").map(|s| s.as_str()),
        q.get("time").map(|s| s.as_str()),
        q.get("tzone").and_then(|s| parse_tzone(s)),
    )?;
    Ok(Json(chart_json(name, ci)))
}

/// `POST /chart` — JSON body form (same contract as GET).
pub async fn post_chart(
    Json(body): Json<serde_json::Value>,
) -> Result<Json<Value>, ApiError> {
    let name = body.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let date = body.get("date").and_then(|v| v.as_str());
    let time = body.get("time").and_then(|v| v.as_str());
    let tzone = body.get("tzone").and_then(|v| v.as_f64());
    let (name, ci) = instant_from_parts(
        name,
        date,
        time,
        tzone,
    )?;
    Ok(Json(chart_json(name, ci)))
}

pub(crate) fn chart_json(name: String, ci: CivilInstant) -> Value {
    let jd = ci.utc_jd().expect("validated earlier");
    let summary = astrology::chart_summary(
        &name,
        &format!("{:04}-{:02}-{:02} {:02}:{:02} tzone {:+.1}", ci.year, ci.month, ci.day, ci.hour, ci.minute, ci.tzone),
        &utc_iso(jd),
        jd,
    );
    serde_json::to_value(summary).expect("serializable")
}

fn utc_iso(jd: f64) -> String {
    // JD → {YYYY-MM-DDTHH:MM:SSZ} (proleptic Gregorian)
    let day_int = (jd - 0.5).floor();
    let frac = jd - 0.5 - day_int;
    // day_int + 0.5 = noon of civil date
    let civil = crate::parse_util::civil_from_jd_noon(day_int + 0.5)
        .expect("jd within range");
    let secs = (frac * 86400.0).round() as i64;
    let hh = (secs / 3600) as u32;
    let mm = ((secs % 3600) / 60) as u32;
    let ss = (secs % 60) as u32;
    let _ = mm;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        civil.0, civil.1, civil.2, hh, mm, ss
    )
}
