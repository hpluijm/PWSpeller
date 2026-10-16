// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test dat de echte bestanden in data/ compleet en geldig zijn.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 5: tests voor woordenlijsten, blokkeerlijsten en de zes spellingstalen.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Test op de echte databestanden aangemaakt (Taak 4).
// ──────────────────────────────────────────────────────────

use std::fs;
use std::path::{Path, PathBuf};

use pwspeller_data::{Catalogus, controleer, lees_catalogus};

fn data_map() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data")
}

/// Alle bestanden onder data/ als (pad met /, inhoud).
fn lees_alles(map: &Path, voorvoegsel: &str, uit: &mut Vec<(String, String)>) {
    let Ok(ingang) = fs::read_dir(map) else { return };
    for e in ingang.flatten() {
        let naam = e.file_name().to_string_lossy().into_owned();
        let pad = if voorvoegsel.is_empty() { naam } else { format!("{voorvoegsel}/{naam}") };
        if e.path().is_dir() {
            lees_alles(&e.path(), &pad, uit);
        } else if pad.ends_with(".toml") || pad.ends_with(".txt") {
            uit.push((pad, fs::read_to_string(e.path()).expect("bestand leesbaar")));
        }
    }
}

fn echte_catalogus() -> Catalogus {
    let mut bestanden = Vec::new();
    lees_alles(&data_map(), "", &mut bestanden);
    let refs: Vec<(&str, &str)> = bestanden.iter().map(|(p, i)| (p.as_str(), i.as_str())).collect();
    lees_catalogus(&refs).unwrap_or_else(|f| {
        panic!("{}", f.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"))
    })
}

#[test]
fn echte_data_is_volledig() {
    let c = echte_catalogus();
    let fouten = controleer(&c);
    assert!(
        fouten.is_empty(),
        "{}",
        fouten.iter().map(|f| f.to_string()).collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn alle_alfabetten_omschrijvingen_en_interfaces_zijn_er() {
    let c = echte_catalogus();
    for a in ["nl", "vl", "fr", "de", "navo"] {
        assert!(c.alfabetten.contains_key(a), "alfabet {a} ontbreekt");
    }
    for o in ["nl", "en", "fr", "de"] {
        assert!(c.omschrijvingen.contains_key(o), "omschrijvingen {o} ontbreken");
    }
    for i in ["nl", "en"] {
        assert!(c.interface.contains_key(i), "interface {i} ontbreekt");
    }
}

#[test]
fn nederlands_a_is_anna_en_navo_a_is_alfa() {
    let c = echte_catalogus();
    assert_eq!(c.alfabetten["nl"].letters[&'a'], "Anna");
    assert_eq!(c.alfabetten["navo"].letters[&'a'], "Alfa");
    assert_eq!(c.alfabetten["de"].letters[&'c'], "Chemnitz");
    assert_eq!(c.alfabetten["vl"].letters[&'d'], "Desiré");
}

#[test]
fn leestekens_met_bijzondere_escaping_zijn_aanwezig() {
    let c = echte_catalogus();
    let t = &c.omschrijvingen["nl"].tekens;
    for teken in [' ', '"', '\\', '\''] {
        assert!(t.contains_key(&teken), "teken {teken:?} ontbreekt");
    }
}

#[test]
fn eff_lijst_zonder_koppeltekens_en_niet_leeg() {
    let c = echte_catalogus();
    let eff = &c.woordenlijsten["en-eff"];
    assert!(eff.woorden.len() > 7000);
    assert!(eff.woorden.iter().all(|w| !w.contains('-')));
}

#[test]
fn nl_lijst_bevat_geen_geblokkeerde_woorden() {
    let c = echte_catalogus();
    let nl = &c.woordenlijsten["nl"];
    for w in ["moord", "porno", "schijt"] {
        assert!(!nl.woorden.iter().any(|x| x == w), "{w} staat in nl");
        assert!(c.geblokkeerd["nl"].contains(w), "{w} staat niet in de blokkeerlijst");
    }
}

#[test]
fn nl_lijst_heeft_minstens_7500_woorden() {
    let c = echte_catalogus();
    assert!(c.woordenlijsten["nl"].woorden.len() >= 7500);
}

#[test]
fn fr_en_de_lijsten_hebben_minstens_5500_woorden() {
    let c = echte_catalogus();
    for code in ["fr", "de"] {
        assert!(c.woordenlijsten[code].woorden.len() >= 5500, "{code} te klein");
        assert!(c.geblokkeerd.contains_key(code), "{code}-geblokkeerd ontbreekt");
    }
}

#[test]
fn frans_en_duits_gebruiken_eigen_woordenlijst() {
    let c = echte_catalogus();
    for code in ["fr", "de"] {
        let t = c.spellingstalen.iter().find(|t| t.code == code).expect("spellingstaal");
        assert_eq!(t.woordenlijst, code);
        assert_eq!(t.omschrijvingen.as_deref(), Some(code));
    }
}

#[test]
fn zes_spellingstalen_in_vaste_volgorde() {
    let c = echte_catalogus();
    let codes: Vec<&str> = c.spellingstalen.iter().map(|t| t.code.as_str()).collect();
    assert_eq!(codes, ["nl", "vl", "fr", "de", "en", "navo"]);
    let navo = c.spellingstalen.last().unwrap();
    assert_eq!(navo.omschrijvingen, None);
    assert_eq!(navo.woordenlijst, "en-eff");
}
