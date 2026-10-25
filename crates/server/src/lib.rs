//! Library half of the server so integration tests can drive the router.

pub mod astrologers;
pub mod chart;
pub mod error;
pub mod horoscope;
pub mod pages;
pub mod panchang;
pub mod parse_util;

use axum::routing::get;
use axum::Router;

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { pages::index() }))
        .route("/healthz", get(health))
        .route("/horoscope/{period}", get(horoscope::horoscope))
        .route("/free-kundli", get(|| async { pages::kundli_form() }))
        .route("/chart", get(chart::get_chart).post(chart::post_chart))
        .route("/panchang", get(panchang::panchang_dir))
        .route("/panchang/{city}/{day}", get(panchang::panchang))
        .route("/api/astrologers", get(astrologers::list))
        .route("/astrologer/{code}", get(astrologers::detail))
        .fallback(not_found)
        .layer(axum::extract::DefaultBodyLimit::max(32 * 1024))
}

async fn health() -> impl axum::response::IntoResponse {
    axum::Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn not_found() -> impl axum::response::IntoResponse {
    error::ApiError::not_found("route_not_found", "no such route on this origin")
}
