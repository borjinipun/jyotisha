//! `/horoscope/:period` handlers (F-HOR-01..04).
//!
//! HTML by default; structured JSON when `?format=json`.

use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::collections::HashMap;

use crate::error::ApiError;
use crate::pages;
use astrology::horoscope::{text, Period};
use astrology::zodiac::{SIGN_SLUGS, SIGNS};

const PERIODS: [(&str, Period); 5] = [
    ("today", Period::Daily),
    ("tomorrow", Period::Daily),
    ("weekly", Period::Weekly),
    ("monthly", Period::Monthly),
    ("yearly", Period::Yearly),
];

fn period_of(slug: &str) -> Option<Period> {
    PERIODS.iter().find(|p| p.0 == slug).map(|p| p.1)
}

/// Anchor string from the query date (F-HOR-02 determinism).
fn anchor(period_slug: &str, date_str: Option<&str>) -> Result<String, ApiError> {
    let fallback = "2026-01-01";
    let d = date_str.unwrap_or(fallback);
    crate::parse_util::parse_date(d)
        .ok_or_else(|| ApiError::bad_request("invalid_date", "use YYYY-MM-DD"))?;
    let day = d.to_string();
    let anchor = match period_slug {
        "tomorrow" => {
            let (y, m, dd) = crate::parse_util::parse_date(d).unwrap();
            let nxt = crate::parse_util::next_day(y, m, dd)
                .ok_or_else(|| ApiError::bad_request("invalid_date", "date out of range"))?;
            format!("{}-{:02}-{:02}|tomorrow", nxt.0, nxt.1, nxt.2)
        }
        "weekly" => format!("{}|week", day),
        "monthly" => format!("{}|month", &day[..7]),
        "yearly" => format!("{}|year", &day[..4]),
        _ => format!("{}|today", day),
    };
    Ok(anchor)
}

/// `GET /horoscope/:period?sign=…&date=…&format=json`
pub async fn horoscope(
    Path(period): Path<String>,
    Query(q): Query<HashMap<String, String>>,
) -> Result<Response, ApiError> {
    let p = period_of(&period).ok_or_else(|| {
        ApiError::not_found("invalid_period", "use today|tomorrow|weekly|monthly|yearly")
    })?;
    let sign_slug = q
        .get("sign")
        .map(|s| s.trim().to_ascii_lowercase())
        .unwrap_or_default();
    let sign_idx = astrology::zodiac::slug_lookup(&sign_slug).ok_or_else(|| {
        ApiError::bad_request("invalid_sign", "use a lowercase zodiac sign like 'aries'")
    })?;
    let label = period.as_str();
    let anchor = anchor(label, q.get("date").map(|s| s.as_str()))?;
    let msg = text(SIGNS[sign_idx as usize], p, &anchor);
    let date_label = anchor.split('|').next().unwrap_or("").to_string();

    let body = serde_json::json!({
        "sign": SIGN_SLUGS[sign_idx as usize],
        "sign_name": SIGNS[sign_idx as usize],
        "period": label,
        "anchor_date": date_label,
        "horoscope": msg,
    });

    if q.get("format").map(|f| f.as_str()) == Some("json") {
        Ok(Json(body).into_response())
    } else {
        Ok(pages::horoscope_page(
            SIGNS[sign_idx as usize],
            label,
            &date_label,
            &msg,
            SIGN_SLUGS.iter().map(|s| (*s).to_string()),
            &sign_slug,
        )
        .into_response())
    }
}
