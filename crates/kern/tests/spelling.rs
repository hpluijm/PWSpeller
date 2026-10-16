// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test van de spelling: tekst omzetten in regels om telefonisch voor te lezen.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Tests voor spelling toegevoegd.
// ──────────────────────────────────────────────────────────

use pwspeller_kern::spelling::{Regel, Soort, Speller};
use pwspeller_kern::talen::{Spellingstaal, catalogus, omschrijvingen_voor};
use std::time::{Duration, Instant};

fn taal(code: &str) -> &'static Spellingstaal {
    catalogus()
        .spellingstalen
        .iter()
        .find(|t| t.code == code)
        .expect("spellingstaal bestaat")
}

fn speller(code: &str) -> Speller<'static> {
    let t = taal(code);
    Speller::nieuw(
        &catalogus().alfabetten[&t.alfabet],
        omschrijvingen_voor(t, "nl"),
    )
}

fn omschr(regels: Vec<Regel>) -> Vec<String> {
    regels.into_iter().map(|r| r.omschrijving).collect()
}

#[test]
fn nl_per_teken() {
    assert_eq!(
        omschr(speller("nl").per_teken("Ab7!")),
        [
            "hoofdletter Anna",
            "kleine letter Bernard",
            "cijfer zeven",
            "uitroepteken"
        ]
    );
}

#[test]
fn en_gebruikt_navo_met_capital() {
    assert_eq!(omschr(speller("en").per_teken("A"))[0], "capital Alfa");
}

#[test]
fn spatie_en_onbekend() {
    let r = speller("nl").per_teken("a é");
    assert_eq!(r[1].soort, Soort::Spatie);
    assert_eq!(r[1].omschrijving, "spatie");
    assert_eq!(r[2].omschrijving, "teken é (U+00E9)");
    assert_eq!(r[2].soort, Soort::Onbekend);
}

#[test]
fn soorten_per_teken() {
    let r = speller("nl").per_teken("Ab7!");
    let soorten: Vec<Soort> = r.iter().map(|r| r.soort).collect();
    assert_eq!(
        soorten,
        [
            Soort::Hoofdletter,
            Soort::KleineLetter,
            Soort::Cijfer,
            Soort::Speciaal
        ]
    );
    assert!(r.iter().all(|r| !r.lastig));
}

#[test]
fn hoofdletter_buiten_alfabet_is_onbekend() {
    let r = speller("nl").per_teken("É");
    assert_eq!(r[0].soort, Soort::Onbekend);
    assert_eq!(r[0].omschrijving, "teken É (U+00C9)");
}

#[test]
fn emoji_en_lange_invoer_crashen_niet() {
    let tekst = "👍🏽\t".repeat(5000);
    let r = speller("nl").per_teken(&tekst);
    assert_eq!(r.len(), tekst.chars().count());
    assert_eq!(r[0].omschrijving, "teken 👍 (U+1F44D)");
    assert_eq!(r[2].omschrijving, "teken \t (U+0009)");
}

#[test]
fn tienduizend_tekens_ruim_binnen_een_seconde() {
    let s = speller("nl");
    let tekst = "Ab7!é".repeat(2000);
    let start = Instant::now();
    let r = s.per_teken(&tekst);
    assert_eq!(r.len(), 10_000);
    assert!(start.elapsed() < Duration::from_millis(500));
}

#[test]
fn wachtzin_per_woord() {
    let w = vec!["Zonnig".into(), "tafel7".into(), "berg".into()];
    assert_eq!(
        omschr(speller("nl").wachtzin(&w, "-")),
        [
            "Zonnig, met hoofdletter",
            "streepje",
            "tafel",
            "cijfer zeven",
            "streepje",
            "berg"
        ]
    );
}

#[test]
fn lastig_woord_wordt_per_letter_gespeld_en_gemarkeerd() {
    let w = vec!["wijd".into()];
    let r = speller("nl").wachtzin(&w, "-");
    assert_eq!(
        omschr(r.clone()),
        [
            "kleine letter Willem",
            "kleine letter Izaak",
            "kleine letter Jan",
            "kleine letter Dirk"
        ]
    );
    assert!(r.iter().all(|r| r.lastig));
}

#[test]
fn lastig_woord_met_hoofdletter_spelt_de_eerste_als_hoofdletter() {
    let w = vec!["Wijn".into()];
    let r = speller("nl").wachtzin(&w, "-");
    assert_eq!(r[0].omschrijving, "hoofdletter Willem");
    assert_eq!(r[0].soort, Soort::Hoofdletter);
    assert!(r.iter().all(|r| r.lastig));
}

#[test]
fn eindcijfer_telt_niet_mee_voor_lastig() {
    let s = speller("nl");
    assert!(s.is_lastig("wijd"));
    assert!(s.is_lastig("Wijd7"));
    assert!(s.is_lastig("tast")); // eindigt op t
    assert!(!s.is_lastig("tafel"));
    // Het eindcijfer wordt eerst losgehaald: "wal7d" blijft lastig, "tafel7" niet.
    assert!(s.is_lastig("wal7d"));
    assert!(!s.is_lastig("tafel7"));
}

#[test]
fn lastig_woord_met_eindcijfer_geeft_cijferregel_niet_lastig() {
    let w = vec!["wijd7".into()];
    let r = speller("nl").wachtzin(&w, "-");
    assert_eq!(r.len(), 5);
    assert!(r[..4].iter().all(|r| r.lastig));
    assert_eq!(r[4].omschrijving, "cijfer zeven");
    assert!(!r[4].lastig);
}

#[test]
fn scheidingsteken_van_meer_tekens_wordt_per_teken_gespeld() {
    let w = vec!["a".into(), "b".into()];
    let r = speller("nl").wachtzin(&w, "--");
    assert_eq!(omschr(r), ["a", "streepje", "streepje", "b"]);
}

#[test]
fn posities_lopen_door_over_woorden_en_scheidingstekens() {
    let w = vec!["Zonnig".into(), "tafel7".into(), "berg".into()];
    let r = speller("nl").wachtzin(&w, "--");
    let posities: Vec<usize> = r.iter().map(|r| r.positie).collect();
    // Zonnig(1-6) -- (7,8) tafel(9-13) 7(14) -- (15,16) berg(17-20)
    assert_eq!(posities, [1, 7, 8, 9, 14, 15, 16, 17]);
    assert_eq!(r[0].fragment, "Zonnig");
    assert_eq!(r[3].fragment, "tafel");
    assert_eq!(r[4].fragment, "7");
    assert_eq!(r[1].fragment, "-");
    assert_eq!(r[1].soort, Soort::Speciaal);
    assert_eq!(r[0].soort, Soort::Woord);
}

#[test]
fn per_teken_posities_zijn_een_gebaseerd_in_chars() {
    let r = speller("nl").per_teken("é👍a");
    let posities: Vec<usize> = r.iter().map(|r| r.positie).collect();
    assert_eq!(posities, [1, 2, 3]);
}

#[test]
fn lege_invoer_geeft_geen_regels() {
    let s = speller("nl");
    assert!(s.per_teken("").is_empty());
    assert!(s.wachtzin(&[], "-").is_empty());
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
