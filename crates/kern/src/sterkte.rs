// ── Over dit bestand ──────────────────────────────────────
// Doel:           Sterkteberekening in bits voor gegenereerde wachtwoorden, wachtzinnen en eigen tekst, met de niveaus.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Module sterkte aangemaakt: bits, niveaus en schatting van eigen tekst.
// ──────────────────────────────────────────────────────────

//! Sterkte in bits. Gegenereerde wachtwoorden en wachtzinnen rekenen exact;
//! bij eigen tekst is het een schatting op basis van de gebruikte tekensoorten.

/// Het niveau van een wachtwoord, afgeleid van het aantal bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Niveau {
    Zwak,
    Redelijk,
    Sterk,
}

/// Zwak onder de 60 bits, redelijk van 60 tot en met 80, sterk boven de 80.
pub fn niveau(bits: f64) -> Niveau {
    if bits < 60.0 {
        Niveau::Zwak
    } else if bits <= 80.0 {
        Niveau::Redelijk
    } else {
        Niveau::Sterk
    }
}

/// Bits van een gegenereerd wachtwoord: lengte × log₂(aantal mogelijke tekens).
/// `pool` moet minimaal 1 zijn.
pub fn wachtwoord_bits(lengte: usize, pool: usize) -> f64 {
    lengte as f64 * (pool as f64).log2()
}

/// Bits van een wachtzin: woorden × log₂(woorden in de lijst).
/// Het cijfer telt mee als log₂(10 × woorden). `lijst` moet minimaal 1 zijn.
pub fn wachtzin_bits(woorden: usize, lijst: usize, met_cijfer: bool) -> f64 {
    let mut bits = woorden as f64 * (lijst as f64).log2();
    if met_cijfer && woorden > 0 {
        bits += (10.0 * woorden as f64).log2();
    }
    bits
}

/// Schatting van de bits van eigen tekst: lengte × log₂(pool), waarbij de pool
/// de tekensoorten optelt die in de tekst voorkomen (kleine letters 26, hoofdletters 26,
/// cijfers 10, overige ASCII 33, niet-ASCII 100). Lengte is het aantal tekens, geen bytes.
pub fn schatting_bits(tekst: &str) -> f64 {
    let (mut klein, mut hoofd, mut cijfer, mut overig, mut buiten_ascii) =
        (false, false, false, false, false);
    let mut lengte = 0usize;
    for teken in tekst.chars() {
        lengte += 1;
        if teken.is_ascii_lowercase() {
            klein = true;
        } else if teken.is_ascii_uppercase() {
            hoofd = true;
        } else if teken.is_ascii_digit() {
            cijfer = true;
        } else if teken.is_ascii() {
            overig = true;
        } else {
            buiten_ascii = true;
        }
    }
    if lengte == 0 {
        return 0.0;
    }
    let pool = [
        (klein, 26),
        (hoofd, 26),
        (cijfer, 10),
        (overig, 33),
        (buiten_ascii, 100),
    ]
    .iter()
    .filter(|(aanwezig, _)| *aanwezig)
    .map(|(_, grootte)| grootte)
    .sum::<usize>();
    lengte as f64 * (pool as f64).log2()
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
