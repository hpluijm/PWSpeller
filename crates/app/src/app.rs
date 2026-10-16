// ── Over dit bestand ──────────────────────────────────────
// Doel:           De app zelf: het PwApp-struct dat de instellingen en de interfacetaal bewaart en het hoofdvenster tekent.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: PwApp aangemaakt met een leeg hoofdvlak.
// ──────────────────────────────────────────────────────────

use crate::instellingen::{Instellingen, interfacetaal};
use eframe::egui;
use std::path::PathBuf;

pub struct PwApp {
    pub instellingen: Instellingen,
    pub instellingenpad: PathBuf,
    pub interfacetaal: String,
}

impl PwApp {
    pub fn nieuw(instellingenpad: PathBuf) -> Self {
        let instellingen = Instellingen::laad(&instellingenpad);
        let interfacetaal = interfacetaal(&instellingen);
        Self {
            instellingen,
            instellingenpad,
            interfacetaal,
        }
    }
}

impl eframe::App for PwApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |_ui| {});
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
