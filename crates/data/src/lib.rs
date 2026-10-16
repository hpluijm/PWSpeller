// ── Over dit bestand ──────────────────────────────────────
// Doel:           Bibliotheek met het datamodel en het inlezen en controleren van de taalbestanden van PWSpeller.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Modules model, inlezen en controle toegevoegd.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Lege bibliotheek aangemaakt.
// ──────────────────────────────────────────────────────────

//! Datamodel, inlezen en controleren van de taalbestanden (alfabetten, omschrijvingen,
//! woordenlijsten, spellingstalen en interfaceteksten).

pub mod controle;
pub mod inlezen;
pub mod model;

pub use controle::controleer;
pub use inlezen::lees_catalogus;
pub use model::{
    Alfabet, Catalogus, DataFout, LastigeKlanken, Omschrijvingen, Sjablonen, Spellingstaal,
    Woordenlijst,
};

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
