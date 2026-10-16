// ── Over dit bestand ──────────────────────────────────────
// Doel:           Gemeenschappelijke typen van de generator: modus, resultaat en fouten.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 8: module wachtzin, fout LegeWoordenlijst, woorden in Zeroizing<Vec<String>>.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fixronde 1: genereer_wachtwoord_os geëxporteerd, doc bij entropie_bits.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Module aangemaakt.
// ──────────────────────────────────────────────────────────

//! Generatoren voor wachtwoorden en wachtzinnen.

use std::fmt;
use zeroize::Zeroizing;

pub mod wachtwoord;
pub mod wachtzin;

pub use wachtwoord::{
    DUBBELZINNIG, SPECIALE_TEKENS, WachtwoordOpties, effectieve_lengte, genereer_wachtwoord,
    genereer_wachtwoord_os,
};
pub use wachtzin::{WachtzinOpties, genereer_wachtzin, genereer_wachtzin_os};

/// Waarom een generator niets kon maken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratorFout {
    GeenTekensetAan,
    OngeldigeLengte,
    OngeldigAantalWoorden,
    LeegScheidingsteken,
    LegeWoordenlijst,
}

impl fmt::Display for GeneratorFout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::GeenTekensetAan => "Er staat geen enkele tekenset aan.",
            Self::OngeldigeLengte => "De lengte moet tussen 5 en 128 liggen.",
            Self::OngeldigAantalWoorden => "Het aantal woorden moet tussen 3 en 20 liggen.",
            Self::LeegScheidingsteken => "Het scheidingsteken mag niet leeg zijn.",
            Self::LegeWoordenlijst => "De woordenlijst is leeg.",
        })
    }
}

impl std::error::Error for GeneratorFout {}

/// Wat er gegenereerd is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modus {
    Wachtwoord,
    Wachtzin,
}

/// Een gegenereerd wachtwoord of een gegenereerde wachtzin.
pub struct Gegenereerd {
    pub tekst: Zeroizing<String>,
    pub modus: Modus,
    /// Alleen bij een wachtzin: de woorden mét eventuele hoofdletter en cijfer.
    pub woorden: Option<Zeroizing<Vec<String>>>,
    /// Alleen bij een wachtzin.
    pub scheidingsteken: Option<String>,
    /// Bovengrens: lengte × log2(pool). De minima per set (die een deel van de tekens
    /// vastleggen) zijn niet afgetrokken, dus de werkelijke entropie ligt iets lager.
    pub entropie_bits: f64,
}

/// De tekst blijft uit de Debug-uitvoer, zodat hij niet per ongeluk in een logboek belandt.
impl fmt::Debug for Gegenereerd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Gegenereerd")
            .field("tekst", &"***")
            .field("modus", &self.modus)
            .field("woorden", &self.woorden.as_ref().map(|w| w.len()))
            .field("scheidingsteken", &self.scheidingsteken)
            .field("entropie_bits", &self.entropie_bits)
            .finish()
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
