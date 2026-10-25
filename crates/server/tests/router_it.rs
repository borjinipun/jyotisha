//! Router integration tests (F-PAGE, F-HOR, F-CHART, F-PANCH, F-BOT, F-ERR).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn resolve(uri: &'static str) -> (axum::http::StatusCode, String) {
    let app = jyotisha_server::app();
    let resp = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8_lossy(&bytes).to_string())
}

#[tokio::test]
async fn f_page_01_index_links() {
    let (s, b) = resolve("/").await;
    assert_eq!(s, StatusCode::OK);
    for frag in ["/horoscope/today", "/free-kundli", "/panchang", "/api/astrologers"] {
        assert!(b.contains(frag), "missing {frag} in index");
    }
}

#[tokio::test]
async fn f_hor_01_today_html() {
    let (s, b) = resolve("/horoscope/today?sign=aries").await;
    assert_eq!(s, StatusCode::OK);
    assert!(b.contains("<html"), "should be html; got: {b}");
}

#[tokio::test]
async fn f_hor_01_json() {
    let (s, b) = resolve("/horoscope/today?sign=aries&format=json").await;
    assert_eq!(s, StatusCode::OK);
    assert!(b.starts_with('{'), "expected json: {b}");
}

#[tokio::test]
async fn f_hor_02_deterministic() {
    let (_, b1) = resolve("/horoscope/weekly?sign=leo&date=2026-10-19").await;
    let (_, b2) = resolve("/horoscope/weekly?sign=leo&date=2026-10-19").await;
    assert_eq!(b1, b2);
}

#[tokio::test]
async fn f_hor_03_unknown_sign() {
    let (s, b) = resolve("/horoscope/today?sign=omen").await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    assert!(b.contains("invalid_sign"), "{b}");
}

#[tokio::test]
async fn f_val_002_money_404_period() {
    let (s, b) = resolve("/horoscope/sometime?sign=aries").await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    assert!(b.contains("invalid_period"), "{b}");
}

#[tokio::test]
async fn f_chart_02_get_chart() {
    let (s, b) = resolve("/chart?name=Asha&date=2005-01-10&time=13%3A30&tzone=5.5").await;
    assert_eq!(s, StatusCode::OK, "{b}");
    let v: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert!(v["julian_day"].is_number(), "{b}");
    assert!(v["sun"]["tropical_sign_name"].is_string(), "{b}");
    assert!(v["moon"]["sidereal_sign_name"].is_string(), "{b}");
    assert!(v["nakshatra"]["name"].is_string(), "{b}");
    assert!(v["tithi"]["name"].is_string(), "{b}");
    // Sanity: Jan 10 2005 tropical sun → Capricorn; sidereal → Sagittarius
    // (Makar Sankranti ~Jan 14 — deliberately before the boundary).
    assert_eq!(v["sun"]["tropical_sign_name"], "Capricorn");
    assert_eq!(v["sun"]["sidereal_sign_name"], "Sagittarius");
}

#[tokio::test]
async fn f_chart_05_invalid() {
    for uri in [
        "/chart?date=2005-13-01&time=10%3A00&tzone=0",
        "/chart?date=x&time=10%3A00&tzone=0",
        "/chart?date=2005-01-01&time=25%3A00&tzone=0",
        "/chart?date=2005-01-01&time=10%3A00&tzone=99",
    ] {
        let (s, b) = resolve(uri).await;
        assert_eq!(s, StatusCode::BAD_REQUEST, "{uri}: {b}");
        assert!(b.starts_with("{\"error\""), "{uri}: {b}");
    }
}

#[tokio::test]
async fn f_chart_missing_params() {
    let (s, b) = resolve("/chart?date=2005-01-01").await;
    assert_eq!(s, StatusCode::BAD_REQUEST);
    assert!(b.contains("invalid_time"), "{b}");
}

#[tokio::test]
async fn f_panch_01_city_page() {
    let (s, b) = resolve("/panchang/mumbai/today?date=2026-06-21").await;
    assert_eq!(s, StatusCode::OK);
    assert!(b.contains("sunrise"), "{b}");
    assert!(b.contains("Moon phase"), "{b}");
}

#[tokio::test]
async fn f_panch_03_unknown_city() {
    let (s, b) = resolve("/panchang/atlantis/today").await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    assert!(b.contains("invalid_city"), "{b}");
}

#[tokio::test]
async fn f_panch_04_dir() {
    let (s, b) = resolve("/panchang").await;
    assert_eq!(s, StatusCode::OK);
    assert!(b.contains("/panchang/mumbai/today"));
    assert!(b.contains("/panchang/hyderabad/today"));
}

#[tokio::test]
async fn f_bot_01_listing_shape() {
    let (s, b) = resolve("/api/astrologers?skip=0&limit=10&category=All").await;
    assert_eq!(s, StatusCode::OK);
    let v: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert!(v["bots"].is_array());
    assert_eq!(v["bots"].as_array().unwrap().len(), 8);
    assert_eq!(v["count"].as_u64().unwrap(), 8);
    let first = &v["bots"][0];
    for k in [
        "id", "entityName", "entityCode", "entityDescription", "entityDomain",
        "entityCategories", "entityRating", "entityOrders", "entityLanguages",
        "entityPath", "priority", "isDiscoverable",
    ] {
        assert!(first.get(k).is_some(), "missing contract field {k}");
    }
}

#[tokio::test]
async fn f_bot_02_category_filter() {
    let (_, b) = resolve("/api/astrologers?skip=0&limit=10&category=Love").await;
    let v: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert_eq!(v["count"].as_u64(), Some(2));
    let (_, b) = resolve("/api/astrologers?category=Vegetables").await;
    let v: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert_eq!(v["bots"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn f_bot_03_pagination_and_detail() {
    let (_, b) = resolve("/api/astrologers?skip=6&limit=2").await;
    let v: serde_json::Value = serde_json::from_str(&b).unwrap();
    assert_eq!(v["bots"].as_array().unwrap().len(), 2);
    let code = v["bots"][0]["entityCode"].as_str().unwrap().to_string();
    let (s, b) = resolve(Box::leak(format!("/astrologer/{}", code).into_boxed_str())).await;
    assert_eq!(s, StatusCode::OK);
    let (s2, b2) = resolve("/astrologer/does-not-exist").await;
    assert_eq!(s2, StatusCode::NOT_FOUND);
    assert!(b2.contains("unknown_astrologer"));
    let _ = b;
}

#[tokio::test]
async fn f_err_001_fallback_404() {
    let (s, b) = resolve("/totally/unknown").await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    assert!(b.starts_with("{\"error\""), "{b}");
}
