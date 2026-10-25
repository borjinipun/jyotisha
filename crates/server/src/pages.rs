//! HTML pages (minimal, original styling — no external assets).

use axum::response::Html;

const STYLE: &str = r#"
  <style>
    body{font-family:system-ui,sans-serif;margin:2rem auto;max-width:56rem;padding:0 1rem;color:#1c1b22}
    h1{font-weight:650} a{color:#7a4de8} .card{border:1px solid #e3e1ea;border-radius:12px;padding:1rem;margin:.6rem 0}
    table{border-collapse:collapse} td,th{border:1px solid #d4d2de;padding:.35rem .6rem}
    input{padding:.4rem;border:1px solid #cbc9d6;border-radius:6px}
    nav{display:flex;gap:1rem;flex-wrap:wrap;font-size:.95rem}
  </style>"#;

fn page(title: &str, body: &str) -> Html<String> {
    Html(format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
         <title>{title}</title>{STYLE}</head><body>{body}</body></html>"
    ))
}

fn nav() -> String {
    format!(
        "<nav><a href=\"/\">Home</a> <a href=\"/horoscope/today?sign=aries\">Horoscopes</a>\
         <a href=\"/free-kundli\">Free Kundli</a> <a href=\"/panchang\">Panchang</a>\
         <a href=\"/api/astrologers\">Astrologers</a> <a href=\"/healthz\">Health</a></nav><hr>"
    )
}

/// `GET /`
pub fn index() -> Html<String> {
    page(
        "Hi Astro — horoscopes, kundli, panchang",
        &format!(
            "{}<h1>Hi Astro</h1><p>An offline, deterministic astrology toolkit.\
            Clean-room reconstruction of an observed feature surface; all text is original.</p>\
            <div class=\"card\"><h2>Daily horoscope</h2><a href=\"/horoscope/today?sign=aries\">Today</a> · \
            <a href=\"/horoscope/tomorrow?sign=taurus\">Tomorrow</a> \
            <a href=\"/horoscope/weekly?sign=gemini\">Weekly</a> · \
            <a href=\"/horoscope/monthly?sign=cancer\">Monthly</a> · \
            <a href=\"/horoscope/yearly?sign=leo\">Yearly</a></div>\
            <div class=\"card\"><h2>Free Kundli</h2><a href=\"/free-kundli\">Open the chart form</a>\
            or <code>/chart?name=Asha&amp;date=2005-01-14&amp;time=13;30&amp;tzone=5.5</code> \
            (use %3A for ':')</div>\
            <div class=\"card\"><h2>Panchang</h2><a href=\"/panchang\">10 city pages</a></div>\
            <div class=\"card\"><h2>AI astrologer listing</h2>\
            <a href=\"/api/astrologers?skip=0&amp;limit=10&amp;category=All\">/api/astrologers</a></div>",
            nav(),
        ),
    )
}

/// `GET /free-kundli`
pub fn kundli_form() -> Html<String> {
    page(
        "Free Kundli",
        &format!(
            "{}<h1>Free Kundli (chart facts)</h1>\
            <p>Mandatory request shape: <code>date</code>, <code>time</code>, <code>tzone</code>; \
            <code>name</code> optional.</p>\
            <form action=\"/chart\" method=\"get\" id=\"nf\">\
              <label>Your name <input name=\"name\" placeholder=\"optional\"></label><br>\
              <label>Birth date (YYYY-MM-DD) <input name=\"date\" required pattern=\"\\d{{4}}-\\d{{2}}-\\d{{2}}\"></label><br>\
              <label>Birth time (HH:MM, 24h) <input name=\"time\" required></label><br>\
              <label>Timezone offset from UTC (e.g. 5.5 for IST) <input name=\"tzone\" value=\"5.5\" required></label><br>\
              <button type=\"submit\">Compute chart</button>\
            </form>\
            <p>JSON response fields: <code>utc, julian_day, ayanamsa_deg, sun, moon, nakshatra, tithi</code>.</p>",
            nav(),
        ),
    )
}

/// Panchang city directory — mirrors the 10 slugs observed on the target.
pub fn panchang_index<C: Iterator<Item = (String, String, String)>>(
    cities: C,
) -> Html<String> {
    let mut rows = String::new();
    for (slug, city, _state) in cities {
        rows.push_str(&format!(
            "<li><a href=\"/panchang/{slug}/today\">{city}</a> · <a href=\"/panchang/{slug}/tomorrow\">tomorrow</a></li>"
        ));
    }
    page(
        "City Panchang",
        &format!(
            "{}<h1>City panchang</h1><ul>{rows}</ul><p>Each page shows sunrise/sunset, tithi, \
            nakshatra and moon phase (approximations — see crate docs).</p>",
            nav(),
        ),
    )
}

/// Horoscope detail page (HTML default view).
pub fn horoscope_page<'a>(
    sign: &str,
    period: &str,
    anchor_date: &str,
    message: &str,
    all_signs: impl Iterator<Item = String>,
    current_slug: &str,
) -> Html<String> {
    let mut links = String::new();
    for s in all_signs {
        if s == current_slug {
            links.push_str(&format!("<a href=\"/horoscope/{period}?sign={s}\" class=\"cur\">{}</a> ", cap(&s)));
        } else {
            links.push_str(&format!("<a href=\"/horoscope/{period}?sign={s}\">{}</a> ", cap(&s)));
        }
    }
    page(
        &format!("{sign} — {period} horoscope"),
        &format!(
            "{}<h1>{} — {period} horoscope</h1><p id=\"anchor\">Anchor: {anchor_date}</p>\
             <div class=\"card\" id=\"horoscope\">{message}</div>\
             <nav class=\"signs\">{links}</nav>\
             <p>JSON: append <code>&amp;format=json</code>.</p>",
            nav(),
            sign,
        ),
    )
}

fn cap(s: &str) -> String {
    let mut c = s.to_string();
    if let Some(f) = c.get_mut(0..1) {
        f.make_ascii_uppercase();
    }
    c
}

/// Panchang city page wrapper.
pub fn panchang_page(inner: String) -> Html<String> {
    page("City Panchang", &format!("{}{inner}", nav()))
}
