// ── Over dit bestand ──────────────────────────────────────
// Doel:           Startpunt van het programma PWSpeller: logboek, venster, thema en lettertype.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13 fixronde 1: instellingen één keer geladen en doorgegeven.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13: de app start met het splashscreen via PwApp::nieuw(Some(cc), nu).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12 fixronde 1: fouten bij start van logboek en venster ook naar pwspeller-fout.log.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: venster, logboek en thema gekoppeld; gebruikt nu de bibliotheek.
// ──────────────────────────────────────────────────────────

#![windows_subsystem = "windows"]

use eframe::egui;
use pwspeller::app::PwApp;
use pwspeller::instellingen::{Instellingen, instellingenpad, interfacetaal, logboekpad};
use pwspeller::logboek;
use pwspeller_kern::talen::interfacetekst;
use std::time::Instant;

fn main() -> eframe::Result {
    if let Err(e) = logboek::start_logboek(&logboekpad()) {
        meld_fout(&format!("logboek niet gestart: {e}"));
    }
    let pad = instellingenpad();
    let instellingen = Instellingen::laad(&pad);
    let taal = interfacetaal(&instellingen);
    log::info!("PWSpeller {} gestart", pwspeller::instellingen::VERSIE);
    let titel = interfacetekst(&taal, "app.titel").to_owned();
    let opties = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(titel.clone())
            .with_inner_size([560.0, 640.0])
            .with_min_inner_size([480.0, 560.0]),
        ..Default::default()
    };
    let resultaat = eframe::run_native(
        &titel,
        opties,
        Box::new(move |cc| {
            Ok(Box::new(PwApp::nieuw(
                Some(cc),
                Instant::now(),
                instellingen,
                pad,
            )))
        }),
    );
    if let Err(e) = &resultaat {
        log::error!("venster gestopt met een fout: {e}");
        meld_fout(&format!("venster gestopt met een fout: {e}"));
    }
    resultaat
}

/// Onder `windows_subsystem = "windows"` is de console onzichtbaar; schrijf een fout daarom
/// ook naar een bestand in de tijdelijke map.
fn meld_fout(tekst: &str) {
    eprintln!("{tekst}");
    let pad = std::env::temp_dir().join("pwspeller-fout.log");
    let _ = std::fs::write(&pad, format!("{tekst}\n"));
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
