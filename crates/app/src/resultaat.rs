// ── Over dit bestand ──────────────────────────────────────
// Doel:           Het resultaat in kleur en groepjes van vier, de sterktebalk en de spelling als genummerde lijst.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14 fixronde 1: SpelCache (wist zichzelf), groepjes met onzichtbare tekens als code, opmaak per run.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14: aangemaakt.
// ──────────────────────────────────────────────────────────

use crate::thema::{
    CIJFER_DONKER, CIJFER_LICHT, FAMILIE_VET, GOUD, SPECIAAL_DONKER, SPECIAAL_LICHT,
};
use eframe::egui::{self, Color32, FontFamily, FontId, ProgressBar, RichText, TextFormat, text};
use pwspeller_kern::spelling::{Regel, Soort};
use pwspeller_kern::sterkte::{Niveau, niveau};
use pwspeller_kern::talen::interfacetekst;
use zeroize::Zeroize;

const RESULTAAT_GROOTTE: f32 = 28.0;
/// Bij een sterkte van deze bits is de balk vol.
const BALK_VOL_BITS: f64 = 128.0;

/// Deelt tekst in groepjes van vier tekens, met de startpositie (1-based) van elk groepje.
pub fn groepjes(tekst: &str) -> Vec<(usize, String)> {
    let mut tekens: Vec<char> = tekst.chars().collect();
    let uit = tekens
        .chunks(4)
        .enumerate()
        .map(|(i, groep)| (i * 4 + 1, groep.iter().collect()))
        .collect();
    tekens.zeroize();
    uit
}

/// Groepjes zoals ze in beeld komen: een onzichtbaar teken staat er als `U+xxxx`.
pub fn groepjes_weergave(tekst: &str) -> Vec<(usize, String)> {
    let mut groepen = groepjes(tekst);
    for (_, groep) in &mut groepen {
        let mut nieuw: String = groep
            .chars()
            .map(|c| weergave_fragment(c.encode_utf8(&mut [0; 4])))
            .collect();
        std::mem::swap(groep, &mut nieuw);
        nieuw.zeroize();
    }
    groepen
}

/// Wat het hoofdvenster van de spelling onthoudt zolang de tekst gelijk blijft. Bij het
/// vervangen of vrijgeven worden de teksten met nullen overschreven.
pub struct SpelCache {
    pub(crate) sleutel: (crate::hoofd::Tab, bool, String, String, u64),
    pub groepen: Vec<(usize, String)>,
    pub regels: Vec<Regel>,
}

impl SpelCache {
    pub fn nieuw(
        sleutel: (crate::hoofd::Tab, bool, String, String, u64),
        groepen: Vec<(usize, String)>,
        regels: Vec<Regel>,
    ) -> Self {
        Self {
            sleutel,
            groepen,
            regels,
        }
    }
}

impl Drop for SpelCache {
    fn drop(&mut self) {
        for (_, g) in &mut self.groepen {
            g.zeroize();
        }
        for r in &mut self.regels {
            r.fragment.zeroize();
            r.omschrijving.zeroize();
        }
    }
}

/// Een teken dat je niet ziet of dat in het lettertype ontbreekt, komt als code in beeld.
pub fn weergave_fragment(fragment: &str) -> String {
    let onzichtbaar = |c: char| {
        c.is_control()
            || c.is_whitespace()
            || matches!(c, '\u{200B}'..='\u{200F}' | '\u{2060}' | '\u{FEFF}')
    };
    match fragment.chars().next() {
        Some(c) if fragment.chars().count() == 1 && onzichtbaar(c) => {
            format!("U+{:04X}", c as u32)
        }
        _ => fragment.to_owned(),
    }
}

fn kleur_cijfer(donker: bool) -> Color32 {
    if donker { CIJFER_DONKER } else { CIJFER_LICHT }
}

fn kleur_speciaal(donker: bool) -> Color32 {
    if donker {
        SPECIAAL_DONKER
    } else {
        SPECIAAL_LICHT
    }
}

/// Opmaak per teken: hoofdletters vet, cijfers blauw, speciale tekens rood.
fn teken_opmaak(c: char, grootte: f32, standaard: Color32, donker: bool) -> TextFormat {
    let (familie, kleur) = if c.is_ascii_digit() {
        (FontFamily::Monospace, kleur_cijfer(donker))
    } else if c.is_uppercase() {
        (FontFamily::Name(FAMILIE_VET.into()), standaard)
    } else if c.is_alphabetic() {
        (FontFamily::Monospace, standaard)
    } else {
        (FontFamily::Monospace, kleur_speciaal(donker))
    };
    TextFormat {
        font_id: FontId::new(grootte, familie),
        color: kleur,
        ..Default::default()
    }
}

/// Eén append per run van tekens met dezelfde opmaak.
fn opgemaakt(tekst: &str, grootte: f32, standaard: Color32, donker: bool) -> text::LayoutJob {
    let mut job = text::LayoutJob::default();
    let mut run = String::new();
    let mut huidig: Option<TextFormat> = None;
    for c in tekst.chars() {
        let f = teken_opmaak(c, grootte, standaard, donker);
        if huidig.as_ref().is_some_and(|h| *h != f) {
            job.append(&run, 0.0, huidig.take().unwrap_or_default());
            run.clear();
        }
        huidig.get_or_insert(f);
        run.push(c);
    }
    if let Some(f) = huidig {
        job.append(&run, 0.0, f);
    }
    job
}

/// Tekent de tekst in groepjes van vier, het startnummer klein erboven.
pub fn teken_resultaat(ui: &mut egui::Ui, alle: &[(usize, String)]) {
    let donker = ui.visuals().dark_mode;
    let standaard = ui.visuals().text_color();
    let groep_breedte = ui.fonts_mut(|f| {
        f.layout_no_wrap(
            "WWWW".to_owned(),
            FontId::new(RESULTAAT_GROOTTE, FontFamily::Name(FAMILIE_VET.into())),
            standaard,
        )
        .size()
        .x
    }) + ui.spacing().item_spacing.x * 2.0;
    let per_rij = ((ui.available_width() / groep_breedte).floor() as usize).max(1);
    for rij in alle.chunks(per_rij) {
        ui.horizontal(|ui| {
            for (start, groep) in rij {
                ui.vertical(|ui| {
                    ui.label(RichText::new(start.to_string()).size(11.0).weak());
                    ui.label(opgemaakt(groep, RESULTAAT_GROOTTE, standaard, donker));
                });
                ui.add_space(8.0);
            }
        });
    }
}

/// Sterktebalk met de bits en het niveau in woorden (kleur is nooit het enige verschil).
pub fn teken_sterkte(ui: &mut egui::Ui, taal: &str, bits: f64, schatting: bool) {
    let donker = ui.visuals().dark_mode;
    let (sleutel, kleur) = match niveau(bits) {
        Niveau::Zwak => ("sterkte.zwak", kleur_speciaal(donker)),
        Niveau::Redelijk => ("sterkte.redelijk", GOUD),
        Niveau::Sterk => ("sterkte.sterk", kleur_cijfer(donker)),
    };
    let titel = interfacetekst(
        taal,
        if schatting {
            "sterkte.schatting"
        } else {
            "label.sterkte"
        },
    );
    let bits_tekst = interfacetekst(taal, "label.bits").replace("{bits}", &format!("{bits:.0}"));
    ui.label(format!(
        "{titel}: {bits_tekst}, {}",
        interfacetekst(taal, sleutel)
    ));
    ui.add(
        ProgressBar::new((bits / BALK_VOL_BITS).clamp(0.0, 1.0) as f32)
            .fill(kleur)
            .desired_height(10.0),
    );
}

/// De spelling als genummerde lijst; een regel is één label (positie, teken en omschrijving).
pub fn teken_spelling(ui: &mut egui::Ui, taal: &str, regels: &[Regel]) {
    let donker = ui.visuals().dark_mode;
    let standaard = ui.visuals().text_color();
    if regels.iter().any(|r| r.soort == Soort::Onbekend) {
        ui.colored_label(
            kleur_speciaal(donker),
            interfacetekst(taal, "melding.moeilijk_teken"),
        );
    }
    for r in regels {
        let fragment = weergave_fragment(&r.fragment);
        let mut job = text::LayoutJob::default();
        let gewoon = TextFormat {
            font_id: FontId::new(16.0, FontFamily::Monospace),
            color: standaard,
            ..Default::default()
        };
        job.append(&format!("{}. ", r.positie), 0.0, gewoon.clone());
        let mut deel = fragment.chars();
        if let (Some(c), None) = (deel.next(), deel.next()) {
            job.append(&fragment, 0.0, teken_opmaak(c, 16.0, standaard, donker));
        } else {
            job.append(&fragment, 0.0, gewoon.clone());
        }
        // De omschrijving van een onbekend teken bevat het teken zelf; een onzichtbaar teken komt als code.
        let omschrijving = if fragment == r.fragment {
            r.omschrijving.clone()
        } else {
            r.omschrijving.replace(&r.fragment, &fragment)
        };
        job.append(&format!("  {omschrijving}"), 0.0, gewoon.clone());
        if r.lastig {
            let markering = format!("  ({})", interfacetekst(taal, "label.lastig"));
            job.append(&markering, 0.0, gewoon);
        }
        ui.label(job);
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
