// ── Over dit bestand ──────────────────────────────────────
// Doel:           Datatypes van de catalogus: alfabetten, omschrijvingen, woordenlijsten en spellingstalen.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Datamodel aangemaakt.
// ──────────────────────────────────────────────────────────

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Deserialize;

/// Het codewoord per letter van één alfabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alfabet {
    pub code: String,
    pub naam: String,
    pub letters: BTreeMap<char, String>,
}

/// Zinsdelen met plaatsaanduidingen als `{woord}`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Sjablonen {
    pub hoofdletter: String,
    pub kleine_letter: String,
    pub cijfer: String,
    pub woord: String,
    pub woord_met_hoofdletter: String,
    pub onbekend: String,
}

/// Klanken waarbij een extra uitleg nodig is.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LastigeKlanken {
    pub bevat: Vec<String>,
    pub eindigt_op: Vec<String>,
}

/// De woorden rond de codewoorden, in één taal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Omschrijvingen {
    pub code: String,
    pub sjablonen: Sjablonen,
    pub cijfers: BTreeMap<char, String>,
    pub tekens: BTreeMap<char, String>,
    pub lastige_klanken: LastigeKlanken,
}

/// Een woordenlijst voor wachtzinnen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Woordenlijst {
    pub code: String,
    pub woorden: Vec<String>,
}

/// Een combinatie van alfabet, omschrijvingen en woordenlijst.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Spellingstaal {
    pub code: String,
    pub naam: String,
    pub alfabet: String,
    /// `None` is de taal van de interface.
    #[serde(default)]
    pub omschrijvingen: Option<String>,
    pub woordenlijst: String,
}

/// Alles wat uit de bestanden in `data/` is ingelezen.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Catalogus {
    pub alfabetten: BTreeMap<String, Alfabet>,
    pub omschrijvingen: BTreeMap<String, Omschrijvingen>,
    pub woordenlijsten: BTreeMap<String, Woordenlijst>,
    pub geblokkeerd: BTreeMap<String, BTreeSet<String>>,
    pub spellingstalen: Vec<Spellingstaal>,
    pub interface: BTreeMap<String, BTreeMap<String, String>>,
}

/// Een fout in een databestand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFout {
    pub bestand: String,
    pub melding: String,
}

impl fmt::Display for DataFout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.bestand, self.melding)
    }
}

impl std::error::Error for DataFout {}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
