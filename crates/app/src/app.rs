// ── Over dit bestand ──────────────────────────────────────
// Doel:           De app zelf: het PwApp-struct dat de instellingen en de interfacetaal bewaart en het hoofdvenster tekent.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13 fixronde 1: nieuw() krijgt de geladen instellingen; logo vrijgegeven na de splash; tijdelijk titel in het hoofdvenster.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13: fase Splash/Hoofd en teken(); nieuw() kreeg een CreationContext en een tijdstip.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: PwApp aangemaakt met een leeg hoofdvlak.
// ──────────────────────────────────────────────────────────

use crate::instellingen::{Instellingen, interfacetaal};
use crate::splash::{self, Fase, volgende_fase};
use crate::thema;
use eframe::egui;
use pwspeller_kern::talen::interfacetekst;
use std::path::PathBuf;
use std::time::Instant;

pub struct PwApp {
    pub instellingen: Instellingen,
    pub instellingenpad: PathBuf,
    pub interfacetaal: String,
    pub fase: Fase,
    logo: Option<egui::TextureHandle>,
    logo_geprobeerd: bool,
}

impl PwApp {
    /// Maakt de app. Met een `CreationContext` (de echte start) worden ook thema en lettertype gezet.
    pub fn nieuw(
        cc: Option<&eframe::CreationContext>,
        nu: Instant,
        instellingen: Instellingen,
        instellingenpad: PathBuf,
    ) -> Self {
        if let Some(cc) = cc {
            thema::zet_thema(&cc.egui_ctx);
        }
        Self::van(instellingen, instellingenpad, nu)
    }

    /// App voor tests: standaardinstellingen, Nederlandse interface en een pad dat nooit beschreven wordt.
    pub fn voor_test(nu: Instant) -> Self {
        let instellingen = Instellingen {
            interfacetaal: Some("nl".to_string()),
            ..Instellingen::default()
        };
        Self::van(
            instellingen,
            std::env::temp_dir().join("pwspeller-test-nooit-schrijven.toml"),
            nu,
        )
    }

    fn van(instellingen: Instellingen, instellingenpad: PathBuf, nu: Instant) -> Self {
        let interfacetaal = interfacetaal(&instellingen);
        Self {
            instellingen,
            instellingenpad,
            interfacetaal,
            fase: Fase::Splash { start: nu },
            logo: None,
            logo_geprobeerd: false,
        }
    }

    /// Zet de logo-textuur klaar (of laat hem weg als het laden mislukt) en tekent het splashscreen.
    pub fn zet_logo(&mut self, logo: Option<egui::TextureHandle>) {
        self.logo = logo;
        self.logo_geprobeerd = true;
    }

    /// Tekent één beeld: het splashscreen of het hoofdvenster.
    pub fn teken(&mut self, ui: &mut egui::Ui, nu: Instant) {
        self.fase = volgende_fase(&self.fase, nu);
        match self.fase {
            Fase::Splash { start } => {
                if !self.logo_geprobeerd {
                    let logo = splash::laad_logo(ui.ctx());
                    self.zet_logo(logo);
                }
                splash::teken(ui, &self.interfacetaal, self.logo.as_ref(), start, nu);
            }
            Fase::Hoofd => {
                // De textuur is niet meer nodig.
                self.logo = None;
                // Tijdelijk: Taak 14 vervangt dit door de echte inhoud.
                let titel = interfacetekst(&self.interfacetaal, "app.titel");
                egui::CentralPanel::default().show(ui, |ui| {
                    ui.heading(titel);
                });
            }
        }
    }
}

impl eframe::App for PwApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.teken(ui, Instant::now());
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
