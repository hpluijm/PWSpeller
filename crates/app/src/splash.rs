// ── Over dit bestand ──────────────────────────────────────
// Doel:           Het splashscreen van drie seconden vóór het hoofdvenster: logo, naam, klant, versie en copyright.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13: splashscreen aangemaakt (afspraak 24).
// ──────────────────────────────────────────────────────────

use crate::instellingen::VERSIE;
use crate::thema::{FAMILIE_VET, GOUD};
use eframe::egui::{self, Color32, FontFamily, FontId, RichText, Stroke, TextureHandle, Vec2};
use pwspeller_kern::talen::interfacetekst;
use std::time::{Duration, Instant};

/// Hoe lang het splashscreen blijft staan.
pub const SPLASHDUUR: Duration = Duration::from_secs(3);

const LOGO: &[u8] = include_bytes!("../assets/petisys-logo.png");
const MAX_LOGOBREEDTE: u32 = 640;
const KLANT: &str = "Petisys";

/// Waar de app in zit: eerst het splashscreen, daarna het hoofdvenster.
#[derive(Debug, Clone, Copy)]
pub enum Fase {
    Splash { start: Instant },
    Hoofd,
}

pub fn volgende_fase(f: &Fase, nu: Instant) -> Fase {
    match f {
        Fase::Splash { start } if nu.saturating_duration_since(*start) < SPLASHDUUR => *f,
        _ => Fase::Hoofd,
    }
}

/// Decodeert het logo en maakt er een textuur van. Lukt dat niet, dan gaat er een
/// waarschuwing naar het logboek en loopt het splashscreen door zonder logo.
pub fn laad_logo(ctx: &egui::Context) -> Option<TextureHandle> {
    match image::load_from_memory_with_format(LOGO, image::ImageFormat::Png) {
        Ok(plaatje) => {
            // Verklein: het origineel (2639 px breed) past niet op elke videokaart (maximum 2048 px)
            // en is voor een logo van 320 punten breed (640 px bij scherpe schermen) veel te groot.
            let rgba = plaatje
                .resize(
                    MAX_LOGOBREEDTE,
                    MAX_LOGOBREEDTE,
                    image::imageops::FilterType::Lanczos3,
                )
                .to_rgba8();
            let grootte = [rgba.width() as usize, rgba.height() as usize];
            let kleur = egui::ColorImage::from_rgba_unmultiplied(grootte, rgba.as_raw());
            Some(ctx.load_texture("petisys-logo", kleur, egui::TextureOptions::LINEAR))
        }
        Err(e) => {
            log::warn!("splashscreen: logo niet geladen: {e}");
            None
        }
    }
}

/// Tekent het splashscreen en vraagt een herteken aan tot het voorbij is.
pub fn teken(
    ui: &mut egui::Ui,
    taal: &str,
    logo: Option<&TextureHandle>,
    start: Instant,
    nu: Instant,
) {
    let rest = SPLASHDUUR.saturating_sub(nu.saturating_duration_since(start));
    ui.ctx().request_repaint_after(rest);
    let klant = interfacetekst(taal, "splash.klant").replace("{klant}", KLANT);
    egui::CentralPanel::default().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space((ui.available_height() * 0.2).max(8.0));
            if let Some(t) = logo {
                let breedte = ui.available_width().min(320.0);
                let verhouding = t.size_vec2().y / t.size_vec2().x;
                ui.add(
                    egui::Image::from_texture(t)
                        .fit_to_exact_size(Vec2::new(breedte, breedte * verhouding))
                        .alt_text(KLANT),
                );
            }
            ui.add_space(16.0);
            ui.label(
                RichText::new(interfacetekst(taal, "app.titel"))
                    .font(FontId::new(32.0, FontFamily::Name(FAMILIE_VET.into()))),
            );
            let (lijn, _) = ui.allocate_exact_size(Vec2::new(160.0, 2.0), egui::Sense::hover());
            ui.painter().line_segment(
                [lijn.left_center(), lijn.right_center()],
                Stroke::new(1.0, GOUD),
            );
            ui.add_space(8.0);
            ui.label(klant);
            ui.label(VERSIE);
            ui.add_space(16.0);
            ui.label(RichText::new(interfacetekst(taal, "splash.copyright")).color(Color32::GRAY));
        });
    });
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
