// ── Over dit bestand ──────────────────────────────────────
// Doel:           Wachtwoordgenerator naar het concept van Bitwarden: tekensets, minima, uniforme trekking en schudden.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fixronde 1: minima begrensd, bytes in Zeroizing, genereer_wachtwoord_os, doc bij entropie.
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
const MAX_MINIMUM: u8 = 9;

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
fn sets(o: &WachtwoordOpties) -> Vec<(Vec<u8>, usize)> {
    let mut sets = Vec::new();
    let mut voeg_toe = |aan: bool, tekens: Vec<u8>, minimum: usize| {
        if aan {
            let tekens: Vec<u8> = tekens
                .into_iter()
                .filter(|&b| !(o.vermijd_dubbelzinnig && DUBBELZINNIG.contains(&(b as char))))
                .collect();
            sets.push((tekens, minimum.max(1)));
        }
    };
    voeg_toe(o.hoofdletters, (b'A'..=b'Z').collect(), 1);
    voeg_toe(o.kleine_letters, (b'a'..=b'z').collect(), 1);
    voeg_toe(o.cijfers, (b'0'..=b'9').collect(), o.min_cijfers as usize);
    voeg_toe(
        o.speciaal,
        SPECIALE_TEKENS.iter().map(|&c| c as u8).collect(),
        o.min_speciaal as usize,
    );
    sets
}

/// Som van de minima van de sets die aanstaan (elke set telt minimaal één).
fn nodig(o: &WachtwoordOpties) -> usize {
    sets(o).iter().map(|(_, m)| m).sum()
}

/// De lengte die werkelijk gebruikt wordt: minstens 5 en minstens de som van de
/// minima van de sets die aanstaan (elke set telt minimaal één), met een bovengrens van 128.
/// `genereer_wachtwoord` levert altijd precies zoveel tekens, of een fout als dat niet past.
pub fn effectieve_lengte(o: &WachtwoordOpties) -> u8 {
    (o.lengte as usize)
        .max(MIN_LENGTE as usize)
        .max(nodig(o))
        .min(MAX_LENGTE as usize) as u8
}

/// Genereert een wachtwoord met de willekeurbron van het besturingssysteem.
/// `rand::rng()` is in rand 0.10 een ChaCha12-generator die uit het besturingssysteem
/// geseed wordt en cryptografisch veilig is. Gebruik dit in productie.
pub fn genereer_wachtwoord_os(o: &WachtwoordOpties) -> Result<Gegenereerd, GeneratorFout> {
    genereer_wachtwoord(o, &mut rand::rng())
}

/// Genereert een wachtwoord met een willekeurbron naar keuze: alleen voor tests met vaste seed.
/// Fout `OngeldigeLengte` bij lengte boven 128, een minimum boven 9 of een som van minima boven 128.
#[doc(hidden)]
pub fn genereer_wachtwoord<R: Rng + ?Sized>(
    o: &WachtwoordOpties,
    rng: &mut R,
) -> Result<Gegenereerd, GeneratorFout> {
    if o.lengte > MAX_LENGTE
        || (o.cijfers && o.min_cijfers > MAX_MINIMUM)
        || (o.speciaal && o.min_speciaal > MAX_MINIMUM)
    {
        return Err(GeneratorFout::OngeldigeLengte);
    }
    let sets = sets(o);
    if sets.is_empty() {
        return Err(GeneratorFout::GeenTekensetAan);
    }
    if nodig(o) > MAX_LENGTE as usize {
        return Err(GeneratorFout::OngeldigeLengte);
    }
    let lengte = effectieve_lengte(o) as usize;
    let alles: Vec<u8> = sets.iter().flat_map(|(t, _)| t.iter().copied()).collect();

    // Alle tekens zijn ASCII; de bytes blijven in beschermd geheugen.
    let mut bytes: Zeroizing<Vec<u8>> = Zeroizing::new(Vec::with_capacity(lengte));
    for (set, minimum) in &sets {
        for _ in 0..*minimum {
            bytes.push(set[rng.random_range(0..set.len())]);
        }
    }
    while bytes.len() < lengte {
        bytes.push(alles[rng.random_range(0..alles.len())]);
    }
    bytes.shuffle(rng);

    // De Vec verhuist zonder kopie naar de String; de lege Zeroizing laat niets achter.
    let tekst = String::from_utf8(std::mem::take(&mut *bytes)).expect("alleen ASCII");
    Ok(Gegenereerd {
        tekst: Zeroizing::new(tekst),
        modus: Modus::Wachtwoord,
        woorden: None,
        scheidingsteken: None,
        entropie_bits: wachtwoord_bits(lengte, alles.len()),
    })
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
