// ── Over dit bestand ──────────────────────────────────────
// Doel:           De ingebakken taaldata: catalogus, interfaceteksten en omschrijvingen per spellingstaal.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   interfacetekst geeft bij een onbekende sleutel de sleutel terug, zonder geheugenlek.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Module talen toegevoegd.
// ──────────────────────────────────────────────────────────

//! De taaldata uit `data/`, bij het bouwen ingebakken en gecontroleerd door `build.rs`.

use std::sync::OnceLock;

pub use pwspeller_data::*;

include!(concat!(env!("OUT_DIR"), "/bestanden.rs"));

/// De ingebouwde catalogus. Wordt één keer ingelezen.
pub fn catalogus() -> &'static Catalogus {
    static CATALOGUS: OnceLock<Catalogus> = OnceLock::new();
    CATALOGUS.get_or_init(|| match lees_catalogus(BESTANDEN) {
        Ok(c) => c,
        Err(fouten) => {
            let regels: Vec<String> = fouten.iter().map(|f| f.to_string()).collect();
            panic!("ingebouwde taaldata onleesbaar (build.rs had dit moeten afvangen):\n{}", regels.join("\n"))
        }
    })
}

/// De interfacetekst voor `sleutel`: eerst de gevraagde taal, dan Engels, dan de sleutel zelf.
pub fn interfacetekst<'a>(taal: &str, sleutel: &'a str) -> &'a str {
    let interface = &catalogus().interface;
    [taal, "en"]
        .iter()
        .find_map(|t| interface.get(*t).and_then(|m| m.get(sleutel)))
        .map(String::as_str)
        .unwrap_or(sleutel)
}

/// De omschrijvingen van een spellingstaal: de eigen, anders die van de interfacetaal, anders Engels.
pub fn omschrijvingen_voor(t: &Spellingstaal, interfacetaal: &str) -> &'static Omschrijvingen {
    let o = &catalogus().omschrijvingen;
    t.omschrijvingen
        .as_deref()
        .and_then(|c| o.get(c))
        .or_else(|| o.get(interfacetaal))
        .or_else(|| o.get("en"))
        .expect("omschrijvingen voor 'en' ontbreken (build.rs had dit moeten afvangen)")
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
