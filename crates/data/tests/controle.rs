// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test het inlezen en controleren van de taalbestanden van pwspeller-data.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Tests voor lees_catalogus en controleer toegevoegd.
// ──────────────────────────────────────────────────────────

use pwspeller_data::{Catalogus, DataFout, controleer, lees_catalogus};

/// Alfabet met a t/m z; letters in `weglaten` ontbreken.
fn alfabet(weglaten: &str) -> String {
    let mut s = String::from("[meta]\ncode = \"nl\"\nnaam = \"Nederlands\"\n\n[letters]\n");
    for l in 'a'..='z' {
        if !weglaten.contains(l) {
            s.push_str(&format!("{l} = \"Woord{l}\"\n"));
        }
    }
    s
}

/// Omschrijvingen met alle tekens; tekens in `weglaten` ontbreken.
fn omschrijvingen(weglaten: &str) -> String {
    let mut s = String::from(
        "[meta]\ncode = \"nl\"\n\n[sjablonen]\nhoofdletter = \"hoofdletter {woord}\"\n\
         kleine_letter = \"kleine letter {woord}\"\ncijfer = \"cijfer {naam}\"\nwoord = \"{woord}\"\n\
         woord_met_hoofdletter = \"{woord}, met hoofdletter\"\nonbekend = \"teken {teken} ({code})\"\n\n[cijfers]\n",
    );
    for c in '0'..='9' {
        s.push_str(&format!("\"{c}\" = \"cijfer{c}\"\n"));
    }
    s.push_str("\n[tekens]\n");
    for b in 32u8..=126 {
        let c = b as char;
        if c.is_ascii_alphanumeric() || weglaten.contains(c) {
            continue;
        }
        let sleutel = match c {
            '"' => "\\\"".to_string(),
            '\\' => "\\\\".to_string(),
            _ => c.to_string(),
        };
        s.push_str(&format!("\"{sleutel}\" = \"teken{b}\"\n"));
    }
    s.push_str("\n[lastige_klanken]\nbevat = [\"ij\"]\neindigt_op = [\"d\", \"t\"]\n");
    s
}

const INTERFACE_EN: &str = "\"knop.genereer\" = \"Generate\"\n\"label.titel\" = \"Title\"\n";
const INTERFACE_NL: &str = "\"knop.genereer\" = \"Genereer\"\n\"label.titel\" = \"Titel\"\n";
const SPELLINGSTALEN: &str = "[[taal]]\ncode = \"nl\"\nnaam = \"Nederlands\"\nalfabet = \"nl\"\n\
                              omschrijvingen = \"nl\"\nwoordenlijst = \"nl\"\n";

fn catalogus(wijzig: impl Fn(&str, String) -> String) -> Catalogus {
    let bestanden: Vec<(&str, String)> = vec![
        ("alfabetten/nl.toml", alfabet("")),
        ("omschrijvingen/nl.toml", omschrijvingen("")),
        ("interface/en.toml", INTERFACE_EN.to_string()),
        ("interface/nl.toml", INTERFACE_NL.to_string()),
        ("woordenlijsten/nl.txt", "appel\nbanaan\n".to_string()),
        ("woordenlijsten/nl-geblokkeerd.txt", "moord\n".to_string()),
        ("spellingstalen.toml", SPELLINGSTALEN.to_string()),
    ];
    let bestanden: Vec<(String, String)> = bestanden
        .into_iter()
        .map(|(p, i)| (p.to_string(), wijzig(p, i)))
        .collect();
    let refs: Vec<(&str, &str)> = bestanden
        .iter()
        .map(|(p, i)| (p.as_str(), i.as_str()))
        .collect();
    lees_catalogus(&refs).unwrap_or_else(|f| panic!("inlezen mislukt: {f:?}"))
}

fn heeft(f: &[DataFout], bestand: &str, deel: &str) -> bool {
    f.iter()
        .any(|x| x.bestand == bestand && x.melding.contains(deel))
}

#[test]
fn alfabet_zonder_q_geeft_fout_met_bestand_en_letter() {
    let c = catalogus(|p, i| {
        if p == "alfabetten/nl.toml" {
            alfabet("q")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "alfabetten/nl.toml", "'q'"), "{f:?}");
}

#[test]
fn omschrijvingen_moeten_ascii_32_tot_126_dekken() {
    let c = catalogus(|p, i| {
        if p == "omschrijvingen/nl.toml" {
            omschrijvingen("~")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "omschrijvingen/nl.toml", "'~'"), "{f:?}");
}

#[test]
fn combinatie_naar_onbekend_alfabet_is_fout() {
    let c = catalogus(|p, i| {
        if p == "spellingstalen.toml" {
            i.replace("alfabet = \"nl\"", "alfabet = \"xx\"")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "spellingstalen.toml", "xx"), "{f:?}");
}

#[test]
fn interface_mist_sleutel_uit_en() {
    let c = catalogus(|p, i| {
        if p == "interface/nl.toml" {
            "\"label.titel\" = \"Titel\"\n".to_string()
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "interface/nl.toml", "knop.genereer"), "{f:?}");
}

#[test]
fn woord_korter_dan_5_is_fout() {
    let c = catalogus(|p, i| {
        if p == "woordenlijsten/nl.txt" {
            format!("{i}kat\n")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "woordenlijsten/nl.txt", "kat"), "{f:?}");
}

#[test]
fn woord_met_hoofdletter_of_accent_is_fout() {
    let c = catalogus(|p, i| {
        if p == "woordenlijsten/nl.txt" {
            format!("{i}Appeltje\ncafeetje\ncafé\n")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "woordenlijsten/nl.txt", "Appeltje"), "{f:?}");
    assert!(heeft(&f, "woordenlijsten/nl.txt", "café"), "{f:?}");
}

#[test]
fn dubbel_woord_is_fout() {
    let c = catalogus(|p, i| {
        if p == "woordenlijsten/nl.txt" {
            format!("{i}appel\n")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "woordenlijsten/nl.txt", "appel"), "{f:?}");
}

#[test]
fn geblokkeerd_woord_in_lijst_is_fout() {
    let c = catalogus(|p, i| {
        if p == "woordenlijsten/nl.txt" {
            format!("{i}moord\n")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "woordenlijsten/nl.txt", "moord"), "{f:?}");
}

#[test]
fn sleutel_van_twee_tekens_is_fout() {
    let bestanden = [(
        "alfabetten/nl.toml",
        "[meta]\ncode = \"nl\"\nnaam = \"N\"\n[letters]\nab = \"x\"\n",
    )];
    let f = lees_catalogus(&bestanden).unwrap_err();
    assert!(heeft(&f, "alfabetten/nl.toml", "ab"), "{f:?}");
}

#[test]
fn en_eff_staat_korte_woorden_toe_maar_niet_dubbel() {
    let bestanden = [("woordenlijsten/en-eff.txt", "zip\nzap\nzip\n")];
    let c = lees_catalogus(&bestanden).unwrap();
    let f = controleer(&c);
    assert!(heeft(&f, "woordenlijsten/en-eff.txt", "zip"), "{f:?}");
    assert!(!heeft(&f, "woordenlijsten/en-eff.txt", "zap"), "{f:?}");
}

#[test]
fn onbekend_pad_en_ongeldige_toml_zijn_fouten() {
    let f = lees_catalogus(&[("rommel/x.bin", ""), ("alfabetten/nl.toml", "[meta")]).unwrap_err();
    assert!(heeft(&f, "rommel/x.bin", ""), "{f:?}");
    assert!(heeft(&f, "alfabetten/nl.toml", ""), "{f:?}");
    assert_eq!(f.len(), 2, "alle fouten worden verzameld");
}

#[test]
fn complete_set_geeft_geen_fouten() {
    let c = catalogus(|_, i| i);
    let f = controleer(&c);
    assert!(f.is_empty(), "{f:?}");
    assert_eq!(c.alfabetten["nl"].letters[&'a'], "Woorda");
}

#[test]
fn dataklacht_toont_bestand_en_melding() {
    let f = DataFout {
        bestand: "a.toml".into(),
        melding: "kapot".into(),
    };
    assert_eq!(f.to_string(), "a.toml: kapot");
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
