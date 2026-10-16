// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test het inlezen en controleren van de taalbestanden van pwspeller-data.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fixronde 1: tests voor exacte sleutels, lege waarden, dubbelen, verweesde blokkeerlijst en en.toml.
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

fn lees_met(wijzig: impl Fn(&str, String) -> String) -> Result<Catalogus, Vec<DataFout>> {
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
    lees_catalogus(&refs)
}

fn catalogus(wijzig: impl Fn(&str, String) -> String) -> Catalogus {
    lees_met(wijzig).unwrap_or_else(|f| panic!("inlezen mislukt: {f:?}"))
}

/// Vervangt `van` door `naar` in het bestand `pad`.
fn pas(
    pad: &'static str,
    van: &'static str,
    naar: &'static str,
) -> impl Fn(&str, String) -> String {
    move |p, i| {
        if p == pad {
            assert!(i.contains(van), "{van:?} komt niet voor in {pad}");
            i.replacen(van, naar, 1)
        } else {
            i
        }
    }
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
            format!("{i}Appeltje\ncafeetje\ncrème\n")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "woordenlijsten/nl.txt", "Appeltje"), "{f:?}");
    assert!(heeft(&f, "woordenlijsten/nl.txt", "crème"), "{f:?}");
    assert!(heeft(&f, "woordenlijsten/nl.txt", "a-z"), "{f:?}");
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

const A: &str = "alfabetten/nl.toml";
const O: &str = "omschrijvingen/nl.toml";

#[test]
fn extra_sleutel_in_letters_is_fout() {
    for sleutel in ["Q", "é"] {
        let c = catalogus(move |p, i| {
            if p == A {
                format!("{i}\"{sleutel}\" = \"x\"\n")
            } else {
                i
            }
        });
        let f = controleer(&c);
        assert!(heeft(&f, A, &format!("'{sleutel}'")), "{sleutel}: {f:?}");
    }
}

#[test]
fn letter_als_cijfersleutel_is_fout() {
    let c = catalogus(pas(O, "[cijfers]\n", "[cijfers]\n\"a\" = \"x\"\n"));
    let f = controleer(&c);
    assert!(heeft(&f, O, "'a'"), "{f:?}");
}

#[test]
fn letter_of_cijfer_in_tekens_is_fout() {
    for sleutel in ["a", "5"] {
        let c = catalogus(move |p, i| {
            if p == O {
                i.replacen(
                    "[tekens]\n",
                    &format!("[tekens]\n\"{sleutel}\" = \"x\"\n"),
                    1,
                )
            } else {
                i
            }
        });
        let f = controleer(&c);
        assert!(heeft(&f, O, &format!("'{sleutel}'")), "{sleutel}: {f:?}");
    }
}

#[test]
fn ontbrekend_cijfer_is_fout() {
    let c = catalogus(pas(O, "\"3\" = \"cijfer3\"\n", ""));
    let f = controleer(&c);
    assert!(heeft(&f, O, "'3'"), "{f:?}");
}

#[test]
fn dubbel_pad_is_fout() {
    let f = lees_catalogus(&[
        ("interface/en.toml", INTERFACE_EN),
        ("interface/en.toml", INTERFACE_EN),
    ])
    .unwrap_err();
    assert!(heeft(&f, "interface/en.toml", "dubbel"), "{f:?}");
}

#[test]
fn dubbele_code_in_spellingstalen_is_fout() {
    let c = catalogus(|p, i| {
        if p == "spellingstalen.toml" {
            format!("{i}\n{i}")
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "spellingstalen.toml", "dubbel"), "{f:?}");
}

#[test]
fn blokkeerlijst_zonder_woordenlijst_is_fout() {
    let f = lees_catalogus(&[("woordenlijsten/xx-geblokkeerd.txt", "moord\n")]).unwrap();
    let f = controleer(&f);
    assert!(
        heeft(&f, "woordenlijsten/xx-geblokkeerd.txt", "woordenlijst"),
        "{f:?}"
    );
}

#[test]
fn lege_waarden_zijn_fout() {
    let gevallen = [
        (A, "a = \"Woorda\"", "a = \"  \""),
        (O, "cijfer = \"cijfer {naam}\"", "cijfer = \"\""),
        (O, "\"7\" = \"cijfer7\"", "\"7\" = \"\""),
        (O, "\"!\" = \"teken33\"", "\"!\" = \" \""),
        (O, "bevat = [\"ij\"]", "bevat = [\"\"]"),
        (O, "eindigt_op = [\"d\", \"t\"]", "eindigt_op = [\" \"]"),
    ];
    for (pad, van, naar) in gevallen {
        let c = catalogus(pas(pad, van, naar));
        let f = controleer(&c);
        assert!(heeft(&f, pad, "leeg"), "{van} -> {naar}: {f:?}");
    }
}

#[test]
fn lege_naam_is_fout_en_ontbrekende_naam_wordt_niet_stil_leeg() {
    let c = catalogus(pas(A, "naam = \"Nederlands\"", "naam = \"\""));
    assert!(heeft(&controleer(&c), A, "leeg"));
    let f = lees_met(pas(A, "naam = \"Nederlands\"\n", "")).unwrap_err();
    assert!(heeft(&f, A, "naam"), "{f:?}");
}

#[test]
fn interface_zonder_en_is_fout() {
    let c = catalogus(|p, i| {
        if p == "interface/en.toml" {
            String::new()
        } else {
            i
        }
    });
    let mut c = c;
    c.interface.remove("en");
    let f = controleer(&c);
    assert!(heeft(&f, "interface/en.toml", "ontbreekt"), "{f:?}");
}

#[test]
fn lege_woordenlijst_is_fout() {
    let c = catalogus(|p, i| {
        if p == "woordenlijsten/nl.txt" {
            "\n\n".to_string()
        } else {
            i
        }
    });
    let f = controleer(&c);
    assert!(heeft(&f, "woordenlijsten/nl.txt", "leeg"), "{f:?}");
}

#[test]
fn blokkeerlijst_is_hoofdletterongevoelig_en_negeert_lege_regels() {
    let c = catalogus(|p, i| match p {
        "woordenlijsten/nl-geblokkeerd.txt" => "\n  \nMoord\n".to_string(),
        "woordenlijsten/nl.txt" => format!("{i}moord\n"),
        _ => i,
    });
    assert!(!c.geblokkeerd["nl"].contains(""));
    let f = controleer(&c);
    assert!(heeft(&f, "woordenlijsten/nl.txt", "moord"), "{f:?}");
}

#[test]
fn verwijzing_naar_onbekende_omschrijving_of_woordenlijst_is_fout() {
    let c = catalogus(pas(
        "spellingstalen.toml",
        "omschrijvingen = \"nl\"",
        "omschrijvingen = \"yy\"",
    ));
    assert!(heeft(&controleer(&c), "spellingstalen.toml", "yy"));
    let c = catalogus(pas(
        "spellingstalen.toml",
        "woordenlijst = \"nl\"",
        "woordenlijst = \"zz\"",
    ));
    assert!(heeft(&controleer(&c), "spellingstalen.toml", "zz"));
}

#[test]
fn meta_code_die_afwijkt_van_bestandsnaam_is_fout() {
    let f = lees_met(pas(A, "code = \"nl\"", "code = \"xx\"")).unwrap_err();
    assert!(heeft(&f, A, "xx"), "{f:?}");
}

#[test]
fn hoofdletter_in_letters_telt_niet_als_kleine_letter() {
    let c = catalogus(pas(A, "q = \"Woordq\"", "Q = \"Woordq\""));
    let f = controleer(&c);
    assert!(heeft(&f, A, "'q'"), "{f:?}");
    assert!(heeft(&f, A, "'Q'"), "{f:?}");
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
