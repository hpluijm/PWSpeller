// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test het bewaren, laden en de interfacetaal van de instellingen van de app.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: tests voor instellingen aangemaakt.
// ──────────────────────────────────────────────────────────

use pwspeller::instellingen::{Instellingen, interfacetaal, kies_interfacetaal};
use std::path::PathBuf;

fn testmap(naam: &str) -> PathBuf {
    let map = std::env::temp_dir().join(format!("pwspeller-test-{}-{naam}", std::process::id()));
    let _ = std::fs::remove_dir_all(&map);
    map
}

#[test]
fn bewaren_en_laden_geeft_gelijke_instellingen() {
    let map = testmap("heen-terug");
    let pad = map.join("diep/instellingen.toml");
    let mut i = Instellingen {
        interfacetaal: Some("en".to_string()),
        spellingstaal: "fr".to_string(),
        woordenlijst: Some("nl".to_string()),
        klembord_seconden: 45,
        geschiedenis_minuten: None,
        ..Default::default()
    };
    i.wachtwoord.lengte = 20;
    i.wachtzin.woorden = 6;
    i.bewaar(&pad)
        .expect("bewaren moet lukken en de map aanmaken");
    assert_eq!(Instellingen::laad(&pad), i);
    assert!(
        !pad.with_extension("toml.tmp").exists(),
        "tijdelijk bestand blijft niet achter"
    );
    let _ = std::fs::remove_dir_all(&map);
}

#[test]
fn kapot_bestand_geeft_default() {
    let map = testmap("kapot");
    std::fs::create_dir_all(&map).unwrap();
    let pad = map.join("instellingen.toml");
    std::fs::write(&pad, "dit is geen = [ toml").unwrap();
    assert_eq!(Instellingen::laad(&pad), Instellingen::default());
    let _ = std::fs::remove_dir_all(&map);
}

#[test]
fn ontbrekend_bestand_geeft_default() {
    let pad = testmap("ontbreekt").join("instellingen.toml");
    assert_eq!(Instellingen::laad(&pad), Instellingen::default());
}

#[test]
fn standaarden_kloppen() {
    let i = Instellingen::default();
    assert_eq!(i.interfacetaal, None);
    assert_eq!(i.spellingstaal, "nl");
    assert_eq!(i.woordenlijst, None);
    assert_eq!(i.klembord_seconden, 30);
    assert_eq!(i.geschiedenis_minuten, Some(15));
}

#[cfg(target_os = "macos")]
#[test]
fn op_macos_wist_het_klembord_standaard_niet() {
    assert!(!Instellingen::default().klembord_wissen);
}

#[test]
fn interfacetaal_buiten_de_catalogus_wordt_engels() {
    // Gekozen "de" staat niet in de catalogus; het systeem (hier ook "de") evenmin.
    assert_eq!(
        kies_interfacetaal(Some("de"), Some("de-DE".to_string())),
        "en"
    );
    assert_eq!(kies_interfacetaal(None, None), "en");
}

#[test]
fn interfacetaal_volgt_keuze_dan_systeem() {
    assert_eq!(
        kies_interfacetaal(Some("nl"), Some("en-US".to_string())),
        "nl"
    );
    assert_eq!(kies_interfacetaal(None, Some("NL_nl".to_string())), "nl");
    assert_eq!(
        kies_interfacetaal(Some("de"), Some("nl-NL".to_string())),
        "nl"
    );
}

#[test]
fn interfacetaal_van_de_instellingen_geeft_een_taal_uit_de_catalogus() {
    let i = Instellingen {
        interfacetaal: Some("en".to_string()),
        ..Default::default()
    };
    assert_eq!(interfacetaal(&i), "en");
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
