// ── Over dit bestand ──────────────────────────────────────
// Doel:           Wachtzingenerator naar het concept van Bitwarden: uniforme woordkeuze, hoofdletters en een cijfer.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Module aangemaakt (Taak 8).
// ──────────────────────────────────────────────────────────

//! Wachtzingenerator. Elk woord wordt uniform uit de lijst gekozen; het cijfer komt
//! achter één uniform gekozen woord. De woorden en de tekst staan in `Zeroizing`.

use super::{Gegenereerd, GeneratorFout, Modus};
use crate::sterkte::wachtzin_bits;
use crate::talen::Woordenlijst;
use rand::seq::IndexedRandom;
use rand::{Rng, RngExt};
use zeroize::Zeroizing;

const MIN_WOORDEN: u8 = 3;
const MAX_WOORDEN: u8 = 20;

/// Instellingen van de wachtzingenerator.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WachtzinOpties {
    pub woorden: u8,
    pub scheidingsteken: String,
    pub hoofdletters: bool,
    pub cijfer: bool,
}

impl Default for WachtzinOpties {
    fn default() -> Self {
        Self {
            woorden: 6,
            scheidingsteken: "-".to_string(),
            hoofdletters: false,
            cijfer: false,
        }
    }
}

/// Genereert een wachtzin met de willekeurbron van het besturingssysteem.
/// `rand::rng()` is in rand 0.10 een ChaCha12-generator die uit het besturingssysteem
/// geseed wordt en cryptografisch veilig is. Gebruik dit in productie.
pub fn genereer_wachtzin_os(
    o: &WachtzinOpties,
    lijst: &Woordenlijst,
) -> Result<Gegenereerd, GeneratorFout> {
    genereer_wachtzin(o, lijst, &mut rand::rng())
}

/// Genereert een wachtzin met een willekeurbron naar keuze: alleen voor tests met vaste seed.
/// Fouten: `OngeldigAantalWoorden` (buiten 3 tot 20), `LeegScheidingsteken`, `LegeWoordenlijst`.
#[doc(hidden)]
pub fn genereer_wachtzin<R: Rng + ?Sized>(
    o: &WachtzinOpties,
    lijst: &Woordenlijst,
    rng: &mut R,
) -> Result<Gegenereerd, GeneratorFout> {
    if !(MIN_WOORDEN..=MAX_WOORDEN).contains(&o.woorden) {
        return Err(GeneratorFout::OngeldigAantalWoorden);
    }
    if o.scheidingsteken.is_empty() {
        return Err(GeneratorFout::LeegScheidingsteken);
    }
    if lijst.woorden.is_empty() {
        return Err(GeneratorFout::LegeWoordenlijst);
    }
    let aantal = o.woorden as usize;

    // Elk woord is een nieuwe String in beschermd geheugen; de lijst zelf wordt niet gewijzigd.
    let mut woorden: Zeroizing<Vec<String>> = Zeroizing::new(Vec::with_capacity(aantal));
    for _ in 0..aantal {
        let bron = lijst.woorden.choose(rng).expect("lijst is niet leeg");
        let mut woord = String::with_capacity(bron.len() + 1);
        let mut tekens = bron.chars();
        match (o.hoofdletters, tekens.next()) {
            (true, Some(eerste)) => {
                woord.extend(eerste.to_uppercase());
                woord.push_str(tekens.as_str());
            }
            _ => woord.push_str(bron),
        }
        woorden.push(woord);
    }
    if o.cijfer {
        let plek = rng.random_range(0..aantal);
        let cijfer = rng.random_range(0..=9u8);
        woorden[plek].push(char::from(b'0' + cijfer));
    }

    let lengte: usize =
        woorden.iter().map(String::len).sum::<usize>() + o.scheidingsteken.len() * (aantal - 1);
    let mut tekst = Zeroizing::new(String::with_capacity(lengte));
    for (i, woord) in woorden.iter().enumerate() {
        if i > 0 {
            tekst.push_str(&o.scheidingsteken);
        }
        tekst.push_str(woord);
    }

    Ok(Gegenereerd {
        tekst,
        modus: Modus::Wachtzin,
        woorden: Some(woorden),
        scheidingsteken: Some(o.scheidingsteken.clone()),
        entropie_bits: wachtzin_bits(aantal, lijst.woorden.len(), o.cijfer),
    })
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
