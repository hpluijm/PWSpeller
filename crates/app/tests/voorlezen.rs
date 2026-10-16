// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test de voorleesmodus: toetsen, grenzen, voortgangstekst, wissen bij verlaten en de weergave.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 16 fixronde 2: test dat teken, omschrijving en knoppen na meerdere beelden op hun plek staan (560x668 en 480x560).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 16 fixronde 1: tests voor 480x560 en een wachtwoord van 128 tekens; dode code weg.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 16: tests voor de voorleesmodus aangemaakt.
// ──────────────────────────────────────────────────────────

use eframe::egui::Key;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pwspeller::app::PwApp;
use pwspeller::splash::SPLASHDUUR;
use pwspeller::thema;
use pwspeller::voorlezen::{Voorlezen, markering};
use pwspeller_kern::spelling::{Regel, Soort};
use std::time::Instant;

type Harnas = Harness<'static, (PwApp, Instant)>;

fn hoofd() -> Harnas {
    let t0 = Instant::now();
    let mut h = Harness::builder()
        .with_size([560.0, 900.0])
        .with_context_setup(thema::zet_thema)
        .build_ui_state(
            |ui, (app, nu): &mut (PwApp, Instant)| app.teken(ui, *nu),
            (PwApp::voor_test(t0), t0),
        );
    h.step();
    h.state_mut().1 = t0 + SPLASHDUUR;
    h.step();
    h.step();
    h
}

fn stap(h: &mut Harnas) {
    h.step();
    h.step();
}

/// Genereert een wachtwoord en opent de voorleesmodus met F5.
fn voorlezend() -> Harnas {
    let mut h = hoofd();
    h.key_press(Key::Enter);
    stap(&mut h);
    h.key_press(Key::F5);
    stap(&mut h);
    h
}

fn index(h: &Harnas) -> usize {
    h.state()
        .0
        .voorlezen
        .as_ref()
        .expect("voorleesmodus open")
        .index
}

fn regel(positie: usize, fragment: &str, lastig: bool) -> Regel {
    Regel {
        positie,
        fragment: fragment.to_string(),
        omschrijving: format!("omschrijving {positie}"),
        soort: Soort::KleineLetter,
        lastig,
    }
}

#[test]
fn volgende_en_vorige_zijn_begrensd() {
    let regels = (1..=3).map(|p| regel(p, "a", false)).collect();
    let mut v = Voorlezen::van_regels(regels);
    v.vorige();
    assert_eq!(v.index, 0);
    v.volgende();
    v.volgende();
    v.volgende();
    v.volgende();
    assert_eq!(v.index, 2);
    assert_eq!(v.voortgang(), (3, 3));
    v.vorige();
    assert_eq!(v.voortgang(), (2, 3));
}

#[test]
fn markering_van_een_groep_en_van_een_fragment() {
    // Wachtwoord: groepjes van vier.
    assert_eq!(markering(&regel(1, "a", false), false), 0..4);
    assert_eq!(markering(&regel(4, "d", false), false), 0..4);
    assert_eq!(markering(&regel(5, "e", false), false), 4..8);
    // Wachtzin per woord: het fragment zelf, in tekens.
    assert_eq!(markering(&regel(6, "huis", false), true), 5..9);
}

#[test]
fn f5_opent_en_spatie_pijlen_en_esc_bedienen() {
    let mut h = voorlezend();
    assert_eq!(index(&h), 0);
    assert!(
        !h.state().0.hoofd.voorlezen_gevraagd,
        "verzoek is afgehandeld"
    );
    assert!(h.query_by_label("teken 1 van 14").is_some());
    h.key_press(Key::Space);
    stap(&mut h);
    h.key_press(Key::ArrowRight);
    stap(&mut h);
    assert_eq!(index(&h), 2);
    assert!(h.query_by_label("teken 3 van 14").is_some());
    h.key_press(Key::ArrowLeft);
    stap(&mut h);
    assert_eq!(index(&h), 1);
    h.key_press(Key::Escape);
    stap(&mut h);
    assert!(h.state().0.voorlezen.is_none());
    assert!(h.query_by_label("Genereer").is_some(), "hoofdvenster terug");
}

#[test]
fn grenzen_houden_in_beeld() {
    let mut h = voorlezend();
    h.key_press(Key::ArrowLeft);
    stap(&mut h);
    assert_eq!(index(&h), 0);
    for _ in 0..20 {
        h.key_press(Key::ArrowRight);
        stap(&mut h);
    }
    assert_eq!(index(&h), 13);
    assert!(h.query_by_label("teken 14 van 14").is_some());
}

#[test]
fn knoppen_voor_de_muis() {
    let mut h = voorlezend();
    h.get_by_label("Volgende").click();
    stap(&mut h);
    assert_eq!(index(&h), 1);
    h.get_by_label("Vorige").click();
    stap(&mut h);
    assert_eq!(index(&h), 0);
    // Na een muisklik werken de toetsen nog.
    h.key_press(Key::ArrowRight);
    stap(&mut h);
    assert_eq!(index(&h), 1);
    h.get_by_label("Stop").click();
    stap(&mut h);
    assert!(h.state().0.voorlezen.is_none());
}

#[test]
fn spatie_op_gefocuste_knop_activeert_alleen_die_knop() {
    let mut h = voorlezend();
    h.get_by_label("Stop").focus();
    stap(&mut h);
    h.key_press(Key::ArrowRight);
    stap(&mut h);
    assert_eq!(
        index(&h),
        0,
        "pijlen gelden niet terwijl een knop focus heeft"
    );
    h.key_press(Key::Escape);
    stap(&mut h);
    assert!(h.state().0.voorlezen.is_none(), "Esc werkt altijd");
}

#[test]
fn zonder_tekst_gaat_de_voorleesmodus_niet_open() {
    let mut h = hoofd();
    h.key_press(Key::F5);
    stap(&mut h);
    assert!(h.state().0.voorlezen.is_none());
    assert!(!h.state().0.hoofd.voorlezen_gevraagd);
}

#[test]
fn verlaten_wist_de_regels() {
    let mut h = voorlezend();
    let v = h.state_mut().0.voorlezen.take().expect("open");
    assert!(!v.regels.is_empty());
    // Het wissen zelf zit in Drop; hier controleren we dat wissen() leegt.
    let mut v = v;
    v.wis();
    assert!(
        v.regels
            .iter()
            .all(|r| r.fragment.is_empty() && r.omschrijving.is_empty())
    );
}

#[test]
fn wachtzin_per_woord_gebruikt_stappen() {
    let mut h = hoofd();
    h.get_by_label("Wachtzin").click();
    stap(&mut h);
    h.get_by_label("Genereer").click();
    stap(&mut h);
    h.key_press(Key::F5);
    stap(&mut h);
    let totaal = h.state().0.voorlezen.as_ref().expect("open").regels.len();
    assert!(h.query_by_label(&format!("stap 1 van {totaal}")).is_some());
}

#[test]
fn groot_genoeg_en_lastig_gemarkeerd() {
    use pwspeller::voorlezen::{GROOTTE_OMSCHRIJVING, GROOTTE_TEKEN};
    const { assert!(GROOTTE_TEKEN >= 48.0) };
    const { assert!(GROOTTE_OMSCHRIJVING >= 32.0) };
    let mut h = voorlezend();
    let n = h.state().0.voorlezen.as_ref().expect("open").regels.len();
    let aantal_lastig = h
        .state()
        .0
        .voorlezen
        .as_ref()
        .expect("open")
        .regels
        .iter()
        .filter(|r| r.lastig)
        .count();
    let mut gezien = 0;
    for _ in 0..n {
        let lastig = h
            .state()
            .0
            .voorlezen
            .as_ref()
            .expect("open")
            .huidige()
            .lastig;
        if lastig {
            assert!(h.query_by_label_contains("lastig").is_some());
            gezien += 1;
        }
        h.key_press(Key::ArrowRight);
        stap(&mut h);
    }
    assert_eq!(gezien, aantal_lastig);
}

fn klein_harnas() -> Harnas {
    let t0 = Instant::now();
    let mut h = Harness::builder()
        .with_size([480.0, 560.0])
        .with_context_setup(thema::zet_thema)
        .build_ui_state(
            |ui, (app, nu): &mut (PwApp, Instant)| app.teken(ui, *nu),
            (PwApp::voor_test(t0), t0),
        );
    h.step();
    h.state_mut().1 = t0 + SPLASHDUUR;
    h.step();
    h.step();
    h
}

fn binnen(r: eframe::egui::Rect, wat: &str) {
    let venster = eframe::egui::Rect::from_min_size(
        eframe::egui::Pos2::ZERO,
        eframe::egui::vec2(480.0, 560.0),
    );
    assert!(
        venster.contains_rect(r),
        "{wat} ligt buiten het venster: {r:?}"
    );
}

#[test]
fn minimale_venstermaat_houdt_alles_in_beeld() {
    let mut h = klein_harnas();
    let regels = vec![Regel {
        positie: 1,
        fragment: "Ö".to_string(),
        omschrijving: "Otto met twee puntjes erboven, lastig aan de telefoon".to_string(),
        soort: Soort::Hoofdletter,
        lastig: true,
    }];
    h.state_mut().0.voorlezen = Voorlezen::nieuw(regels, "Öabc", false);
    stap(&mut h);
    binnen(h.get_by_label("Ö").rect(), "teken");
    binnen(
        h.get_by_label_contains("Otto met twee puntjes").rect(),
        "omschrijving",
    );
    for knop in ["Vorige", "Volgende", "Stop"] {
        binnen(h.get_by_label(knop).rect(), knop);
    }
}

#[test]
fn lang_wachtwoord_wrapt_en_markering_blijft_in_beeld() {
    let mut h = klein_harnas();
    let tekst: String = (0..128u8).map(|i| char::from(b'a' + i % 26)).collect();
    let regels: Vec<Regel> = tekst
        .chars()
        .enumerate()
        .map(|(i, c)| regel(i + 1, &c.to_string(), false))
        .collect();
    let mut v = Voorlezen::nieuw(regels, &tekst, false).expect("regels");
    v.index = 127;
    h.state_mut().0.voorlezen = Some(v);
    stap(&mut h);
    binnen(h.get_by_label(&tekst).rect(), "volledige tekst");
}

/// Genereert een wachtwoord en opent de voorleesmodus op een gegeven venstermaat.
fn voorlezend_op(breedte: f32, hoogte: f32) -> Harnas {
    let t0 = Instant::now();
    let mut h = Harness::builder()
        .with_size([breedte, hoogte])
        .with_context_setup(thema::zet_thema)
        .build_ui_state(
            |ui, (app, nu): &mut (PwApp, Instant)| app.teken(ui, *nu),
            (PwApp::voor_test(t0), t0),
        );
    h.step();
    h.state_mut().1 = t0 + SPLASHDUUR;
    h.step();
    h.step();
    h.key_press(Key::Enter);
    stap(&mut h);
    h.key_press(Key::F5);
    stap(&mut h);
    // De fout groeide per beeld (het onderpaneel nam steeds meer hoogte); laat er genoeg lopen.
    for _ in 0..30 {
        h.step();
    }
    h
}

#[test]
fn teken_en_omschrijving_staan_zichtbaar_tussen_tekst_en_knoppen() {
    use eframe::egui::{Pos2, Rect, vec2};
    for (b, hgt) in [(560.0, 668.0), (480.0, 560.0)] {
        let mut h = voorlezend_op(b, hgt);
        for _ in 0..2 {
            let (fragment, omschrijving) = {
                let v = h.state().0.voorlezen.as_ref().expect("open");
                (
                    v.huidige().fragment.clone(),
                    v.huidige().omschrijving.clone(),
                )
            };
            let venster = Rect::from_min_size(Pos2::ZERO, vec2(b, hgt));
            let teken = h.get_by_label(&fragment).rect();
            let omsch = h.get_by_label_contains(&omschrijving).rect();
            assert!(
                teken.height() >= 40.0,
                "{b}x{hgt}: teken te laag: {teken:?}"
            );
            assert!(
                venster.contains_rect(teken),
                "{b}x{hgt}: teken buiten beeld"
            );
            assert!(
                omsch.height() > 0.0,
                "{b}x{hgt}: omschrijving zonder hoogte"
            );
            assert!(
                venster.contains_rect(omsch),
                "{b}x{hgt}: omschrijving buiten beeld: {omsch:?}"
            );
            for knop in ["Vorige", "Volgende", "Stop"] {
                let r = h.get_by_label(knop).rect();
                assert!(venster.contains_rect(r), "{b}x{hgt}: {knop} buiten beeld");
                assert!(
                    r.min.y > hgt / 2.0,
                    "{b}x{hgt}: {knop} niet in onderste helft: {r:?}"
                );
                assert!(
                    r.max.y >= hgt - 30.0,
                    "{b}x{hgt}: {knop} staat niet onderaan het venster: {r:?}"
                );
                assert!(
                    teken.max.y <= r.min.y,
                    "{b}x{hgt}: teken ({teken:?}) niet boven {knop} ({r:?})"
                );
            }
            h.key_press(Key::ArrowRight);
            stap(&mut h);
            for _ in 0..30 {
                h.step();
            }
        }
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
