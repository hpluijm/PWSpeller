// ── Over dit bestand ──────────────────────────────────────
// Doel:           Startpunt van het programma PWSpeller: logboek, venster, thema en lettertype.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: venster, logboek en thema gekoppeld; gebruikt nu de bibliotheek.
// ──────────────────────────────────────────────────────────

#![windows_subsystem = "windows"]

use eframe::egui;
use pwspeller::app::PwApp;
use pwspeller::instellingen::{instellingenpad, logboekpad};
use pwspeller::{logboek, thema};
use pwspeller_kern::talen::interfacetekst;

fn main() -> eframe::Result {
    if let Err(e) = logboek::start_logboek(&logboekpad()) {
        eprintln!("logboek niet gestart: {e}");
    }
    let app = PwApp::nieuw(instellingenpad());
    log::info!("PWSpeller {} gestart", pwspeller::instellingen::VERSIE);
    let titel = interfacetekst(&app.interfacetaal, "app.titel").to_owned();
    let opties = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(titel.clone())
            .with_inner_size([560.0, 640.0])
            .with_min_inner_size([480.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        &titel,
        opties,
        Box::new(move |cc| {
            thema::zet_thema(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
