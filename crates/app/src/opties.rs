// ── Over dit bestand ──────────────────────────────────────
// Doel:           De opties van het actieve tabblad: tekensets, lengte en minima voor wachtwoorden; woorden, scheidingsteken en woordenlijst voor wachtzinnen.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14: aangemaakt.
// ──────────────────────────────────────────────────────────

use eframe::egui::{self, Slider};
use pwspeller_kern::generator::{WachtwoordOpties, WachtzinOpties};
use pwspeller_kern::talen::{catalogus, interfacetekst};

/// De opties van een wachtwoord. De laatste tekenset die aanstaat is niet uit te zetten, en een
/// minimum is alleen te kiezen als de bijbehorende tekenset aanstaat.
pub fn wachtwoord_opties(ui: &mut egui::Ui, taal: &str, o: &mut WachtwoordOpties) {
    ui.add(Slider::new(&mut o.lengte, 5..=128).text(interfacetekst(taal, "optie.lengte")));
    let aan = [o.hoofdletters, o.kleine_letters, o.cijfers, o.speciaal]
        .iter()
        .filter(|&&a| a)
        .count();
    let sets = [
        (&mut o.hoofdletters, "optie.hoofdletters"),
        (&mut o.kleine_letters, "optie.kleine_letters"),
        (&mut o.cijfers, "optie.cijfers"),
        (&mut o.speciaal, "optie.speciaal"),
    ];
    for (waarde, sleutel) in sets {
        let laatste = *waarde && aan == 1;
        ui.add_enabled(
            !laatste,
            egui::Checkbox::new(waarde, interfacetekst(taal, sleutel)),
        );
    }
    if o.cijfers {
        ui.add(
            Slider::new(&mut o.min_cijfers, 1..=9).text(interfacetekst(taal, "optie.min_cijfers")),
        );
    }
    if o.speciaal {
        ui.add(
            Slider::new(&mut o.min_speciaal, 1..=9)
                .text(interfacetekst(taal, "optie.min_speciaal")),
        );
    }
    ui.checkbox(
        &mut o.vermijd_dubbelzinnig,
        interfacetekst(taal, "optie.dubbelzinnig"),
    );
}

/// De opties van een wachtzin; `woordenlijst` is `None` zolang die de spellingstaal volgt.
pub fn wachtzin_opties(
    ui: &mut egui::Ui,
    taal: &str,
    o: &mut WachtzinOpties,
    woordenlijst: &mut Option<String>,
) {
    ui.add(Slider::new(&mut o.woorden, 3..=20).text(interfacetekst(taal, "optie.woorden")));
    ui.horizontal(|ui| {
        ui.label(interfacetekst(taal, "optie.scheidingsteken"));
        ui.add(egui::TextEdit::singleline(&mut o.scheidingsteken).desired_width(80.0));
    });
    ui.checkbox(
        &mut o.hoofdletters,
        interfacetekst(taal, "optie.woord_hoofdletter"),
    );
    ui.checkbox(&mut o.cijfer, interfacetekst(taal, "optie.woord_cijfer"));
    let volg = interfacetekst(taal, "label.woordenlijst_volg");
    egui::ComboBox::from_label(interfacetekst(taal, "optie.woordenlijst"))
        .selected_text(woordenlijst.as_deref().unwrap_or(volg))
        .show_ui(ui, |ui| {
            ui.selectable_value(woordenlijst, None, volg);
            for code in catalogus().woordenlijsten.keys() {
                ui.selectable_value(woordenlijst, Some(code.clone()), code);
            }
        });
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
