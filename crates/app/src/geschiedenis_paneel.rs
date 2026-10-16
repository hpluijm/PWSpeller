// ── Over dit bestand ──────────────────────────────────────
// Doel:           Het geschiedenispaneel: gemaskeerde lijst, tonen, kopiëren, opnieuw spellen, wissen, en het wissen na inactiviteit.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17 fixronde 1: wissen na inactiviteit wist ook het huidige resultaat; invoer telt als gebruik.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17: aangemaakt.
// ──────────────────────────────────────────────────────────

use crate::app::PwApp;
use crate::hoofd::Tab;
use chrono::{DateTime, Local};
use eframe::egui::{self, FontFamily, FontId, RichText};
use pwspeller_kern::generator::{Gegenereerd, Modus};
use pwspeller_kern::geschiedenis::ItemInfo;
use pwspeller_kern::talen::{catalogus, interfacetekst};
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

const MASKER: &str = "••••••••";

/// Wat de gebruiker in het paneel koos; het wordt na het tekenen uitgevoerd.
enum Actie {
    Toon(u64),
    Verberg(u64),
    Kopieer(u64),
    Spel(u64),
    Wis,
}

impl PwApp {
    /// Het aantal items waarvan de tekst nu ontsleuteld in beeld staat.
    pub fn aantal_getoond(&self) -> usize {
        self.getoond.len()
    }

    /// Eén stap per beeld: de time-out bijwerken, wissen na inactiviteit en zorgen dat er
    /// opnieuw getekend wordt zolang er iets te wissen valt.
    pub(crate) fn tik_geschiedenis(&mut self, ctx: &egui::Context, nu: Instant) {
        let time_out = self
            .instellingen
            .geschiedenis_minuten
            .map(|m| Duration::from_secs(u64::from(m) * 60));
        self.geschiedenis.zet_time_out(time_out);
        if self.geschiedenis.controleer_inactiviteit(nu) {
            self.getoond.clear();
            // Ook wat nu in beeld staat gaat weg, en de voorleesmodus sluit.
            self.wis_resultaat();
            self.gewist_melding = true;
            log::info!("geschiedenis gewist na inactiviteit");
            ctx.request_repaint();
        }
        // Elke invoer telt als gebruik (muisbewegen alleen niet).
        let invoer = ctx.input(|i| {
            i.pointer.any_pressed()
                || i.events.iter().any(|e| {
                    matches!(
                        e,
                        egui::Event::Key { .. }
                            | egui::Event::Text(_)
                            | egui::Event::PointerButton { .. }
                            | egui::Event::MouseWheel { .. }
                            | egui::Event::Touch { .. }
                            | egui::Event::Copy
                            | egui::Event::Cut
                            | egui::Event::Paste(_)
                    )
                })
        });
        if invoer {
            self.geschiedenis.markeer_gebruik(nu);
        }
        let ids: Vec<u64> = self.geschiedenis.items().iter().map(|i| i.id).collect();
        self.getoond.retain(|(id, _)| ids.contains(id));
        if time_out.is_some() && !ids.is_empty() {
            ctx.request_repaint_after(Duration::from_secs(1));
        }
    }

    /// Tekent het paneel als het open staat; bij sluiten verdwijnen de ontsleutelde items.
    pub(crate) fn teken_geschiedenis(&mut self, ctx: &egui::Context, nu: Instant) {
        if !self.geschiedenis_open {
            self.getoond.clear();
            return;
        }
        let taal = self.interfacetaal.clone();
        let mut open = true;
        let mut actie = None;
        egui::Window::new(interfacetekst(&taal, "geschiedenis.titel"))
            .open(&mut open)
            .default_width(380.0)
            .show(ctx, |ui| actie = self.inhoud(ui, &taal));
        if !open {
            self.geschiedenis_open = false;
            self.getoond.clear();
        }
        if let Some(a) = actie {
            self.voer_uit(a, nu, ctx);
        }
    }

    fn inhoud(&self, ui: &mut egui::Ui, taal: &str) -> Option<Actie> {
        let mut actie = None;
        if self.gewist_melding {
            ui.label(interfacetekst(taal, "geschiedenis.gewist_inactief"));
        }
        let items = self.geschiedenis.items();
        if items.is_empty() {
            ui.label(interfacetekst(taal, "geschiedenis.leeg"));
            return actie;
        }
        egui::ScrollArea::vertical()
            .max_height(420.0)
            .show(ui, |ui| {
                for info in &items {
                    if let Some(a) = self.teken_item(ui, taal, info) {
                        actie = Some(a);
                    }
                    ui.separator();
                }
            });
        if ui
            .button(interfacetekst(taal, "geschiedenis.wis"))
            .clicked()
        {
            actie = Some(Actie::Wis);
        }
        actie
    }

    fn teken_item(&self, ui: &mut egui::Ui, taal: &str, info: &ItemInfo) -> Option<Actie> {
        let mut actie = None;
        let tijd: DateTime<Local> = info.tijdstip.into();
        let modus = match info.modus {
            Modus::Wachtwoord => interfacetekst(taal, "tab.wachtwoord"),
            Modus::Wachtzin => interfacetekst(taal, "tab.wachtzin"),
        };
        ui.label(format!(
            "{}  {}  {}",
            tijd.format("%d-%m-%Y %H:%M:%S"),
            modus,
            info.spellingstaal
        ));
        let getoond = self.getoond.iter().find(|(id, _)| *id == info.id);
        let tekst = getoond.map_or(MASKER, |(_, g)| g.tekst.as_str());
        ui.label(RichText::new(tekst).font(FontId::new(16.0, FontFamily::Monospace)));
        ui.horizontal_wrapped(|ui| {
            if getoond.is_some() {
                if ui
                    .button(interfacetekst(taal, "geschiedenis.verberg"))
                    .clicked()
                {
                    actie = Some(Actie::Verberg(info.id));
                }
            } else if ui
                .button(interfacetekst(taal, "geschiedenis.toon"))
                .clicked()
            {
                actie = Some(Actie::Toon(info.id));
            }
            if ui.button(interfacetekst(taal, "knop.kopieer")).clicked() {
                actie = Some(Actie::Kopieer(info.id));
            }
            if ui
                .button(interfacetekst(taal, "geschiedenis.spel"))
                .clicked()
            {
                actie = Some(Actie::Spel(info.id));
            }
        });
        actie
    }

    fn voer_uit(&mut self, actie: Actie, nu: Instant, ctx: &egui::Context) {
        match actie {
            Actie::Toon(id) => {
                if let Some(g) = self.geschiedenis.ontsleutel(id, nu) {
                    self.getoond.push((id, g));
                }
            }
            Actie::Verberg(id) => self.getoond.retain(|(i, _)| *i != id),
            Actie::Kopieer(id) => {
                if let Some(g) = self.geschiedenis.ontsleutel(id, nu) {
                    self.extra_kopie = Some(g.tekst.clone());
                }
            }
            Actie::Spel(id) => {
                let taal = self
                    .geschiedenis
                    .items()
                    .into_iter()
                    .find(|i| i.id == id)
                    .map(|i| i.spellingstaal);
                if let (Some(g), Some(taal)) = (self.geschiedenis.ontsleutel(id, nu), taal) {
                    self.laad_item(g, &taal);
                }
            }
            Actie::Wis => {
                self.geschiedenis.wis();
                self.getoond.clear();
                self.gewist_melding = false;
                log::info!("geschiedenis gewist op verzoek");
            }
        }
        ctx.request_repaint();
    }

    /// Zet een item uit de geschiedenis als huidig resultaat in het hoofdvenster.
    fn laad_item(&mut self, g: Gegenereerd, spellingstaal: &str) {
        if catalogus()
            .spellingstalen
            .iter()
            .any(|t| t.code == spellingstaal)
        {
            self.instellingen.spellingstaal = spellingstaal.to_owned();
        }
        self.hoofd.fout = None;
        self.hoofd.per_teken = false;
        match g.modus {
            Modus::Wachtwoord => {
                self.hoofd.tab = Tab::Wachtwoord;
                self.hoofd.wachtwoord = Some(g);
            }
            Modus::Wachtzin => {
                self.hoofd.tab = Tab::Wachtzin;
                self.hoofd.wachtzin = Some(g);
            }
        }
    }

    /// Het kopieerverzoek uit het paneel: de ontsleutelde tekst voor één keer, die daarna verdwijnt.
    pub(crate) fn neem_extra_kopie(&mut self) -> Option<Zeroizing<String>> {
        self.extra_kopie.take()
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
