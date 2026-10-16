// ── Over dit bestand ──────────────────────────────────────
// Doel:           Het hoofdvenster: tabbladen, knoppen, sneltoetsen en het genereren; tekent resultaat, sterkte, opties en spelling.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14: aangemaakt.
// ──────────────────────────────────────────────────────────

use crate::app::PwApp;
use crate::{opties, resultaat};
use eframe::egui::{self, Event, FontFamily, FontId, Key, Modifiers, TextEdit};
use pwspeller_kern::generator::{
    Gegenereerd, WachtwoordOpties, effectieve_lengte, genereer_wachtwoord_os, genereer_wachtzin_os,
};
use pwspeller_kern::spelling::{Regel, Speller};
use pwspeller_kern::sterkte::schatting_bits;
use pwspeller_kern::talen::{catalogus, interfacetekst, omschrijvingen_voor};
use std::time::Instant;
use zeroize::Zeroize;

/// De tabbladen van het hoofdvenster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Wachtwoord,
    Wachtzin,
    EigenTekst,
}

/// Wat het hoofdvenster onthoudt tussen twee beelden.
#[derive(Default)]
pub struct HoofdState {
    pub tab: Tab,
    pub wachtwoord: Option<Gegenereerd>,
    pub wachtzin: Option<Gegenereerd>,
    pub eigen_tekst: String,
    /// Wachtzin per teken spellen in plaats van per woord.
    pub per_teken: bool,
    pub fout: Option<String>,
    pub lengte_opgehoogd: bool,
    /// Gezet door F5 of de knop; de voorleesmodus (Taak 16) neemt het over.
    pub voorlezen_gevraagd: bool,
    kopieer_gevraagd: bool,
}

impl Drop for HoofdState {
    fn drop(&mut self) {
        self.eigen_tekst.zeroize();
    }
}

impl PwApp {
    /// De tekst die nu in beeld staat: het gegenereerde resultaat of de eigen tekst.
    pub fn huidige_tekst(&self) -> Option<&str> {
        let h = &self.hoofd;
        let tekst = match h.tab {
            Tab::Wachtwoord => h.wachtwoord.as_ref().map(|g| g.tekst.as_str()),
            Tab::Wachtzin => h.wachtzin.as_ref().map(|g| g.tekst.as_str()),
            Tab::EigenTekst => Some(h.eigen_tekst.as_str()),
        };
        tekst.filter(|t| !t.is_empty())
    }

    /// Vraagt om kopiëren (knop of Ctrl/Cmd+C). Taak 15 haalt het verzoek op met `neem_kopieer_verzoek`.
    pub fn kopieer_verzoek(&mut self) {
        if self.huidige_tekst().is_some() {
            self.hoofd.kopieer_gevraagd = true;
        }
    }

    /// Geeft door of er om kopiëren gevraagd is, en wist het verzoek.
    pub fn neem_kopieer_verzoek(&mut self) -> bool {
        std::mem::take(&mut self.hoofd.kopieer_gevraagd)
    }

    /// De spellingregels van wat nu in beeld staat, in de gekozen spellingstaal.
    pub fn spelregels(&self) -> Vec<Regel> {
        let Some(tekst) = self.huidige_tekst() else {
            return Vec::new();
        };
        let taal = self.spellingstaal();
        let c = catalogus();
        let (Some(alfabet), omschrijvingen) = (
            c.alfabetten.get(&taal.alfabet),
            omschrijvingen_voor(taal, &self.interfacetaal),
        ) else {
            return Vec::new();
        };
        let speller = Speller::nieuw(alfabet, omschrijvingen);
        let h = &self.hoofd;
        if h.tab == Tab::Wachtzin
            && !h.per_teken
            && let Some(g) = &h.wachtzin
            && let Some(woorden) = &g.woorden
        {
            let scheiding = g.scheidingsteken.as_deref().unwrap_or("-");
            return speller.wachtzin(woorden, scheiding);
        }
        speller.per_teken(tekst)
    }

    fn spellingstaal(&self) -> &'static pwspeller_kern::talen::Spellingstaal {
        let talen = &catalogus().spellingstalen;
        talen
            .iter()
            .find(|t| t.code == self.instellingen.spellingstaal)
            .or_else(|| talen.first())
            .expect("de catalogus bevat minstens één spellingstaal")
    }

    /// Verhoogt de lengte tot wat de minima vragen; geeft terug of dat gebeurde.
    fn verhoog_lengte(&mut self) -> bool {
        let o: &mut WachtwoordOpties = &mut self.instellingen.wachtwoord;
        let nodig = effectieve_lengte(o);
        if nodig > o.lengte {
            o.lengte = nodig;
            self.hoofd.lengte_opgehoogd = true;
            return true;
        }
        false
    }

    fn genereer(&mut self, nu: Instant) {
        let code = self.instellingen.spellingstaal.clone();
        let resultaat = match self.hoofd.tab {
            Tab::Wachtwoord => genereer_wachtwoord_os(&self.instellingen.wachtwoord),
            Tab::Wachtzin => {
                let c = catalogus();
                let lijst = self
                    .instellingen
                    .woordenlijst
                    .as_deref()
                    .and_then(|l| c.woordenlijsten.get(l))
                    .or_else(|| c.woordenlijsten.get(&self.spellingstaal().woordenlijst));
                match lijst {
                    Some(l) => genereer_wachtzin_os(&self.instellingen.wachtzin, l),
                    None => Err(pwspeller_kern::generator::GeneratorFout::LegeWoordenlijst),
                }
            }
            Tab::EigenTekst => return,
        };
        match resultaat {
            Ok(g) => {
                self.geschiedenis.voeg_toe(&g, &code, nu);
                self.hoofd.fout = None;
                match g.modus {
                    pwspeller_kern::generator::Modus::Wachtwoord => self.hoofd.wachtwoord = Some(g),
                    pwspeller_kern::generator::Modus::Wachtzin => self.hoofd.wachtzin = Some(g),
                }
            }
            Err(e) => self.hoofd.fout = Some(e.to_string()),
        }
    }

    /// Leest de sneltoetsen, maar alleen als geen tekstveld het toetsenbord wil hebben.
    fn sneltoetsen(&mut self, ctx: &egui::Context) -> bool {
        if ctx.egui_wants_keyboard_input() {
            return false;
        }
        let (enter, f5, kopieer) = ctx.input_mut(|i| {
            (
                i.consume_key(Modifiers::NONE, Key::Enter),
                i.consume_key(Modifiers::NONE, Key::F5),
                (i.modifiers.command && i.key_pressed(Key::C))
                    || i.events.iter().any(|e| matches!(e, Event::Copy)),
            )
        });
        if f5 && self.huidige_tekst().is_some() {
            self.hoofd.voorlezen_gevraagd = true;
        }
        if kopieer {
            self.kopieer_verzoek();
        }
        enter && self.hoofd.tab != Tab::EigenTekst
    }

    /// Tekent het hoofdvenster.
    pub fn teken_hoofd(&mut self, ui: &mut egui::Ui, nu: Instant) {
        let taal = self.interfacetaal.clone();
        let mut genereer = self.sneltoetsen(ui.ctx());
        self.verhoog_lengte();
        let voor = self.instellingen.wachtwoord;
        egui::Panel::top("tabbladen").show(ui, |ui| self.teken_balk(ui, &taal));
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                genereer |= self.teken_midden(ui, &taal);
            });
        });
        if genereer {
            self.genereer(nu);
        }
        if self.verhoog_lengte() {
            self.hoofd.lengte_opgehoogd = true;
        } else if self.instellingen.wachtwoord != voor {
            self.hoofd.lengte_opgehoogd = false;
        }
        if self.instellingen != self.bewaard {
            self.bewaar_instellingen();
            self.bewaard = self.instellingen.clone();
        }
    }

    fn bewaar_instellingen(&self) {
        if let Some(pad) = &self.instellingenpad
            && let Err(e) = self.instellingen.bewaar(pad)
        {
            log::warn!("instellingen niet bewaard in {}: {e}", pad.display());
        }
    }

    fn teken_balk(&mut self, ui: &mut egui::Ui, taal: &str) {
        ui.horizontal_wrapped(|ui| {
            for (tab, sleutel) in [
                (Tab::Wachtwoord, "tab.wachtwoord"),
                (Tab::Wachtzin, "tab.wachtzin"),
                (Tab::EigenTekst, "tab.eigen"),
            ] {
                ui.selectable_value(&mut self.hoofd.tab, tab, interfacetekst(taal, sleutel));
            }
            let huidig = self.spellingstaal().naam.clone();
            egui::ComboBox::from_label(interfacetekst(taal, "label.spellingstaal"))
                .selected_text(huidig)
                .show_ui(ui, |ui| {
                    for t in &catalogus().spellingstalen {
                        ui.selectable_value(
                            &mut self.instellingen.spellingstaal,
                            t.code.clone(),
                            &t.naam,
                        );
                    }
                });
        });
    }

    /// Tekent resultaat, knoppen, sterkte, opties en spelling. Geeft terug of er gegenereerd moet worden.
    fn teken_midden(&mut self, ui: &mut egui::Ui, taal: &str) -> bool {
        let mut genereer = false;
        let tab = self.hoofd.tab;
        if tab == Tab::EigenTekst {
            ui.add(
                TextEdit::singleline(&mut self.hoofd.eigen_tekst)
                    .font(FontId::new(22.0, FontFamily::Monospace))
                    .hint_text(interfacetekst(taal, "label.eigen_hint"))
                    .password(false)
                    .desired_width(f32::INFINITY),
            );
        } else if let Some(t) = self.huidige_tekst() {
            resultaat::teken_resultaat(ui, t);
        }
        if let Some(fout) = &self.hoofd.fout {
            ui.colored_label(crate::thema::SPECIAAL_LICHT, fout);
        }
        ui.horizontal_wrapped(|ui| {
            if tab != Tab::EigenTekst {
                genereer = ui.button(interfacetekst(taal, "knop.genereer")).clicked();
            }
            let heeft_tekst = self.huidige_tekst().is_some();
            if ui
                .add_enabled(
                    heeft_tekst,
                    egui::Button::new(interfacetekst(taal, "knop.kopieer")),
                )
                .clicked()
            {
                self.kopieer_verzoek();
            }
            if ui
                .add_enabled(
                    heeft_tekst,
                    egui::Button::new(interfacetekst(taal, "knop.voorlezen")),
                )
                .clicked()
            {
                self.hoofd.voorlezen_gevraagd = true;
            }
            if tab == Tab::Wachtzin {
                let sleutel = if self.hoofd.per_teken {
                    "knop.per_woord"
                } else {
                    "knop.per_teken"
                };
                if ui.button(interfacetekst(taal, sleutel)).clicked() {
                    self.hoofd.per_teken = !self.hoofd.per_teken;
                }
            }
        });
        self.teken_sterkte(ui, taal);
        ui.separator();
        match tab {
            Tab::Wachtwoord => {
                opties::wachtwoord_opties(ui, taal, &mut self.instellingen.wachtwoord);
                if self.hoofd.lengte_opgehoogd {
                    ui.label(interfacetekst(taal, "melding.lengte_opgehoogd"));
                }
            }
            Tab::Wachtzin => {
                let i = &mut self.instellingen;
                opties::wachtzin_opties(ui, taal, &mut i.wachtzin, &mut i.woordenlijst);
            }
            Tab::EigenTekst => {}
        }
        ui.separator();
        ui.heading(interfacetekst(taal, "label.spelling"));
        resultaat::teken_spelling(ui, taal, &self.spelregels());
        genereer
    }

    fn teken_sterkte(&self, ui: &mut egui::Ui, taal: &str) {
        let h = &self.hoofd;
        match h.tab {
            Tab::Wachtwoord | Tab::Wachtzin => {
                let g = if h.tab == Tab::Wachtwoord {
                    &h.wachtwoord
                } else {
                    &h.wachtzin
                };
                if let Some(g) = g {
                    resultaat::teken_sterkte(ui, taal, g.entropie_bits, false);
                }
            }
            Tab::EigenTekst if !h.eigen_tekst.is_empty() => {
                resultaat::teken_sterkte(ui, taal, schatting_bits(&h.eigen_tekst), true);
            }
            Tab::EigenTekst => {}
        }
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
