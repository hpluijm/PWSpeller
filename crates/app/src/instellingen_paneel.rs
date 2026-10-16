// ── Over dit bestand ──────────────────────────────────────
// Doel:           Het instellingenpaneel: interfacetaal, klembord wissen en de tijd voor de geschiedenis.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17: aangemaakt.
// ──────────────────────────────────────────────────────────

use crate::app::PwApp;
use crate::instellingen::interfacetaal;
use eframe::egui::{self, DragValue};
use pwspeller_kern::talen::{catalogus, interfacetekst};

/// Standaardwaarde van de geschiedenistijd als het "alleen bij sluiten" weer uitgaat.
const STANDAARD_MINUTEN: u32 = 15;

fn taalnaam(code: &str) -> String {
    match code {
        "nl" => "Nederlands".to_owned(),
        "en" => "English".to_owned(),
        andere => andere.to_uppercase(),
    }
}

impl PwApp {
    /// Tekent het instellingenpaneel als het open staat. Wijzigen bewaart via het hoofdvenster.
    pub(crate) fn teken_instellingen(&mut self, ctx: &egui::Context) {
        if !self.instellingen_open {
            return;
        }
        let taal = self.interfacetaal.clone();
        let mut open = true;
        egui::Window::new(interfacetekst(&taal, "instellingen.titel"))
            .open(&mut open)
            .default_width(360.0)
            .show(ctx, |ui| self.inhoud_instellingen(ui, &taal));
        self.instellingen_open = open;
        // De interfacetaal volgt de keuze; "systeem" vraagt het besturingssysteem.
        let nieuw = interfacetaal(&self.instellingen);
        if nieuw != self.interfacetaal {
            self.interfacetaal = nieuw;
            ctx.request_repaint();
        }
    }

    fn inhoud_instellingen(&mut self, ui: &mut egui::Ui, taal: &str) {
        let i = &mut self.instellingen;
        let huidig = i.interfacetaal.as_deref().map_or_else(
            || interfacetekst(taal, "instellingen.systeem").to_owned(),
            taalnaam,
        );
        egui::ComboBox::from_label(interfacetekst(taal, "instellingen.interfacetaal"))
            .selected_text(huidig)
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut i.interfacetaal,
                    None,
                    interfacetekst(taal, "instellingen.systeem"),
                );
                let mut codes: Vec<&String> = catalogus().interface.keys().collect();
                codes.sort();
                for code in codes {
                    ui.selectable_value(&mut i.interfacetaal, Some(code.clone()), taalnaam(code));
                }
            });
        ui.separator();
        ui.checkbox(
            &mut i.klembord_wissen,
            interfacetekst(taal, "instellingen.klembord_wissen"),
        );
        ui.add_enabled_ui(i.klembord_wissen, |ui| {
            ui.horizontal(|ui| {
                ui.label(interfacetekst(taal, "instellingen.klembord_seconden"));
                ui.add(DragValue::new(&mut i.klembord_seconden).range(5..=300));
            });
        });
        ui.separator();
        ui.label(interfacetekst(taal, "instellingen.geschiedenis_minuten"));
        let mut alleen_bij_sluiten = i.geschiedenis_minuten.is_none();
        if ui
            .checkbox(
                &mut alleen_bij_sluiten,
                interfacetekst(taal, "instellingen.alleen_bij_sluiten"),
            )
            .changed()
        {
            i.geschiedenis_minuten = if alleen_bij_sluiten {
                None
            } else {
                Some(STANDAARD_MINUTEN)
            };
        }
        if let Some(minuten) = i.geschiedenis_minuten.as_mut() {
            ui.add(DragValue::new(minuten).range(1..=60));
        }
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
