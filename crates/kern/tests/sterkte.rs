// ── Over dit bestand ──────────────────────────────────────
// Doel:           Tests van de sterkteberekening: bits per wachtwoord, wachtzin en eigen tekst, en de niveaus.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Tests voor de sterkte-module toegevoegd (eerst falend, daarna groen).
// ──────────────────────────────────────────────────────────

use pwspeller_kern::sterkte::{Niveau, niveau, schatting_bits, wachtwoord_bits, wachtzin_bits};

/// Vergelijkt twee floats met een tolerantie van 0,01 bit.
fn benaderd(werkelijk: f64, verwacht: f64) {
    assert!(
        (werkelijk - verwacht).abs() < 0.01,
        "verwacht ≈ {verwacht}, kreeg {werkelijk}"
    );
}

#[test]
fn niveau_grenzen() {
    assert_eq!(niveau(59.9), Niveau::Zwak);
    assert_eq!(niveau(60.0), Niveau::Redelijk);
    assert_eq!(niveau(80.0), Niveau::Redelijk);
    assert_eq!(niveau(80.1), Niveau::Sterk);
}

#[test]
fn wachtwoord_bits_is_lengte_maal_log2_pool() {
    benaderd(wachtwoord_bits(14, 62), 83.36);
}

#[test]
fn wachtzin_zonder_cijfer() {
    benaderd(wachtzin_bits(6, 7776, false), 77.55);
}

#[test]
fn wachtzin_met_cijfer_telt_log2_van_tien_maal_woorden_erbij() {
    // 6 × log2(7776) + log2(10 × 6) = 77,55 + 5,91
    benaderd(wachtzin_bits(6, 7776, true), 83.46);
}

#[test]
fn schatting_leeg_is_nul() {
    assert_eq!(schatting_bits(""), 0.0);
}

#[test]
fn schatting_alleen_kleine_letters() {
    benaderd(schatting_bits("abc"), 3.0 * 26f64.log2());
}

#[test]
fn schatting_telt_elke_tekensoort_eenmaal_in_de_pool() {
    // pool = 26 + 26 + 10 + 33 = 95, lengte 4
    benaderd(schatting_bits("Ab1!"), 4.0 * 95f64.log2());
}

#[test]
fn schatting_niet_ascii_telt_als_honderd() {
    benaderd(schatting_bits("é"), 100f64.log2());
}

#[test]
fn schatting_telt_tekens_niet_bytes() {
    // "éé" is 4 bytes in UTF-8 maar 2 tekens: 2 × log2(100)
    benaderd(schatting_bits("éé"), 2.0 * 100f64.log2());
}

#[test]
fn schatting_gemengd_pool_en_lengte() {
    // pool = 26 + 100 = 126, lengte 2
    benaderd(schatting_bits("aé"), 2.0 * 126f64.log2());
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
