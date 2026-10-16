// ── Over dit bestand ──────────────────────────────────────
// Doel:           Zet tekst om in regels om telefonisch voor te lezen: per teken of per woord van een wachtzin.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Module spelling aangemaakt.
// ──────────────────────────────────────────────────────────

//! Spellen: tekst omzetten in regels `(positie, fragment, omschrijving, soort, lastig)`.

use crate::talen::{Alfabet, Omschrijvingen};

/// De soort van een gespeld teken of woord.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Soort {
    Hoofdletter,
    KleineLetter,
    Cijfer,
    Speciaal,
    Spatie,
    Woord,
    Onbekend,
}

/// Eén regel van de spelling.
#[derive(Debug, Clone, PartialEq)]
pub struct Regel {
    /// 1-based, in tekens (chars), doorlopend over woorden, cijfers en scheidingstekens.
    pub positie: usize,
    pub fragment: String,
    pub omschrijving: String,
    pub soort: Soort,
    pub lastig: bool,
}

/// Spelt tekst met een alfabet en de omschrijvingen van één taal.
pub struct Speller<'a> {
    alfabet: &'a Alfabet,
    omschrijvingen: &'a Omschrijvingen,
}

/// Vult `{plaatshouder}`-sleutels in één doorgang in; de ingevulde waarden worden niet opnieuw bekeken.
fn vul(sjabloon: &str, waarden: &[(&str, &str)]) -> String {
    let mut uit = String::with_capacity(sjabloon.len() + 16);
    let mut rest = sjabloon;
    while let Some(begin) = rest.find('{') {
        uit.push_str(&rest[..begin]);
        let na = &rest[begin..];
        let gevonden = waarden.iter().find(|(sleutel, _)| {
            na.strip_prefix('{')
                .and_then(|r| r.strip_prefix(*sleutel))
                .is_some_and(|r| r.starts_with('}'))
        });
        match gevonden {
            Some((sleutel, waarde)) => {
                uit.push_str(waarde);
                rest = &na[sleutel.len() + 2..];
            }
            None => {
                uit.push('{');
                rest = &na[1..];
            }
        }
    }
    uit.push_str(rest);
    uit
}

impl<'a> Speller<'a> {
    pub fn nieuw(alfabet: &'a Alfabet, omschrijvingen: &'a Omschrijvingen) -> Self {
        Self {
            alfabet,
            omschrijvingen,
        }
    }

    /// Spelt elk teken van `tekst` als eigen regel.
    pub fn per_teken(&self, tekst: &str) -> Vec<Regel> {
        tekst
            .chars()
            .enumerate()
            .map(|(i, c)| self.spel_teken(c, i + 1, false))
            .collect()
    }

    /// Spelt een wachtzin per woord, met `scheidingsteken` (per teken gespeld) tussen de woorden.
    pub fn wachtzin(&self, woorden: &[String], scheidingsteken: &str) -> Vec<Regel> {
        let mut regels = Vec::new();
        let mut positie = 1;
        for (i, woord) in woorden.iter().enumerate() {
            if i > 0 {
                for c in scheidingsteken.chars() {
                    regels.push(self.spel_teken(c, positie, false));
                    positie += 1;
                }
            }
            let (stam, cijfer) = splits_eindcijfer(woord);
            if !stam.is_empty() {
                if self.stam_is_lastig(stam) {
                    for (j, c) in stam.chars().enumerate() {
                        regels.push(self.spel_teken(c, positie + j, true));
                    }
                } else {
                    regels.push(self.woordregel(stam, positie));
                }
                positie += stam.chars().count();
            }
            if let Some(c) = cijfer {
                regels.push(self.spel_teken(c, positie, false));
                positie += 1;
            }
        }
        regels
    }

    /// Bevat het woord (zonder eindcijfer) een lastige klank, of eindigt het erop?
    pub fn is_lastig(&self, woord: &str) -> bool {
        self.stam_is_lastig(splits_eindcijfer(woord).0)
    }

    fn stam_is_lastig(&self, stam: &str) -> bool {
        let klein = stam.to_lowercase();
        let k = &self.omschrijvingen.lastige_klanken;
        k.bevat
            .iter()
            .any(|p| !p.is_empty() && klein.contains(p.as_str()))
            || k.eindigt_op
                .iter()
                .any(|p| !p.is_empty() && klein.ends_with(p.as_str()))
    }

    fn woordregel(&self, stam: &str, positie: usize) -> Regel {
        let s = &self.omschrijvingen.sjablonen;
        let begint_hoofdletter = stam.chars().next().is_some_and(char::is_uppercase);
        let sjabloon = if begint_hoofdletter {
            &s.woord_met_hoofdletter
        } else {
            &s.woord
        };
        Regel {
            positie,
            fragment: stam.to_string(),
            omschrijving: vul(sjabloon, &[("woord", stam)]),
            soort: Soort::Woord,
            lastig: false,
        }
    }

    /// Het codewoord van een letter, als die (in kleine letters) in het alfabet staat.
    fn codewoord(&self, c: char) -> Option<&'a str> {
        let mut klein = c.to_lowercase();
        let k = klein.next()?;
        if klein.next().is_some() {
            return None;
        }
        self.alfabet.letters.get(&k).map(String::as_str)
    }

    fn spel_teken(&self, c: char, positie: usize, lastig: bool) -> Regel {
        let o = self.omschrijvingen;
        let s = &o.sjablonen;
        let (omschrijving, soort) = if c.is_uppercase() && self.codewoord(c).is_some() {
            let woord = self.codewoord(c).unwrap_or_default();
            (vul(&s.hoofdletter, &[("woord", woord)]), Soort::Hoofdletter)
        } else if c.is_lowercase() && self.codewoord(c).is_some() {
            let woord = self.codewoord(c).unwrap_or_default();
            (
                vul(&s.kleine_letter, &[("woord", woord)]),
                Soort::KleineLetter,
            )
        } else if let Some(naam) = o.cijfers.get(&c) {
            (vul(&s.cijfer, &[("naam", naam)]), Soort::Cijfer)
        } else if let Some(naam) = o.tekens.get(&c) {
            let soort = if c == ' ' {
                Soort::Spatie
            } else {
                Soort::Speciaal
            };
            (naam.clone(), soort)
        } else {
            let teken = c.to_string();
            let code = format!("U+{:04X}", c as u32);
            (
                vul(&s.onbekend, &[("teken", &teken), ("code", &code)]),
                Soort::Onbekend,
            )
        };
        Regel {
            positie,
            fragment: c.to_string(),
            omschrijving,
            soort,
            lastig,
        }
    }
}

/// Haalt één cijfer (0-9) aan het eind los.
fn splits_eindcijfer(woord: &str) -> (&str, Option<char>) {
    match woord.chars().next_back() {
        Some(c) if c.is_ascii_digit() => (&woord[..woord.len() - 1], Some(c)),
        _ => (woord, None),
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
