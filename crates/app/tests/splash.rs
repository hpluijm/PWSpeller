// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test het splashscreen: de vijf gegevens, de volgorde vóór het hoofdvenster en de duur van anderhalve seconde.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fix 2: duur van het splashscreen 1,5 seconde (grens 1499 ms).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14: volgordetest controleert ook dat de knop Genereer pas na de splash komt.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13 fixronde 1: volgordetest met titel in het hoofdvenster; test voor een kapot logo.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13: tests voor het splashscreen aangemaakt.
// ──────────────────────────────────────────────────────────

use eframe::egui;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pwspeller::app::PwApp;
use pwspeller::instellingen::VERSIE;
use pwspeller::splash::{Fase, SPLASHDUUR, laad_logo_uit, volgende_fase};
use pwspeller::thema;
use std::time::{Duration, Instant};

fn harnas(t0: Instant) -> Harness<'static, (PwApp, Instant)> {
    Harness::builder()
        .with_size([560.0, 640.0])
        .with_context_setup(thema::zet_thema)
        .build_ui_state(
            |ui, (app, nu): &mut (PwApp, Instant)| app.teken(ui, *nu),
            (PwApp::voor_test(t0), t0),
        )
}

#[test]
fn splash_toont_vijf_gegevens() {
    let t0 = Instant::now();
    let mut h = harnas(t0);
    h.step();
    h.step();
    assert!(matches!(h.state().0.fase, Fase::Splash { .. }));
    assert!(h.query_by_label("PWSpeller").is_some(), "naam ontbreekt");
    assert!(h.query_all_by_label("PWSpeller").count() == 1);
    assert!(
        h.query_by_label("Klant: Petisys").is_some(),
        "klant ontbreekt"
    );
    assert!(h.query_by_label(VERSIE).is_some(), "versie ontbreekt");
    assert!(
        h.query_by_label("© 2026 Petisys. Alle rechten voorbehouden.")
            .is_some(),
        "copyright ontbreekt"
    );
    assert!(h.query_by_label("Petisys").is_some(), "logo ontbreekt");
}

#[test]
fn splash_komt_voor_hoofdvenster() {
    let t0 = Instant::now();
    let mut h = harnas(t0);
    h.step();
    assert!(matches!(h.state().0.fase, Fase::Splash { .. }));
    // Het hoofdvenster toont (tijdelijk) de titel als kop; de splash toont de naam ook, maar
    // als eigen kop in vet. Het onderscheid: de splash heeft de regel "Klant: Petisys", het hoofdvenster niet.
    assert!(h.query_by_label("Klant: Petisys").is_some());
    assert!(
        h.query_by_label("Genereer").is_none(),
        "knop Genereer hoort niet op de splash"
    );
    h.state_mut().1 = t0 + SPLASHDUUR;
    h.step();
    h.step();
    assert!(matches!(h.state().0.fase, Fase::Hoofd));
    assert!(h.query_by_label("Klant: Petisys").is_none());
    assert!(h.query_by_label(VERSIE).is_none());
    assert!(
        h.query_by_label("Genereer").is_some(),
        "hoofdvenster toont de knop Genereer"
    );
}

#[test]
fn kapot_logo_geeft_geen_paniek_en_splash_blijft() {
    let ctx = egui::Context::default();
    assert!(laad_logo_uit(&ctx, b"geen png").is_none());
    let t0 = Instant::now();
    let mut h = harnas(t0);
    h.state_mut().0.zet_logo(None);
    h.step();
    assert!(h.query_by_label("Klant: Petisys").is_some());
    assert!(h.query_by_label("PWSpeller").is_some());
    assert!(h.query_by_label("Petisys").is_none(), "geen logo verwacht");
}

#[test]
fn na_anderhalve_seconde_hoofdvenster() {
    let t0 = Instant::now();
    assert!(matches!(
        volgende_fase(
            &Fase::Splash { start: t0 },
            t0 + Duration::from_millis(1499)
        ),
        Fase::Splash { .. }
    ));
    assert!(matches!(
        volgende_fase(&Fase::Splash { start: t0 }, t0 + SPLASHDUUR),
        Fase::Hoofd
    ));
    assert!(matches!(
        volgende_fase(
            &Fase::Splash { start: t0 },
            t0 + Duration::from_millis(1500)
        ),
        Fase::Hoofd
    ));
    assert!(matches!(volgende_fase(&Fase::Hoofd, t0), Fase::Hoofd));
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
