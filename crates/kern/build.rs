// ── Over dit bestand ──────────────────────────────────────
// Doel:           Bakt de taaldata uit data/ in de kern en laat de build falen bij onvolledige data.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Build-script toegevoegd.
// ──────────────────────────────────────────────────────────

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR ontbreekt"));
    let data = manifest
        .join("../../data")
        .canonicalize()
        .expect("map data/ niet gevonden");
    println!("cargo:rerun-if-changed={}", data.display());

    let mut paden = Vec::new();
    verzamel(&data, &mut paden);
    paden.sort();

    let mut inhoud = Vec::new();
    for pad in &paden {
        let relatief = pad
            .strip_prefix(&data)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let tekst =
            fs::read_to_string(pad).unwrap_or_else(|e| panic!("{relatief}: kan niet lezen: {e}"));
        inhoud.push((relatief, tekst));
    }

    let bestanden: Vec<(&str, &str)> = inhoud
        .iter()
        .map(|(p, t)| (p.as_str(), t.as_str()))
        .collect();
    let mut fouten = match pwspeller_data::lees_catalogus(&bestanden) {
        Ok(catalogus) => pwspeller_data::controleer(&catalogus),
        Err(fouten) => fouten,
    };
    if !fouten.is_empty() {
        fouten.dedup();
        let regels: Vec<String> = fouten.iter().map(|f| f.to_string()).collect();
        panic!(
            "De taaldata in data/ is onvolledig of onjuist:\n{}",
            regels.join("\n")
        );
    }

    let mut uit = String::from("pub static BESTANDEN: &[(&str, &str)] = &[\n");
    for (pad, (relatief, _)) in paden.iter().zip(&inhoud) {
        uit.push_str(&format!(
            "    ({relatief:?}, include_str!({:?})),\n",
            pad.to_string_lossy()
        ));
    }
    uit.push_str("];\n");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR ontbreekt"));
    fs::write(out_dir.join("bestanden.rs"), uit).expect("bestanden.rs schrijven mislukt");
}

fn verzamel(map: &Path, paden: &mut Vec<PathBuf>) {
    for item in fs::read_dir(map).unwrap_or_else(|e| panic!("{}: {e}", map.display())) {
        let pad = item.expect("map lezen mislukt").path();
        if pad.is_dir() {
            verzamel(&pad, paden);
        } else if pad.extension().is_some_and(|e| e == "toml" || e == "txt") {
            paden.push(pad);
        }
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
