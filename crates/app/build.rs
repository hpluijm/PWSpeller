// ── Over dit bestand ──────────────────────────────────────
// Doel:           Bepaalt het versienummer bij het bouwen: uit de omgeving (CI), anders uit git, anders uit Cargo.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: build-script aangemaakt.
// ──────────────────────────────────────────────────────────

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=PWSPELLER_VERSIE");
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/refs/tags");
    let versie = std::env::var("PWSPELLER_VERSIE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .or_else(uit_git)
        .unwrap_or_else(|| {
            std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION ontbreekt")
        });
    println!("cargo:rustc-env=PWSPELLER_VERSIE={versie}");
}

fn uit_git() -> Option<String> {
    let uit = Command::new("git")
        .args(["describe", "--tags", "--always"])
        .output()
        .ok()?;
    if !uit.status.success() {
        return None;
    }
    let tekst = String::from_utf8(uit.stdout).ok()?.trim().to_owned();
    (!tekst.is_empty()).then_some(tekst)
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
