// ── Over dit bestand ──────────────────────────────────────
// Doel:           Thema van de app: lettertype, de Petisys-kleuren voor licht en donker, en de kleuren van tekensoorten.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Eindreview: labels niet selecteerbaar, zodat egui geen wachtwoord naar het klembord kopieert.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: thema aangemaakt.
// ──────────────────────────────────────────────────────────

use eframe::egui::{
    self, Color32, FontData, FontDefinitions, FontFamily, Stroke, ThemePreference, Visuals,
};
use std::sync::Arc;

pub const ZWART: Color32 = Color32::from_rgb(0x11, 0x13, 0x12);
pub const GOUD: Color32 = Color32::from_rgb(0x92, 0x7C, 0x43);
pub const DONKERGOUD: Color32 = Color32::from_rgb(0x6D, 0x5D, 0x32);
pub const ANTRACIET: Color32 = Color32::from_rgb(0x2B, 0x2A, 0x26);
pub const GRIJS: Color32 = Color32::from_rgb(0x59, 0x59, 0x59);
pub const LICHTGRIJS: Color32 = Color32::from_rgb(0xE8, 0xE8, 0xE8);
/// Lichte goudtint voor tekst en links op de donkere achtergrond (het goud zelf haalt daar 4,5:1 niet).
pub const GOUD_LICHT: Color32 = Color32::from_rgb(0xD4, 0xBE, 0x7A);

/// Achtergrond van het hoofdvlak en van invoervakken, per thema.
pub const ACHTERGROND_LICHT: Color32 = Color32::WHITE;
pub const ACHTERGROND_DONKER: Color32 = ANTRACIET;

/// Kleuren voor tekensoorten in een wachtwoord (Taak 14), per thema.
pub const CIJFER_LICHT: Color32 = Color32::from_rgb(0x1F, 0x5F, 0xBF);
pub const CIJFER_DONKER: Color32 = Color32::from_rgb(0x7F, 0xB0, 0xFF);
pub const SPECIAAL_LICHT: Color32 = Color32::from_rgb(0xB3, 0x26, 0x1E);
pub const SPECIAAL_DONKER: Color32 = Color32::from_rgb(0xFF, 0x8A, 0x80);

const REGULIER: &[u8] = include_bytes!("../assets/AtkinsonHyperlegibleMono-Regular.ttf");
const VET: &[u8] = include_bytes!("../assets/AtkinsonHyperlegibleMono-Bold.ttf");

/// Naam van de lettertypefamilie voor vette tekst: `FontFamily::Name("vet")`.
pub const FAMILIE_VET: &str = "vet";

/// Zet het lettertype (Atkinson Hyperlegible Mono) en het thema (volgt het besturingssysteem).
pub fn zet_thema(ctx: &egui::Context) {
    ctx.set_fonts(lettertypen());
    ctx.set_visuals_of(egui::Theme::Light, visuals_licht());
    ctx.set_visuals_of(egui::Theme::Dark, visuals_donker());
    // Labels (resultaat, spelling) zijn niet selecteerbaar: egui zou er met Cmd/Ctrl+C een
    // wachtwoord mee naar het systeemklembord zetten, buiten het klembordbeheer om.
    ctx.all_styles_mut(|s| s.interaction.selectable_labels = false);
    ctx.set_theme(ThemePreference::System);
}

pub fn lettertypen() -> FontDefinitions {
    let mut f = FontDefinitions::empty();
    f.font_data.insert(
        "atkinson-regulier".to_owned(),
        Arc::new(FontData::from_static(REGULIER)),
    );
    f.font_data.insert(
        "atkinson-vet".to_owned(),
        Arc::new(FontData::from_static(VET)),
    );
    for familie in [FontFamily::Proportional, FontFamily::Monospace] {
        f.families
            .entry(familie)
            .or_default()
            .push("atkinson-regulier".to_owned());
    }
    f.families.insert(
        FontFamily::Name(FAMILIE_VET.into()),
        vec!["atkinson-vet".to_owned()],
    );
    f
}

pub fn visuals_licht() -> Visuals {
    let mut v = Visuals::light();
    v.override_text_color = Some(ZWART);
    v.panel_fill = ACHTERGROND_LICHT;
    v.window_fill = ACHTERGROND_LICHT;
    v.extreme_bg_color = ACHTERGROND_LICHT;
    v.faint_bg_color = LICHTGRIJS;
    v.hyperlink_color = DONKERGOUD;
    v.selection.bg_fill = DONKERGOUD;
    v.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, LICHTGRIJS);
    v.widgets.inactive.bg_fill = LICHTGRIJS;
    v.widgets.inactive.weak_bg_fill = LICHTGRIJS;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, GOUD);
    v.widgets.active.bg_stroke = Stroke::new(1.0, DONKERGOUD);
    v
}

pub fn visuals_donker() -> Visuals {
    let mut v = Visuals::dark();
    v.override_text_color = Some(LICHTGRIJS);
    v.panel_fill = ACHTERGROND_DONKER;
    v.window_fill = ACHTERGROND_DONKER;
    v.extreme_bg_color = ZWART;
    v.faint_bg_color = Color32::from_rgb(0x34, 0x33, 0x2E);
    v.hyperlink_color = GOUD_LICHT;
    v.selection.bg_fill = DONKERGOUD;
    v.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, GRIJS);
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, GOUD);
    v.widgets.active.bg_stroke = Stroke::new(1.0, GOUD);
    v
}

/// Contrastverhouding volgens WCAG 2.x tussen twee kleuren.
pub fn contrast(a: Color32, b: Color32) -> f64 {
    fn lum(c: Color32) -> f64 {
        let lin = |k: u8| {
            let s = f64::from(k) / 255.0;
            if s <= 0.04045 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
    }
    let (l1, l2) = (lum(a), lum(b));
    (l1.max(l2) + 0.05) / (l1.min(l2) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_zijn_niet_selecteerbaar() {
        // Selecteerbare labels zouden via Cmd/Ctrl+C het wachtwoord ongemarkeerd naar het
        // systeemklembord zetten, langs ons eigen klembordbeheer.
        let ctx = egui::Context::default();
        zet_thema(&ctx);
        for thema in [egui::Theme::Light, egui::Theme::Dark] {
            assert!(!ctx.style_of(thema).interaction.selectable_labels);
        }
    }

    #[test]
    fn tekensoorten_halen_4_5_tegen_elke_achtergrond() {
        for (kleur, achtergronden) in [
            (CIJFER_LICHT, [ACHTERGROND_LICHT, LICHTGRIJS]),
            (SPECIAAL_LICHT, [ACHTERGROND_LICHT, LICHTGRIJS]),
            (CIJFER_DONKER, [ACHTERGROND_DONKER, ZWART]),
            (SPECIAAL_DONKER, [ACHTERGROND_DONKER, ZWART]),
        ] {
            for achtergrond in achtergronden {
                let c = contrast(kleur, achtergrond);
                assert!(c >= 4.5, "{kleur:?} op {achtergrond:?}: {c:.2}");
            }
        }
    }

    #[test]
    fn tekst_en_links_halen_4_5() {
        assert!(contrast(ZWART, ACHTERGROND_LICHT) >= 4.5);
        assert!(contrast(DONKERGOUD, ACHTERGROND_LICHT) >= 4.5);
        assert!(contrast(LICHTGRIJS, ACHTERGROND_DONKER) >= 4.5);
        assert!(contrast(GOUD_LICHT, ACHTERGROND_DONKER) >= 4.5);
        assert!(contrast(Color32::WHITE, DONKERGOUD) >= 4.5);
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
