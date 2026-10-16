// ── Over dit bestand ──────────────────────────────────────
// Doel:           Bepaalt het versienummer bij het bouwen: uit de omgeving (CI), anders uit git, anders uit Cargo.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 18: Windows-icoon (assets/app.ico) en programma-info in het .exe via winresource.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12 fixronde 1: ook nieuwe commits op dezelfde branch herberekenen de versie.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: build-script aangemaakt.
// ──────────────────────────────────────────────────────────

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=PWSPELLER_VERSIE");
    // Alleen bestaande paden: een ontbrekend pad laat Cargo bij elke build opnieuw draaien.
    for pad in [
        ".git/HEAD",
        ".git/refs/heads",
        ".git/refs/tags",
        ".git/packed-refs",
    ] {
        if std::path::Path::new("../..").join(pad).exists() {
            println!("cargo:rerun-if-changed=../../{pad}");
        }
    }
    let versie = std::env::var("PWSPELLER_VERSIE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .or_else(uit_git)
        .unwrap_or_else(|| {
            std::env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION ontbreekt")
        });
    println!("cargo:rustc-env=PWSPELLER_VERSIE={versie}");
    windows_resources();
}

/// Zet het icoon en de programma-info in het .exe. Het build-script draait op de host, dus de
/// doelcontrole gaat via `CARGO_CFG_TARGET_OS` (werkt ook bij het bouwen voor ARM64 op x64).
fn windows_resources() {
    println!("cargo:rerun-if-changed=assets/app.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/app.ico");
    res.set("ProductName", "PWSpeller");
    res.set("CompanyName", "Petisys");
    res.set(
        "LegalCopyright",
        "© 2026 Petisys. Alle rechten voorbehouden.",
    );
    if let Err(fout) = res.compile() {
        // Zonder icoon werkt het programma nog; stop de build niet, maar laat het zien.
        println!("cargo:warning=Windows-resources niet gezet: {fout}");
    }
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
