// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test van de ingebakken taaldata: catalogus, interfaceteksten en omschrijvingen.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Test met tijdelijke String als sleutel toegevoegd.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Tests voor talen toegevoegd.
// ──────────────────────────────────────────────────────────

use pwspeller_kern::talen::{catalogus, interfacetekst, omschrijvingen_voor};

#[test]
fn ingebouwde_catalogus_heeft_zes_spellingstalen() {
    assert_eq!(catalogus().spellingstalen.len(), 6);
}

#[test]
fn onbekende_interfacetaal_valt_terug_op_engels() {
    assert_eq!(interfacetekst("xx", "knop.genereer"), interfacetekst("en", "knop.genereer"));
    assert_eq!(interfacetekst("en", "knop.genereer"), "Generate");
}

#[test]
fn onbekende_sleutel_geeft_sleutel_terug() {
    assert_eq!(interfacetekst("nl", "bestaat.niet"), "bestaat.niet");
}

#[test]
fn onbekende_sleutel_werkt_met_tijdelijke_string() {
    let s = String::from("bestaat.niet");
    assert_eq!(interfacetekst("nl", &s), "bestaat.niet");
}

#[test]
fn navo_gebruikt_omschrijvingen_van_de_interfacetaal() {
    let navo = catalogus().spellingstalen.iter().find(|t| t.code == "navo").unwrap();
    assert!(navo.omschrijvingen.is_none());
    assert_eq!(omschrijvingen_voor(navo, "nl").code, "nl");
    assert_eq!(omschrijvingen_voor(navo, "de").code, "de");
    assert_eq!(omschrijvingen_voor(navo, "xx").code, "en");
}

#[test]
fn eigen_omschrijvingen_gaan_voor_de_interfacetaal() {
    let vl = catalogus().spellingstalen.iter().find(|t| t.code == "vl").unwrap();
    assert_eq!(omschrijvingen_voor(vl, "en").code, "nl");
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
