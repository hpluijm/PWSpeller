// ── Over dit bestand ──────────────────────────────────────
// Doel:           Controleert een Catalogus op volledigheid en op de regels voor woordenlijsten; verzamelt alle fouten.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fixronde 1: exacte sleutels, lege waarden, verweesde blokkeerlijst, en.toml verplicht, lege lijst.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Controles voor alfabetten, omschrijvingen, verwijzingen, interface en woordenlijsten.
// ──────────────────────────────────────────────────────────

use crate::model::{Catalogus, DataFout};

fn fout(bestand: &str, melding: impl Into<String>) -> DataFout {
    DataFout {
        bestand: bestand.to_string(),
        melding: melding.into(),
    }
}

fn leeg_als(f: &mut Vec<DataFout>, bestand: &str, wat: &str, waarde: &str) {
    if waarde.trim().is_empty() {
        f.push(fout(bestand, format!("{wat} is leeg")));
    }
}

fn alleen_a_tot_z(woord: &str) -> bool {
    woord.chars().all(|c| c.is_ascii_lowercase())
}

/// Controleert de catalogus en geeft alle gevonden fouten terug (leeg = in orde).
pub fn controleer(c: &Catalogus) -> Vec<DataFout> {
    let mut f = Vec::new();

    for (code, a) in &c.alfabetten {
        let bestand = format!("alfabetten/{code}.toml");
        leeg_als(&mut f, &bestand, "naam", &a.naam);
        for letter in 'a'..='z' {
            if !a.letters.contains_key(&letter) {
                f.push(fout(&bestand, format!("letter '{letter}' ontbreekt")));
            }
        }
        for (letter, woord) in &a.letters {
            if !letter.is_ascii_lowercase() {
                f.push(fout(
                    &bestand,
                    format!("sleutel '{letter}' in [letters] hoort er niet in (alleen a-z)"),
                ));
            }
            leeg_als(
                &mut f,
                &bestand,
                &format!("codewoord van '{letter}'"),
                woord,
            );
        }
    }

    for (code, o) in &c.omschrijvingen {
        let bestand = format!("omschrijvingen/{code}.toml");
        for cijfer in '0'..='9' {
            if !o.cijfers.contains_key(&cijfer) {
                f.push(fout(
                    &bestand,
                    format!("cijfer '{cijfer}' ontbreekt in [cijfers]"),
                ));
            }
        }
        for (cijfer, tekst) in &o.cijfers {
            if !cijfer.is_ascii_digit() {
                f.push(fout(
                    &bestand,
                    format!("sleutel '{cijfer}' in [cijfers] hoort er niet in (alleen 0-9)"),
                ));
            }
            leeg_als(
                &mut f,
                &bestand,
                &format!("omschrijving van cijfer '{cijfer}'"),
                tekst,
            );
        }
        for b in 32u8..=126 {
            let teken = b as char;
            if !teken.is_ascii_alphanumeric() && !o.tekens.contains_key(&teken) {
                f.push(fout(
                    &bestand,
                    format!("teken '{teken}' ontbreekt in [tekens]"),
                ));
            }
        }
        for (teken, tekst) in &o.tekens {
            if teken.is_ascii_alphanumeric() || !(' '..='~').contains(teken) {
                f.push(fout(&bestand, format!("sleutel '{teken}' in [tekens] hoort er niet in (alleen niet-alfanumerieke ASCII 32-126)")));
            }
            leeg_als(
                &mut f,
                &bestand,
                &format!("omschrijving van teken '{teken}'"),
                tekst,
            );
        }
        let s = &o.sjablonen;
        for (naam, tekst) in [
            ("hoofdletter", &s.hoofdletter),
            ("kleine_letter", &s.kleine_letter),
            ("cijfer", &s.cijfer),
            ("woord", &s.woord),
            ("woord_met_hoofdletter", &s.woord_met_hoofdletter),
            ("onbekend", &s.onbekend),
        ] {
            leeg_als(&mut f, &bestand, &format!("sjabloon '{naam}'"), tekst);
        }
        for (lijst, items) in [
            ("bevat", &o.lastige_klanken.bevat),
            ("eindigt_op", &o.lastige_klanken.eindigt_op),
        ] {
            for item in items {
                leeg_als(
                    &mut f,
                    &bestand,
                    &format!("item in lastige_klanken.{lijst}"),
                    item,
                );
            }
        }
    }

    let mut codes = std::collections::BTreeSet::new();
    for t in &c.spellingstalen {
        if !codes.insert(t.code.as_str()) {
            f.push(fout(
                "spellingstalen.toml",
                format!("code '{}' staat dubbel", t.code),
            ));
        }
        leeg_als(
            &mut f,
            "spellingstalen.toml",
            &format!("naam van taal '{}'", t.code),
            &t.naam,
        );
        let bestand = "spellingstalen.toml";
        if !c.alfabetten.contains_key(&t.alfabet) {
            f.push(fout(
                bestand,
                format!(
                    "taal '{}' verwijst naar onbekend alfabet '{}'",
                    t.code, t.alfabet
                ),
            ));
        }
        if let Some(o) = &t.omschrijvingen
            && !c.omschrijvingen.contains_key(o)
        {
            f.push(fout(
                bestand,
                format!(
                    "taal '{}' verwijst naar onbekende omschrijvingen '{o}'",
                    t.code
                ),
            ));
        }
        if !c.woordenlijsten.contains_key(&t.woordenlijst) {
            f.push(fout(
                bestand,
                format!(
                    "taal '{}' verwijst naar onbekende woordenlijst '{}'",
                    t.code, t.woordenlijst
                ),
            ));
        }
    }

    if !c.interface.is_empty() && !c.interface.contains_key("en") {
        f.push(fout(
            "interface/en.toml",
            "ontbreekt, terwijl andere interfacebestanden er wel zijn",
        ));
    }
    if let Some(en) = c.interface.get("en") {
        for (code, teksten) in c.interface.iter().filter(|(code, _)| *code != "en") {
            for sleutel in en.keys().filter(|s| !teksten.contains_key(*s)) {
                f.push(fout(
                    &format!("interface/{code}.toml"),
                    format!("sleutel \"{sleutel}\" uit en.toml ontbreekt"),
                ));
            }
        }
    }

    for code in c
        .geblokkeerd
        .keys()
        .filter(|k| !c.woordenlijsten.contains_key(*k))
    {
        f.push(fout(
            &format!("woordenlijsten/{code}-geblokkeerd.txt"),
            format!("er is geen bijbehorende woordenlijst woordenlijsten/{code}.txt"),
        ));
    }

    for (code, lijst) in &c.woordenlijsten {
        let bestand = format!("woordenlijsten/{code}.txt");
        let eff = code == "en-eff";
        let blok = c.geblokkeerd.get(code);
        if lijst.woorden.is_empty() {
            f.push(fout(&bestand, "woordenlijst is leeg"));
        }
        let mut gezien = std::collections::BTreeSet::new();
        for woord in &lijst.woorden {
            if !alleen_a_tot_z(woord) {
                f.push(fout(
                    &bestand,
                    format!("woord '{woord}' bevat andere tekens dan a-z"),
                ));
            }
            if !eff && woord.chars().count() < 5 {
                f.push(fout(
                    &bestand,
                    format!("woord '{woord}' is korter dan 5 tekens"),
                ));
            }
            if !gezien.insert(woord.as_str()) {
                f.push(fout(
                    &bestand,
                    format!("woord '{woord}' staat dubbel in de lijst"),
                ));
            }
            if blok.is_some_and(|b| b.contains(&woord.to_lowercase())) {
                f.push(fout(
                    &bestand,
                    format!("woord '{woord}' staat in de blokkeerlijst"),
                ));
            }
        }
    }

    f
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
