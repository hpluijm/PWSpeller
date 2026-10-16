// ── Over dit bestand ──────────────────────────────────────
// Doel:           Het hoofdvenster: tabbladen, knoppen, sneltoetsen en het genereren; tekent resultaat, sterkte, opties en spelling.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fix 2: groepjes van vier alleen bij het wachtwoord; eigen tekst (en een wachtzin zonder woorden) als één doorlopende regel.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fix: tabblad Wachtzin toont hele woorden met scheidingsteken in plaats van groepjes van vier.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17 fixronde 1: wis_resultaat.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17: knoppen Geschiedenis en Instellingen, panelen, melding na wissen.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 15: klembordstap per beeld en melding bij mislukken.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14 fixronde 1: spelling en groepjes in een cache die zichzelf wist; Eigen tekst ook in groepjes; sneltoetsen alleen zonder focus op een widget; eigen tekst wissen bij tab-wissel.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14: aangemaakt.
// ──────────────────────────────────────────────────────────

use crate::app::PwApp;
use crate::opties;
use crate::resultaat::{self, SpelCache, groepjes_weergave};
use eframe::egui::{self, Event, FontFamily, FontId, Key, Modifiers, TextEdit};
use pwspeller_kern::generator::{
    Gegenereerd, WachtwoordOpties, effectieve_lengte, genereer_wachtwoord_os, genereer_wachtzin_os,
};
use pwspeller_kern::spelling::{Regel, Speller};
use pwspeller_kern::sterkte::schatting_bits;
use pwspeller_kern::talen::{catalogus, interfacetekst, omschrijvingen_voor};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Instant;
use zeroize::Zeroize;

/// De tabbladen van het hoofdvenster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Tab {
    #[default]
    Wachtwoord,
    Wachtzin,
    EigenTekst,
}

/// Wat het hoofdvenster onthoudt tussen twee beelden.
pub struct HoofdState {
    pub tab: Tab,
    pub wachtwoord: Option<Gegenereerd>,
    pub wachtzin: Option<Gegenereerd>,
    /// De eigen tekst. Wordt met `zeroize()` gewist bij tab-wissel en bij leegmaken. Dat is
    /// beperkt: egui houdt eigen kopieen (galley, undo) die buiten ons bereik liggen, en een
    /// `String` die groeit kopieert zijn buffer; daarom begint hij met ruime capaciteit.
    pub eigen_tekst: String,
    /// Wachtzin per teken spellen in plaats van per woord.
    pub per_teken: bool,
    pub fout: Option<String>,
    pub lengte_opgehoogd: bool,
    /// Gezet door F5 of de knop; de voorleesmodus (Taak 16) neemt het over.
    pub voorlezen_gevraagd: bool,
    kopieer_gevraagd: bool,
    /// Kopiëren of wissen naar het systeemklembord is mislukt (korte melding).
    pub klembord_mislukt: bool,
    /// Spelregels en groepjes van wat in beeld staat; zichzelf wissend bij vervangen.
    pub cache: Option<SpelCache>,
    /// Hoe vaak de cache is opgebouwd (voor de test dat dit niet elk beeld gebeurt).
    pub cache_opbouwen: usize,
    vorige_tab: Tab,
    eigen_was_gevuld: bool,
}

impl Default for HoofdState {
    fn default() -> Self {
        Self {
            tab: Tab::default(),
            wachtwoord: None,
            wachtzin: None,
            eigen_tekst: String::with_capacity(256),
            per_teken: false,
            fout: None,
            lengte_opgehoogd: false,
            voorlezen_gevraagd: false,
            kopieer_gevraagd: false,
            klembord_mislukt: false,
            cache: None,
            cache_opbouwen: 0,
            vorige_tab: Tab::default(),
            eigen_was_gevuld: false,
        }
    }
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

    /// Wist wat in beeld staat: de gegenereerde resultaten, de spelling-cache en de eigen tekst.
    pub fn wis_resultaat(&mut self) {
        let h = &mut self.hoofd;
        h.wachtwoord = None;
        h.wachtzin = None;
        h.cache = None;
        h.eigen_tekst.zeroize();
        self.voorlezen = None;
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
            && let Some(scheiding) = g.scheidingsteken.as_deref()
        {
            return speller.wachtzin(woorden, scheiding);
        }
        speller.per_teken(tekst)
    }

    /// Op het tabblad Wachtzin: de woorden en het scheidingsteken van de huidige wachtzin.
    fn wachtzin_delen(&self) -> Option<(&[String], &str)> {
        let h = &self.hoofd;
        if h.tab != Tab::Wachtzin {
            return None;
        }
        let g = h.wachtzin.as_ref()?;
        Some((
            g.woorden.as_ref()?.as_slice(),
            g.scheidingsteken.as_deref()?,
        ))
    }

    fn spellingstaal(&self) -> &'static pwspeller_kern::talen::Spellingstaal {
        let talen = &catalogus().spellingstalen;
        talen
            .iter()
            .find(|t| t.code == self.instellingen.spellingstaal)
            .or_else(|| talen.first())
            .expect("de catalogus bevat minstens één spellingstaal")
    }

    /// Bouwt spelregels en groepjes opnieuw op als wat in beeld staat veranderd is: andere tab,
    /// tekst, spellingstaal, interfacetaal of spelwijze. Anders blijft de cache staan.
    fn ververs_cache(&mut self) {
        let h = &self.hoofd;
        let Some(tekst) = self.huidige_tekst() else {
            self.hoofd.cache = None;
            return;
        };
        let mut hasher = DefaultHasher::new();
        tekst.hash(&mut hasher);
        let sleutel = (
            h.tab,
            h.per_teken,
            self.instellingen.spellingstaal.clone(),
            self.interfacetaal.clone(),
            hasher.finish(),
        );
        if h.cache.as_ref().is_some_and(|c| c.sleutel == sleutel) {
            return;
        }
        let stukken = match self.wachtzin_delen() {
            Some((woorden, scheiding)) => resultaat::woord_tokens(woorden, scheiding),
            None if h.tab == Tab::Wachtwoord => groepjes_weergave(tekst),
            None => vec![(1, resultaat::doorlopend_weergave(tekst))],
        };
        let nieuw = SpelCache::nieuw(sleutel, stukken, self.spelregels());
        self.hoofd.cache = Some(nieuw);
        self.hoofd.cache_opbouwen += 1;
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
                self.gewist_melding = false;
                self.hoofd.fout = None;
                match g.modus {
                    pwspeller_kern::generator::Modus::Wachtwoord => self.hoofd.wachtwoord = Some(g),
                    pwspeller_kern::generator::Modus::Wachtzin => self.hoofd.wachtzin = Some(g),
                }
            }
            Err(e) => self.hoofd.fout = Some(e.to_string()),
        }
    }

    /// Leest de sneltoetsen, maar alleen als geen enkel widget focus heeft.
    fn sneltoetsen(&mut self, ctx: &egui::Context) -> bool {
        // Heeft een widget focus (tekstveld, maar ook een knop via Tab), dan is Enter voor dat widget.
        if ctx.egui_wants_keyboard_input() || ctx.memory(|m| m.focused().is_some()) {
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
        let voor = self.instellingen.wachtwoord;
        egui::Panel::top("tabbladen").show(ui, |ui| self.teken_balk(ui, &taal));
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                genereer |= self.teken_midden(ui, &taal);
            });
        });
        self.teken_geschiedenis(ui.ctx(), nu);
        self.teken_instellingen(ui.ctx());
        if self.verhoog_lengte() {
            ui.ctx().request_repaint();
        } else if self.instellingen.wachtwoord != voor {
            self.hoofd.lengte_opgehoogd = false;
        }
        if genereer {
            self.genereer(nu);
            ui.ctx().request_repaint();
        }
        self.wis_eigen_tekst_indien_nodig();
        if let Some(moment) = self.klembord_stap(nu) {
            ui.ctx()
                .request_repaint_after(moment.saturating_duration_since(nu));
        }
        if self.instellingen != self.bewaard {
            self.bewaar_instellingen();
            self.bewaard = self.instellingen.clone();
        }
    }

    /// Wist de eigen tekst als je van het tabblad weggaat of hem leegmaakt.
    fn wis_eigen_tekst_indien_nodig(&mut self) {
        let h = &mut self.hoofd;
        let weg = h.vorige_tab == Tab::EigenTekst && h.tab != Tab::EigenTekst;
        if weg || (h.eigen_tekst.is_empty() && h.eigen_was_gevuld) {
            h.eigen_tekst.zeroize();
        }
        h.vorige_tab = h.tab;
        h.eigen_was_gevuld = !h.eigen_tekst.is_empty();
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
            ui.toggle_value(
                &mut self.geschiedenis_open,
                interfacetekst(taal, "geschiedenis.titel"),
            );
            ui.toggle_value(
                &mut self.instellingen_open,
                interfacetekst(taal, "instellingen.titel"),
            );
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
        }
        self.ververs_cache();
        if let Some(c) = &self.hoofd.cache {
            if tab == Tab::Wachtzin && self.wachtzin_delen().is_some() {
                resultaat::teken_wachtzin(ui, &c.groepen);
            } else if tab == Tab::Wachtwoord {
                resultaat::teken_resultaat(ui, &c.groepen);
            } else {
                resultaat::teken_doorlopend(ui, &c.groepen);
            }
        }
        if self.hoofd.klembord_mislukt {
            ui.label(interfacetekst(taal, "melding.klembord_mislukt"));
        }
        if self.gewist_melding {
            ui.label(interfacetekst(taal, "geschiedenis.gewist_inactief"));
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
        if let Some(c) = &self.hoofd.cache {
            resultaat::teken_spelling(ui, taal, &c.regels);
        }
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
