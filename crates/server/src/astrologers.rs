//! `/api/astrologers` and `/astrologer/{code}` — original mock rows shaped
//! like the observed contract (O-4), content 100% original (F-BOT-01..03).

use axum::extract::{Path, Query};
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;

use crate::error::ApiError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    pub id: String,
    pub entity_name: String,
    pub entity_code: String,
    pub entity_description: String,
    pub entity_domain: String,
    pub entity_categories: Vec<String>,
    pub entity_rating: f64,
    pub entity_orders: String,
    pub entity_languages: Vec<String>,
    pub entity_path: String,
    pub priority: i32,
    pub is_discoverable: bool,
}

const CATEGORIES: [&str; 7] = ["All", "Love", "Marriage", "Career", "Life Coach", "Wealth", "General"];

fn row(
    i: usize,
    name: &str,
    desc: &str,
    domain: &str,
    cats: &[&str],
    rating: f64,
    orders: &str,
    priority: i32,
) -> Bot {
    let code = name.to_ascii_lowercase().replace([' ', '.'], "");
    Bot {
        id: format!("mock-entity-{:02}", i + 1),
        entity_name: name.to_string(),
        entity_code: code.clone(),
        entity_description: desc.to_string(),
        entity_domain: domain.to_string(),
        entity_categories: cats.iter().map(|s| s.to_string()).collect(),
        entity_rating: rating,
        entity_orders: orders.to_string(),
        entity_languages: vec!["English".into(), "Hindi".into()],
        entity_path: format!("/astrologer/{}", code),
        priority,
        is_discoverable: true,
    }
}

/// Seeded original sample data (names and blurbs written for this project).
pub fn bots() -> Vec<Bot> {
    vec![
        row(0, "Arya Mishra", "Love and relationships reader with a warm storytelling style", "love", &["Love"], 4.7, "9,800+", 1),
        row(1, "Keva Nair", "Marriage timing and compatibility specialist", "marriage", &["Marriage"], 4.6, "6,300+", 2),
        row(2, "Mohan Pratap", "Career decisions, job switches and growth windows", "career", &["Career"], 4.8, "11,200+", 3),
        row(3, "Insha Qureshi", "Life-coach approach with practical weekly guidance", "life-coach", &["Life Coach"], 4.5, "4,100+", 4),
        row(4, "Tarun Dev", "Wealth, savings rhythm and Jupiter cycles", "wealth", &["Wealth"], 4.4, "3,700+", 5),
        row(5, "Zara .", "General questions, tarot-flavoured but chart-grounded", "general", &["General"], 4.3, "2,900+", 6),
        row(6, "Devpriya S.", "Love predictions blended with numerology", "love", &["Love", "General"], 4.6, "5,500+", 7),
        row(7, "NakulRathod", "Second-opinion readings for big life moves", "general", &["Career", "General"], 4.9, "15,000+", 8),
    ]
}

/// `GET /api/astrologers?skip=&limit=&category=`
pub async fn list(Query(q): Query<HashMap<String, String>>) -> Result<Json<serde_json::Value>, ApiError> {
    let skip = q.get("skip").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let limit = q.get("limit").and_then(|s| s.parse::<usize>().ok()).unwrap_or(10).clamp(1, 50);
    let category = q.get("category").cloned().unwrap_or_else(|| "All".to_string());
    let cat_ok = category == "All" || CATEGORIES.contains(&category.as_str());
    if !cat_ok {
        // Lenient (UNK-2): unknown category → empty, not error.
    }
    let mut all = bots();
    all.sort_by_key(|b| b.priority);
    let mut filtered: Vec<Bot> = all
        .into_iter()
        .filter(|b| category == "All" || b.entity_categories.iter().any(|c| *c == category))
        .collect();
    let total = filtered.len();
    let _ = cat_ok;
    let page: Vec<Bot> = filtered.drain(skip.min(total)..).take(limit).collect();
    Ok(Json(serde_json::json!({
        "bots": page,
        "count": total,
        "skip": skip,
        "limit": limit,
        "category": category,
    })))
}

/// `GET /astrologer/{code}`
pub async fn detail(Path(code): Path<String>) -> Result<Json<serde_json::Value>, ApiError> {
    let code = code.to_ascii_lowercase();
    let bot = bots().into_iter().find(|b| b.entity_code.to_ascii_lowercase() == code);
    bot.map(|b| Json(serde_json::to_value(b).unwrap()))
        .ok_or_else(|| ApiError::not_found("unknown_astrologer", "no mock row with that code"))
}
