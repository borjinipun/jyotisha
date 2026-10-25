# Jyotisha

An open-source **Rust** astrology toolkit and web server: Vedic/Western zodiac facts,
panchang (tithi, nakshatra, sunrise/sunset, moon phase), daily/weekly/monthly horoscope
text, and a bot-listing API — served over HTTP with a small, clean, dependency-light
server built on `axum`.

All astronomical mathematics follows published algorithms: Jean Meeus,
*Astronomical Algorithms* (2nd ed.) for Julian Day, apparent solar longitude, and lunar
longitudes (top-10 periodic terms), with a Lahiri-style linear ayanamsa.

## Layout

```
crates/
  astrology/            pure astronomy/astrology engine (no IO)
    julian.rs           Julian Day computation (Meeus ch. 7)
    sun.rs              apparent solar longitude (Meeus ch. 25)
    moon.rs             lunar longitude, top-10 periodic terms (Meeus ch. 47)
    ayanamsa.rs         Lahiri-style linear ayanamsa
    zodiac.rs           sign/nakshatra pada derivation
    nakshatra.rs        27 nakshatra facts
    tithi.rs            30 tithi phases
    sunrise.rs          sunrise/sunset
    horoscope.rs        deterministic text generation (seeded, no LLM)
    data.rs             10 Indian city coordinates/timezones
  server/               axum web server (lib + bin)
    main.rs             jyotisha-server binary (--port, --bind)
    lib.rs              router assembly
    pages.rs            index, /free-kundli, /panchang directory
    horoscope.rs        /horoscope/{period}
    chart.rs            /chart GET+POST
    panchang.rs         /panchang/{city}/{day}
    astrologers.rs      /api/astrologers
    error.rs            {"error":{"code","message"}}
    parse_util.rs       strict request parsing
    tests/router_it.rs  16 integration tests (router oneshot)
```

## Build, test, run

Requires a Rust toolchain (developed with 1.99.1).

```sh
cargo test            # 37 tests: 21 engine + 16 integration
cargo build --release # produces target/release/jyotisha-server
./target/release/jyotisha-server --port 8080
```

Deterministic "today": pass `JYOTISHA_TODAY=YYYY-MM-DD`, or `?date=` per request, for
reproducible horoscope/panchang output.

## Endpoints

| Route | Behavior |
| --- | --- |
| `GET /` | index page with links |
| `GET /healthz` | liveness JSON |
| `GET /horoscope/{period}?sign=` | daily/tomorrow/weekly/monthly/yearly text; HTML default, `?format=json` |
| `GET /free-kundli` | chart input form |
| `GET/POST /chart` | evaluates name/date/time/tzone into zodiac, nakshatra, tithi facts; strict `400` envelopes on `invalid_date`/`invalid_time`/`invalid_tzone` |
| `GET /panchang` | city directory (10 cities) |
| `GET /panchang/{city}/{today\|tomorrow}` | sunrise/sunset, nakshatra, tithi, moon phase; `404 invalid_city` otherwise |
| `GET /api/astrologers` | seeded sample listing (`entityName`, `entityCode`, `entityCategories[]`, `entityRating`, …) with `skip`/`limit`/`category` paging |
| `GET /astrologer/{code}` | detail page or `404 unknown_astrologer` |
| *(fallback)* | unknown route → JSON 404 envelope |

## Determinism / test gates

- Horoscope text is seeded by `(sign, period, anchor date)` via a deterministic hash;
  tests assert identical output across repeated calls.
- All endpoint responses are fully determined by inputs; no wall-clock dependency when
  `JYOTISHA_TODAY`/`?date=` is supplied.
- `cargo test --workspace` is the release gate.

## License

MIT — see [LICENSE](LICENSE).
