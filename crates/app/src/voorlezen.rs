// ── Over dit bestand ──────────────────────────────────────
// Doel:           De voorleesmodus: het hele venster toont teken voor teken (of woord voor woord) wat aan de telefoon voorgelezen wordt.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 16 fixronde 1: middenzone scrolt en schaalt mee, bovenste regel wrapt, zuiver zwart op goud.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 16: aangemaakt; Voorlezen met begrensde stappen, markering, toetsen en knoppen, en wissen bij verlaten.
// ──────────────────────────────────────────────────────────

use crate::resultaat::weergave_fragment;
use crate::thema::GOUD;
use eframe::egui::{
    self, Align, FontFamily, FontId, Key, Layout, RichText, TextFormat, text::LayoutJob,
};
use pwspeller_kern::spelling::Regel;
use pwspeller_kern::talen::interfacetekst;
use std::ops::Range;
use zeroize::{Zeroize, Zeroizing};

/// Grootte van het teken of fragment in het midden (minstens 48 pt).
pub const GROOTTE_TEKEN: f32 = 96.0;
/// Grootte van de omschrijving (minstens 32 pt).
pub const GROOTTE_OMSCHRIJVING: f32 = 36.0;
const GROOTTE_VOLLEDIG: f32 = 24.0;

/// De voorleesmodus: de regels van de spelling en waar je in zit.
pub struct Voorlezen {
    pub regels: Vec<Regel>,
    pub index: usize,
    /// Het volledige wachtwoord of de wachtzin, voor de bovenste regel.
    tekst: Zeroizing<String>,
    /// Een stap is een woord, scheidingsteken of cijfer (wachtzin per woord) in plaats van een teken.
    per_woord: bool,
}

/// Wat de gebruiker in een beeld deed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actie {
    Geen,
    Stop,
}

/// Het deel van de volledige tekst (in tekens) dat bij deze regel hoort: bij een wachtzin per woord
/// het fragment zelf, anders het groepje van vier waar het teken in zit.
pub fn markering(regel: &Regel, per_woord: bool) -> Range<usize> {
    let begin = regel.positie.saturating_sub(1);
    if per_woord {
        begin..begin + regel.fragment.chars().count().max(1)
    } else {
        let groep = begin / 4 * 4;
        groep..groep + 4
    }
}

impl Voorlezen {
    /// Voorleesmodus op een tekst met haar spelregels. Zonder regels is er niets voor te lezen: `None`.
    pub fn nieuw(regels: Vec<Regel>, tekst: &str, per_woord: bool) -> Option<Self> {
        if regels.is_empty() {
            return None;
        }
        Some(Self {
            regels,
            index: 0,
            tekst: Zeroizing::new(tekst.to_owned()),
            per_woord,
        })
    }

    /// Alleen de regels (tests); de bovenste regel blijft dan leeg en de markering per groep.
    pub fn van_regels(regels: Vec<Regel>) -> Self {
        Self {
            regels,
            index: 0,
            tekst: Zeroizing::new(String::new()),
            per_woord: false,
        }
    }

    pub fn volgende(&mut self) {
        if self.index + 1 < self.regels.len() {
            self.index += 1;
        }
    }

    pub fn vorige(&mut self) {
        self.index = self.index.saturating_sub(1);
    }

    /// (huidige, totaal), 1-based.
    pub fn voortgang(&self) -> (usize, usize) {
        (self.index + 1, self.regels.len())
    }

    /// De regel waar je nu op staat; de index is begrensd, dus dit klopt zolang er regels zijn.
    pub fn huidige(&self) -> &Regel {
        &self.regels[self.index.min(self.regels.len().saturating_sub(1))]
    }

    /// Overschrijft de teksten met nullen en leegt de regels.
    pub fn wis(&mut self) {
        for r in &mut self.regels {
            r.fragment.zeroize();
            r.omschrijving.zeroize();
        }
        self.tekst.zeroize();
    }

    /// Spatie en pijlen alleen als geen widget focus heeft (op een knop activeert spatie die knop);
    /// Esc werkt altijd. Geeft terug of er gestopt moet worden.
    fn toetsen(&mut self, ctx: &egui::Context) -> bool {
        let heeft_focus = ctx.egui_wants_keyboard_input() || ctx.memory(|m| m.focused().is_some());
        let (stop, volgende, vorige) = ctx.input_mut(|i| {
            let stop = i.consume_key(egui::Modifiers::NONE, Key::Escape);
            if heeft_focus {
                return (stop, false, false);
            }
            (
                stop,
                i.consume_key(egui::Modifiers::NONE, Key::Space)
                    | i.consume_key(egui::Modifiers::NONE, Key::ArrowRight),
                i.consume_key(egui::Modifiers::NONE, Key::ArrowLeft),
            )
        });
        if volgende {
            self.volgende();
        }
        if vorige {
            self.vorige();
        }
        stop
    }

    /// De bovenste regel: de hele tekst met de huidige groep of het huidige fragment gemarkeerd.
    fn volledige_tekst(&self, standaard: egui::Color32, breedte: f32) -> LayoutJob {
        let bereik = markering(self.huidige(), self.per_woord);
        let gewoon = TextFormat {
            font_id: FontId::new(GROOTTE_VOLLEDIG, FontFamily::Monospace),
            color: standaard,
            ..Default::default()
        };
        let gemarkeerd = TextFormat {
            color: egui::Color32::BLACK,
            background: GOUD,
            ..gewoon.clone()
        };
        let mut job = LayoutJob::default();
        job.wrap.max_width = breedte;
        job.wrap.break_anywhere = true;
        let mut run = String::new();
        let mut in_markering = false;
        for (i, c) in self.tekst.chars().enumerate() {
            let nu = bereik.contains(&i);
            if nu != in_markering && !run.is_empty() {
                let f = if in_markering { &gemarkeerd } else { &gewoon };
                job.append(&run, 0.0, f.clone());
                run.clear();
            }
            in_markering = nu;
            run.push_str(&weergave_fragment(c.encode_utf8(&mut [0; 4])));
        }
        if !run.is_empty() {
            let f = if in_markering { &gemarkeerd } else { &gewoon };
            job.append(&run, 0.0, f.clone());
        }
        run.zeroize();
        job
    }

    /// Tekent de voorleesmodus over het hele venster.
    pub fn teken(&mut self, ui: &mut egui::Ui, taal: &str) -> Actie {
        let mut actie = if self.toetsen(ui.ctx()) {
            Actie::Stop
        } else {
            Actie::Geen
        };
        let standaard = ui.visuals().text_color();
        egui::Panel::top("voorlezen_tekst").show(ui, |ui| {
            ui.add_space(8.0);
            ui.label(self.volledige_tekst(standaard, ui.available_width()));
            ui.add_space(8.0);
        });
        egui::Panel::bottom("voorlezen_onder").show(ui, |ui| {
            ui.add_space(8.0);
            let (n, totaal) = self.voortgang();
            let sleutel = if self.per_woord {
                "voorlezen.voortgang_woord"
            } else {
                "voorlezen.voortgang"
            };
            let tekst = interfacetekst(taal, sleutel)
                .replace("{n}", &n.to_string())
                .replace("{totaal}", &totaal.to_string());
            ui.vertical_centered(|ui| ui.label(RichText::new(tekst).size(20.0)));
            ui.add_space(8.0);
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let knop = |ui: &mut egui::Ui, sleutel: &str| {
                    let r = ui.add(
                        egui::Button::new(RichText::new(interfacetekst(taal, sleutel)).size(20.0))
                            .min_size(egui::vec2(120.0, 44.0)),
                    );
                    // Na een muisklik hoort de focus niet op de knop te blijven staan,
                    // anders werken spatie en pijlen niet meer.
                    if r.clicked() {
                        r.surrender_focus();
                    }
                    r.clicked()
                };
                if knop(ui, "voorlezen.vorige") {
                    self.vorige();
                }
                if knop(ui, "voorlezen.volgende") {
                    self.volgende();
                }
                if knop(ui, "voorlezen.stop") {
                    actie = Actie::Stop;
                }
            });
            ui.add_space(8.0);
        });
        egui::CentralPanel::default().show(ui, |ui| {
            let regel = self.huidige();
            let fragment = weergave_fragment(&regel.fragment);
            // De omschrijving van een onbekend of onzichtbaar teken bevat het teken zelf.
            let omschrijving = if fragment == regel.fragment {
                regel.omschrijving.clone()
            } else {
                regel.omschrijving.replace(&regel.fragment, &fragment)
            };
            let lastig = regel.lastig;
            let breedte = ui.available_width();
            let tekens = fragment.chars().count().max(1) as f32;
            // Een woord breekt nooit midden in: de letters worden kleiner tot het past.
            let teken_grootte = GROOTTE_TEKEN.min(breedte / (tekens * 0.62)).max(20.0);
            let marge = (ui.available_height() * 0.15).min(24.0);
            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(marge);
                        ui.label(
                            RichText::new(fragment)
                                .font(FontId::new(teken_grootte, FontFamily::Monospace))
                                .strong(),
                        );
                        ui.add_space(16.0);
                        ui.label(RichText::new(omschrijving).size(GROOTTE_OMSCHRIJVING));
                        if lastig {
                            ui.add_space(12.0);
                            // Tekst met achtergrond in plaats van een symbool: geen ontbrekende glyphs.
                            ui.label(
                                RichText::new(format!(
                                    " {} ",
                                    interfacetekst(taal, "label.lastig")
                                ))
                                .size(GROOTTE_OMSCHRIJVING)
                                .color(egui::Color32::BLACK)
                                .background_color(GOUD),
                            );
                        }
                    });
                });
        });
        actie
    }
}

impl Drop for Voorlezen {
    fn drop(&mut self) {
        self.wis();
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
