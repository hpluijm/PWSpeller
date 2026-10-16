// ── Over dit bestand ──────────────────────────────────────
// Doel:           Wachtwoordgenerator naar het concept van Bitwarden: tekensets, minima, uniforme trekking en schudden.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Module aangemaakt.
// ──────────────────────────────────────────────────────────

//! Wachtwoordgenerator. Per tekenset wordt het minimum getrokken, de rest komt uit
//! de vereniging van de sets die aanstaan, daarna wordt alles geschud. Alle
//! trekkingen zijn uniform (`random_range`, geen modulo).

use super::{Gegenereerd, GeneratorFout, Modus};
use crate::sterkte::wachtwoord_bits;
use rand::seq::SliceRandom;
use rand::{Rng, RngExt};
use zeroize::Zeroizing;

pub const SPECIALE_TEKENS: [char; 8] = ['!', '@', '#', '$', '%', '^', '&', '*'];
pub const DUBBELZINNIG: [char; 5] = ['I', 'O', 'l', '0', '1'];

const MIN_LENGTE: u8 = 5;
const MAX_LENGTE: u8 = 128;

/// Instellingen van de wachtwoordgenerator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WachtwoordOpties {
    pub lengte: u8,
    pub hoofdletters: bool,
    pub kleine_letters: bool,
    pub cijfers: bool,
    pub speciaal: bool,
    pub min_cijfers: u8,
    pub min_speciaal: u8,
    pub vermijd_dubbelzinnig: bool,
}

impl Default for WachtwoordOpties {
    fn default() -> Self {
        Self {
            lengte: 14,
            hoofdletters: true,
            kleine_letters: true,
            cijfers: true,
            speciaal: false,
            min_cijfers: 1,
            min_speciaal: 1,
            vermijd_dubbelzinnig: false,
        }
    }
}

/// De sets die aanstaan, elk met het aantal tekens dat er minimaal uit moet komen.
fn sets(o: &WachtwoordOpties) -> Vec<(Vec<char>, usize)> {
    let mut sets = Vec::new();
    let mut voeg_toe = |aan: bool, tekens: Vec<char>, minimum: usize| {
        if aan {
            let tekens: Vec<char> = tekens
                .into_iter()
                .filter(|c| !(o.vermijd_dubbelzinnig && DUBBELZINNIG.contains(c)))
                .collect();
            sets.push((tekens, minimum.max(1)));
        }
    };
    voeg_toe(o.hoofdletters, ('A'..='Z').collect(), 1);
    voeg_toe(o.kleine_letters, ('a'..='z').collect(), 1);
    voeg_toe(o.cijfers, ('0'..='9').collect(), o.min_cijfers as usize);
    voeg_toe(
        o.speciaal,
        SPECIALE_TEKENS.to_vec(),
        o.min_speciaal as usize,
    );
    sets
}

/// De lengte die werkelijk gebruikt wordt: minstens 5 en minstens de som van de
/// minima van de sets die aanstaan (elke set telt minimaal één), met een bovengrens van 128.
pub fn effectieve_lengte(o: &WachtwoordOpties) -> u8 {
    let som: usize = sets(o).iter().map(|(_, m)| m).sum();
    (o.lengte as usize)
        .max(MIN_LENGTE as usize)
        .max(som)
        .min(MAX_LENGTE as usize) as u8
}

/// Genereert een wachtwoord met de gegeven instellingen en willekeurbron.
pub fn genereer_wachtwoord<R: Rng + ?Sized>(
    o: &WachtwoordOpties,
    rng: &mut R,
) -> Result<Gegenereerd, GeneratorFout> {
    if o.lengte > MAX_LENGTE {
        return Err(GeneratorFout::OngeldigeLengte);
    }
    let sets = sets(o);
    if sets.is_empty() {
        return Err(GeneratorFout::GeenTekensetAan);
    }
    let lengte = effectieve_lengte(o) as usize;
    let alles: Vec<char> = sets.iter().flat_map(|(t, _)| t.iter().copied()).collect();

    let mut tekens: Vec<char> = Vec::with_capacity(lengte);
    for (set, minimum) in &sets {
        for _ in 0..*minimum {
            tekens.push(set[rng.random_range(0..set.len())]);
        }
    }
    while tekens.len() < lengte {
        tekens.push(alles[rng.random_range(0..alles.len())]);
    }
    tekens.shuffle(rng);

    Ok(Gegenereerd {
        tekst: Zeroizing::new(tekens.iter().collect()),
        modus: Modus::Wachtwoord,
        woorden: None,
        scheidingsteken: None,
        entropie_bits: wachtwoord_bits(lengte, alles.len()),
    })
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
