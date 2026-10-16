// ── Over dit bestand ──────────────────────────────────────
// Doel:           Leest de databestanden (op pad herkend) in een Catalogus en verzamelt alle leesfouten.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fixronde 1: dubbel pad is fout, naam verplicht, blokkeerlijst in kleine letters.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Inlezen van alfabetten, omschrijvingen, woordenlijsten, spellingstalen en interface.
// ──────────────────────────────────────────────────────────

use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use crate::model::{
    Alfabet, Catalogus, DataFout, LastigeKlanken, Omschrijvingen, Sjablonen, Spellingstaal,
    Woordenlijst,
};

#[derive(Deserialize)]
struct MetaCode {
    code: String,
}

#[derive(Deserialize)]
struct Meta {
    code: String,
    naam: String,
}

#[derive(Deserialize)]
struct AlfabetBestand {
    meta: Meta,
    letters: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct OmschrijvingenBestand {
    meta: MetaCode,
    sjablonen: Sjablonen,
    cijfers: BTreeMap<String, String>,
    tekens: BTreeMap<String, String>,
    lastige_klanken: LastigeKlanken,
}

#[derive(Deserialize)]
struct SpellingstalenBestand {
    taal: Vec<Spellingstaal>,
}

fn fout(bestand: &str, melding: impl Into<String>) -> DataFout {
    DataFout {
        bestand: bestand.to_string(),
        melding: melding.into(),
    }
}

/// Zet sleutels van precies één teken om naar `char`; andere sleutels worden fouten.
fn tekensleutels(
    bestand: &str,
    tabel: &str,
    invoer: BTreeMap<String, String>,
    fouten: &mut Vec<DataFout>,
) -> BTreeMap<char, String> {
    let mut uit = BTreeMap::new();
    for (sleutel, waarde) in invoer {
        let mut tekens = sleutel.chars();
        match (tekens.next(), tekens.next()) {
            (Some(c), None) => {
                uit.insert(c, waarde);
            }
            _ => fouten.push(fout(
                bestand,
                format!("[{tabel}] sleutel '{sleutel}' moet precies één teken zijn"),
            )),
        }
    }
    uit
}

/// De code uit `<map>/<code>.<ext>`.
fn code_uit_pad<'a>(pad: &'a str, map: &str, ext: &str) -> Option<&'a str> {
    let rest = pad.strip_prefix(map)?.strip_prefix('/')?;
    let code = rest.strip_suffix(ext)?.strip_suffix('.')?;
    (!code.is_empty() && !code.contains('/')).then_some(code)
}

fn controleer_meta(bestand: &str, meta_code: &str, pad_code: &str, fouten: &mut Vec<DataFout>) {
    if meta_code != pad_code {
        fouten.push(fout(
            bestand,
            format!(
                "code '{meta_code}' in [meta] komt niet overeen met de bestandsnaam '{pad_code}'"
            ),
        ));
    }
}

/// Leest alle bestanden (pad, inhoud) in. Alle fouten worden verzameld.
pub fn lees_catalogus(bestanden: &[(&str, &str)]) -> Result<Catalogus, Vec<DataFout>> {
    let mut c = Catalogus::default();
    let mut fouten = Vec::new();

    let mut gezien = BTreeSet::new();
    for &(pad, inhoud) in bestanden {
        if !gezien.insert(pad) {
            fouten.push(fout(
                pad,
                "bestand staat dubbel in de invoer; de tweede is genegeerd",
            ));
            continue;
        }
        if let Some(code) = code_uit_pad(pad, "alfabetten", "toml") {
            match toml::from_str::<AlfabetBestand>(inhoud) {
                Ok(b) => {
                    controleer_meta(pad, &b.meta.code, code, &mut fouten);
                    let letters = tekensleutels(pad, "letters", b.letters, &mut fouten);
                    c.alfabetten.insert(
                        code.to_string(),
                        Alfabet {
                            code: b.meta.code,
                            naam: b.meta.naam,
                            letters,
                        },
                    );
                }
                Err(e) => fouten.push(fout(pad, e.message())),
            }
        } else if let Some(code) = code_uit_pad(pad, "omschrijvingen", "toml") {
            match toml::from_str::<OmschrijvingenBestand>(inhoud) {
                Ok(b) => {
                    controleer_meta(pad, &b.meta.code, code, &mut fouten);
                    let cijfers = tekensleutels(pad, "cijfers", b.cijfers, &mut fouten);
                    let tekens = tekensleutels(pad, "tekens", b.tekens, &mut fouten);
                    c.omschrijvingen.insert(
                        code.to_string(),
                        Omschrijvingen {
                            code: b.meta.code,
                            sjablonen: b.sjablonen,
                            cijfers,
                            tekens,
                            lastige_klanken: b.lastige_klanken,
                        },
                    );
                }
                Err(e) => fouten.push(fout(pad, e.message())),
            }
        } else if let Some(code) = code_uit_pad(pad, "interface", "toml") {
            match toml::from_str::<BTreeMap<String, String>>(inhoud) {
                Ok(teksten) => {
                    c.interface.insert(code.to_string(), teksten);
                }
                Err(e) => fouten.push(fout(pad, e.message())),
            }
        } else if let Some(code) = code_uit_pad(pad, "woordenlijsten", "txt") {
            let regels = inhoud.lines().map(str::trim).filter(|r| !r.is_empty());
            if let Some(basis) = code.strip_suffix("-geblokkeerd") {
                let set: BTreeSet<String> = regels.map(str::to_lowercase).collect();
                c.geblokkeerd.insert(basis.to_string(), set);
            } else {
                let woorden = regels.map(str::to_string).collect();
                c.woordenlijsten.insert(
                    code.to_string(),
                    Woordenlijst {
                        code: code.to_string(),
                        woorden,
                    },
                );
            }
        } else if pad == "spellingstalen.toml" {
            match toml::from_str::<SpellingstalenBestand>(inhoud) {
                Ok(b) => c.spellingstalen = b.taal,
                Err(e) => fouten.push(fout(pad, e.message())),
            }
        } else {
            fouten.push(fout(
                pad,
                "onbekend pad: dit bestand hoort in geen van de herkende mappen",
            ));
        }
    }

    if fouten.is_empty() {
        Ok(c)
    } else {
        Err(fouten)
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
