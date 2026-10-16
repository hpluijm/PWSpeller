// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test die bewaakt dat elke historieregel in het blok Over dit bestand een geldig merkteken heeft.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Geneste if samengevoegd (clippy).
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Kopbloktest toegevoegd.
// ──────────────────────────────────────────────────────────

use std::fs;
use std::path::{Path, PathBuf};

const MERKTEKENS: [&str; 4] = [
    "[Claude Code]",
    "[Copilot]",
    "[Claude Code, Copilot]",
    "[onbekend]",
];
const MET_WAARDEN: [&str; 4] = ["Claude Code", "Copilot", "Claude Code, Copilot", "onbekend"];

#[test]
fn elke_historieregel_heeft_een_merkteken() {
    let fouten = controleer_repo(&repo_root());
    assert!(
        fouten.is_empty(),
        "Historieregels zonder geldig merkteken:\n{}",
        fouten.join("\n")
    );
}

#[test]
fn regel_zonder_merkteken_wordt_gevonden() {
    let fouten = controleer_tekst(
        "x.rs",
        "// Over dit bestand\n// 2026-10-16  Hans (Petisys)\n",
    );
    assert_eq!(fouten.len(), 1);
}

#[test]
fn regel_met_merkteken_en_tabelrij_slagen() {
    assert!(
        controleer_tekst(
            "x.rs",
            "// Over dit bestand\n// 2026-10-16  Hans (Petisys)  [Copilot]\n"
        )
        .is_empty()
    );
    let tabel = "## Over dit bestand\n| Wanneer | Wie | Met | Wat |\n|---|---|---|---|\n| 2026-10-16 | Hans | Claude Code, Copilot | x |\n";
    assert!(controleer_tekst("x.md", tabel).is_empty());
    let fout = "## Over dit bestand\n| 2026-10-16 | Hans | iemand | x |\n";
    assert_eq!(controleer_tekst("x.md", fout).len(), 1);
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Begint de tekst met een datum JJJJ-MM-DD?
fn begint_met_datum(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() >= 10
        && b[..10].iter().enumerate().all(|(i, c)| {
            if i == 4 || i == 7 {
                *c == b'-'
            } else {
                c.is_ascii_digit()
            }
        })
}

fn controleer_tekst(pad: &str, inhoud: &str) -> Vec<String> {
    let mut fouten = Vec::new();
    if !inhoud.contains("Over dit bestand") {
        return fouten;
    }
    for (nr, regel) in inhoud.lines().enumerate() {
        let melding = || format!("{pad}:{}: {regel}", nr + 1);
        // Code: `// 2026-10-16 ...` of `# 2026-10-16 ...` moet op een merkteken eindigen.
        let code = regel
            .strip_prefix("// ")
            .or_else(|| regel.strip_prefix("# "));
        if let Some(rest) = code {
            if begint_met_datum(rest) && !MERKTEKENS.iter().any(|m| regel.trim_end().ends_with(m)) {
                fouten.push(melding());
            }
            continue;
        }
        // Markdown: `| 2026-10-16 | Wie | Met | Wat |`, de kolom Met moet geldig zijn.
        if let Some(rest) = regel.strip_prefix('|')
            && begint_met_datum(rest.trim_start())
        {
            let kolommen: Vec<&str> = rest.split('|').map(str::trim).collect();
            if kolommen.get(2).is_none_or(|met| !MET_WAARDEN.contains(met)) {
                fouten.push(melding());
            }
        }
    }
    fouten
}

fn controleer_repo(root: &Path) -> Vec<String> {
    let mut fouten = Vec::new();
    bezoek(root, root, &mut fouten);
    fouten
}

fn bezoek(root: &Path, map: &Path, fouten: &mut Vec<String>) {
    let Ok(items) = fs::read_dir(map) else { return };
    for item in items.flatten() {
        let pad = item.path();
        let naam = item.file_name().to_string_lossy().into_owned();
        if pad.is_dir() {
            if !matches!(naam.as_str(), "target" | ".git" | ".superpowers") {
                bezoek(root, &pad, fouten);
            }
        } else if !naam.ends_with(".txt") {
            // Binaire bestanden (geen geldige UTF-8) vallen vanzelf af.
            if let Ok(inhoud) = fs::read_to_string(&pad) {
                let rel = pad.strip_prefix(root).unwrap_or(&pad).display().to_string();
                fouten.extend(controleer_tekst(&rel, &inhoud));
            }
        }
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
