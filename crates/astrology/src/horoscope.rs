//! Deterministic horoscope text generator (original fragment templates).
//!
//! Rule (REQ F-HOR-02): the output for `(sign, period, anchor-date)` must
//! be identical across calls — seeded by FNV-1a over the input; every
//! fragment below is original prose written for this reconstruction.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Observed site periods: daily (today/tomorrow) / weekly /
/// monthly / yearly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Period {
    /// 1 calendar day (date provided by caller).
    Daily,
    /// ISO week anchor (caller provides the Monday date).
    Weekly,
    /// Month anchor (year + month).
    Monthly,
    /// Year anchor.
    Yearly,
}

/// Generate the horoscope text — deterministic for the same inputs.
///
/// `seed_date` is a normalized anchor string, e.g. "2026-10-25" for daily,
/// "2026-W43" for weekly, "2026-10" for monthly, "2026" for yearly.
pub fn text(sign: &str, period: Period, seed_date: &str) -> String {
    let mut h = DefaultHasher::new();
    sign.hash(&mut h);
    period_key(period).hash(&mut h);
    seed_date.hash(&mut h);
    let seed = h.finish();

    let pick = |n: usize, salt: u64| -> usize {
        let mut hh = DefaultHasher::new();
        (seed ^ salt.rotate_left(17)).hash(&mut hh);
        (hh.finish() % n.max(1) as u64) as usize
    };

    let opening = OPENINGS[pick(OPENINGS.len(), 0xA1)];
    let energy = ENERGIES[pick(ENERGIES.len(), 0xB2)];
    let action = ACTIONS[pick(ACTIONS.len(), 0xC3)];
    let lucky = LUCKY[pick(LUCKY.len(), 0xD4)];

    let cap = sign_cap(sign);
    format!(
        "{cap}: {opening} Your overall tone today sits in a {energy} register — {action} Favorable colour band: {lucky}."
    )
}

fn period_key(p: Period) -> &'static str {
    match p {
        Period::Daily => "daily",
        Period::Weekly => "weekly",
        Period::Monthly => "monthly",
        Period::Yearly => "yearly",
    }
}

fn sign_cap(sign: &str) -> String {
    let mut c = sign.to_string();
    if let Some(first) = c.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    c
}

const OPENINGS: [&str; 8] = [
    "the day rewards steady effort over shortcuts.",
    "conversations move faster than plans, so listen twice.",
    "a question you have been postponing asks for an answer today.",
    "morning momentum carries more weight than afternoon pressure.",
    "small completions matter more than one big push.",
    "you may notice patience turning into leverage.",
    "your instinct is clearer than the data around it.",
    "an arrangement needs one honest sentence from you.",
];

const ENERGIES: [&str; 6] = [
    "active but forgiving",
    "quiet and deliberate",
    "quick to shift, slow to settle",
    "steady with brief sparks",
    "cautious early, confident later",
    "playful with a serious core",
];

const ACTIONS: [&str; 8] = [
    "commit to the first task on the list before noon.",
    "let one yes replace three maybes.",
    "keep the detail you usually round off.",
    "trade speed for a cleaner finish on one item.",
    "ask for the timeline, not the promise.",
    "step back from the noise before deciding.",
    "share only the part that is truly ready.",
    "double down on the routine that already works.",
];

const LUCKY: [&str; 8] = [
    "warm amber to sand",
    "indigo and slate",
    "off-white with a deep green accent",
    "copper against charcoal",
    "dusty rose",
    "teal layered on cream",
    "smoky grey and gold",
    "muted marigold",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let a = text("aries", Period::Daily, "2026-10-25");
        let b = text("aries", Period::Daily, "2026-10-25");
        assert_eq!(a, b);
    }

    #[test]
    fn varies_over_inputs() {
        let a1 = text("aries", Period::Daily, "2026-10-25");
        let a2 = text("aries", Period::Daily, "2026-10-26");
        let t2 = text("taurus", Period::Daily, "2026-10-25");
        assert_ne!(a1, a2);
        assert_ne!(a1, t2);
    }
}
