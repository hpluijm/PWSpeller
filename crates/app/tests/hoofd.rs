// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test het hoofdvenster: tabbladen, opties, resultaat, spelling, sterkte, sneltoetsen en het bewaren van instellingen.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 15: de kopieertest gebruikt een nepbord, want de app handelt het verzoek nu zelf af.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14 fixronde 1: tests voor groepjes bij eigen tekst, de cache, focus op een knop en het wissen van de eigen tekst.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14: tests voor het hoofdvenster aangemaakt.
// ──────────────────────────────────────────────────────────

use eframe::egui::{Key, Modifiers};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use pwspeller::app::PwApp;
use pwspeller::hoofd::Tab;
use pwspeller::instellingen::Instellingen;
use pwspeller::resultaat::groepjes;
use pwspeller::splash::SPLASHDUUR;
use pwspeller::thema;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone, Default)]
struct NepBord(Rc<RefCell<Option<String>>>);

impl pwspeller::klembord::Bord for NepBord {
    fn zet_vertrouwelijk(&mut self, tekst: &str) -> Result<(), String> {
        *self.0.borrow_mut() = Some(tekst.to_string());
        Ok(())
    }
    fn lees(&mut self) -> Option<String> {
        self.0.borrow().clone()
    }
    fn wis(&mut self) -> Result<(), String> {
        *self.0.borrow_mut() = None;
        Ok(())
    }
}

type Harnas = Harness<'static, (PwApp, Instant)>;

fn harnas_met(app: PwApp, t0: Instant) -> Harnas {
    let mut h = Harness::builder()
        .with_size([560.0, 900.0])
        .with_context_setup(thema::zet_thema)
        .build_ui_state(
            |ui, (app, nu): &mut (PwApp, Instant)| app.teken(ui, *nu),
            (app, t0),
        );
    h.step();
    h.state_mut().1 = t0 + SPLASHDUUR;
    h.step();
    h.step();
    h
}

fn hoofd() -> Harnas {
    let t0 = Instant::now();
    harnas_met(PwApp::voor_test(t0), t0)
}

fn stap(h: &mut Harnas) {
    h.step();
    h.step();
}

#[test]
fn groepjes_van_vier() {
    let verwacht: Vec<(usize, String)> =
        vec![(1, "abcd".into()), (5, "efgh".into()), (9, "ij".into())];
    assert_eq!(groepjes("abcdefghij"), verwacht);
    assert!(groepjes("").is_empty());
    // Tekens, geen bytes.
    assert_eq!(
        groepjes("ééééé"),
        vec![(1, "éééé".to_string()), (5, "é".to_string())]
    );
}

#[test]
fn genereer_toont_14_tekens_en_spelling() {
    let mut h = hoofd();
    assert!(h.state().0.hoofd.wachtwoord.is_none());
    h.get_by_label("Genereer").click();
    stap(&mut h);
    let app = &h.state().0;
    let g = app
        .hoofd
        .wachtwoord
        .as_ref()
        .expect("wachtwoord gegenereerd");
    assert_eq!(g.tekst.chars().count(), 14);
    assert_eq!(app.spelregels().len(), 14);
    assert_eq!(app.geschiedenis.items().len(), 1);
    assert!(
        h.query_by_label_contains("14. ").is_some(),
        "spellingregel 14 ontbreekt"
    );
    assert!(h.query_by_label_contains("15. ").is_none());
    assert!(
        h.query_by_label_contains(" bits").is_some(),
        "sterkte ontbreekt"
    );
}

#[test]
fn laatste_tekenset_laat_zich_niet_uitzetten() {
    let mut h = hoofd();
    {
        let o = &mut h.state_mut().0.instellingen.wachtwoord;
        o.hoofdletters = false;
        o.kleine_letters = false;
    }
    stap(&mut h);
    assert!(h.get_by_label("Cijfers").accesskit_node().is_disabled());
    assert!(
        !h.get_by_label("Hoofdletters")
            .accesskit_node()
            .is_disabled()
    );
    h.state_mut().0.instellingen.wachtwoord.hoofdletters = true;
    stap(&mut h);
    assert!(!h.get_by_label("Cijfers").accesskit_node().is_disabled());
}

#[test]
fn min_cijfers_alleen_zichtbaar_als_cijfers_aan() {
    let mut h = hoofd();
    assert!(h.query_all_by_label("Minimaal aantal cijfers").count() > 0);
    assert!(
        h.query_all_by_label("Minimaal aantal speciale tekens")
            .count()
            == 0
    );
    {
        let o = &mut h.state_mut().0.instellingen.wachtwoord;
        o.cijfers = false;
        o.speciaal = true;
    }
    stap(&mut h);
    assert!(h.query_all_by_label("Minimaal aantal cijfers").count() == 0);
    assert!(
        h.query_all_by_label("Minimaal aantal speciale tekens")
            .count()
            > 0
    );
}

#[test]
fn minima_groter_dan_lengte_tonen_melding_en_hogere_lengte() {
    let mut h = hoofd();
    assert!(
        h.query_by_label("De lengte is verhoogd zodat alle gekozen opties passen.")
            .is_none()
    );
    {
        let o = &mut h.state_mut().0.instellingen.wachtwoord;
        o.lengte = 5;
        o.min_cijfers = 9; // hoofdletters 1 + kleine letters 1 + cijfers 9
    }
    stap(&mut h);
    assert_eq!(h.state().0.instellingen.wachtwoord.lengte, 11);
    assert!(
        h.query_by_label("De lengte is verhoogd zodat alle gekozen opties passen.")
            .is_some()
    );
}

#[test]
fn wisselen_van_spellingstaal_spelt_opnieuw() {
    let mut h = hoofd();
    h.state_mut().0.hoofd.tab = Tab::EigenTekst;
    h.state_mut().0.hoofd.eigen_tekst = "capital".to_string();
    stap(&mut h);
    assert!(
        h.query_by_label_contains("kleine letter Cornelis")
            .is_some()
    );
    h.state_mut().0.instellingen.spellingstaal = "en".to_string();
    stap(&mut h);
    assert!(
        h.query_by_label_contains("kleine letter Cornelis")
            .is_none()
    );
    assert!(h.query_by_label_contains("small Charlie").is_some());
}

#[test]
fn eigen_tekst_met_e_accent_toont_waarschuwing() {
    let mut h = hoofd();
    h.state_mut().0.hoofd.tab = Tab::EigenTekst;
    stap(&mut h);
    assert!(h.query_by_label_contains("Let op: dit teken").is_none());
    h.state_mut().0.hoofd.eigen_tekst = "caf\u{e9}".to_string();
    stap(&mut h);
    assert!(h.query_by_label_contains("U+00E9").is_some());
    assert!(
        h.query_by_label_contains("Let op: dit teken is lastig aan de telefoon.")
            .is_some()
    );
    assert!(h.query_by_label_contains("Geschatte sterkte").is_some());
}

#[test]
fn onzichtbaar_teken_verschijnt_als_code() {
    let mut h = hoofd();
    h.state_mut().0.hoofd.tab = Tab::EigenTekst;
    h.state_mut().0.hoofd.eigen_tekst = "a\tb".to_string();
    stap(&mut h);
    assert!(h.query_by_label_contains("2. U+0009").is_some());
    assert!(
        h.query_all_by_label_contains("\t").count() == 0,
        "ruw tabteken in de weergave"
    );
}

#[test]
fn wachtzin_tab_spelt_per_woord_en_knop_per_teken_schakelt() {
    let mut h = hoofd();
    h.state_mut().0.hoofd.tab = Tab::Wachtzin;
    stap(&mut h);
    h.get_by_label("Genereer").click();
    stap(&mut h);
    let (tekens, per_woord) = {
        let app = &h.state().0;
        let g = app.hoofd.wachtzin.as_ref().expect("wachtzin gegenereerd");
        (g.tekst.chars().count(), app.spelregels().len())
    };
    assert!(
        per_woord < tekens,
        "per woord moet korter zijn dan per teken"
    );
    h.get_by_label("Per teken").click();
    stap(&mut h);
    assert_eq!(h.state().0.spelregels().len(), tekens);
    assert!(h.query_by_label("Per woord").is_some());
    h.get_by_label("Per woord").click();
    stap(&mut h);
    assert_eq!(h.state().0.spelregels().len(), per_woord);
}

#[test]
fn sneltoetsen_enter_f5_en_kopieer() {
    let mut h = hoofd();
    let bord = NepBord::default();
    h.state_mut().0.zet_bord(Box::new(bord.clone()));
    h.key_press(Key::Enter);
    stap(&mut h);
    assert!(h.state().0.hoofd.wachtwoord.is_some(), "Enter genereert");
    assert!(!h.state().0.hoofd.voorlezen_gevraagd);
    h.key_press(Key::F5);
    stap(&mut h);
    assert!(h.state().0.hoofd.voorlezen_gevraagd);
    h.key_press_modifiers(Modifiers::COMMAND, Key::C);
    stap(&mut h);
    // De app handelt het verzoek zelf af (Taak 15): het nepbord heeft de tekst, het verzoek is weg.
    let verwacht = h.state().0.huidige_tekst().unwrap().to_string();
    assert_eq!(bord.0.borrow().as_deref(), Some(verwacht.as_str()));
    assert!(!h.state_mut().0.neem_kopieer_verzoek());
}

#[test]
fn sneltoetsen_doen_niets_zonder_resultaat() {
    let mut h = hoofd();
    h.key_press(Key::F5);
    h.key_press_modifiers(Modifiers::COMMAND, Key::C);
    stap(&mut h);
    assert!(!h.state().0.hoofd.voorlezen_gevraagd);
    assert!(!h.state_mut().0.neem_kopieer_verzoek());
}

#[test]
fn instellingen_worden_bewaard_bij_elke_wijziging() {
    let map = std::env::temp_dir().join(format!("pwspeller-hoofdtest-{}", std::process::id()));
    let pad = map.join("instellingen.toml");
    let t0 = Instant::now();
    let instellingen = Instellingen {
        interfacetaal: Some("nl".into()),
        ..Instellingen::default()
    };
    let app = PwApp::nieuw(None, t0, instellingen, pad.clone());
    let mut h = harnas_met(app, t0);
    assert!(!pad.exists(), "zonder wijziging wordt er niets geschreven");
    h.state_mut().0.instellingen.wachtwoord.lengte = 20;
    stap(&mut h);
    let gelezen = Instellingen::laad(&pad);
    assert_eq!(gelezen.wachtwoord.lengte, 20);
    let _ = std::fs::remove_dir_all(&map);
}

#[test]
fn app_voor_test_schrijft_nooit() {
    let app = PwApp::voor_test(Instant::now());
    assert!(app.instellingenpad.is_none());
}

#[test]
fn eigen_tekst_toont_groepjes_met_positienummers_en_codes() {
    let mut h = hoofd();
    h.state_mut().0.hoofd.tab = Tab::EigenTekst;
    h.state_mut().0.hoofd.eigen_tekst = "abcde\tf".to_string();
    stap(&mut h);
    assert!(h.query_by_label("abcd").is_some(), "eerste groepje");
    assert!(
        h.query_by_label("eU+0009f").is_some(),
        "tweede groepje met code"
    );
    assert!(h.query_by_label("1").is_some());
    assert!(h.query_by_label("5").is_some());
}

#[test]
fn cache_wordt_alleen_opnieuw_gebouwd_bij_een_wijziging() {
    let mut h = hoofd();
    h.get_by_label("Genereer").click();
    stap(&mut h);
    let n = h.state().0.hoofd.cache_opbouwen;
    assert!(n >= 1);
    for _ in 0..5 {
        h.step();
    }
    assert_eq!(
        h.state().0.hoofd.cache_opbouwen,
        n,
        "zonder wijziging geen nieuwe opbouw"
    );
    h.state_mut().0.instellingen.spellingstaal = "en".to_string();
    stap(&mut h);
    assert_eq!(h.state().0.hoofd.cache_opbouwen, n + 1);
    h.state_mut().0.hoofd.per_teken = true;
    stap(&mut h);
    assert_eq!(h.state().0.hoofd.cache_opbouwen, n + 2);
}

#[test]
fn enter_op_gefocuste_knop_genereert_niet() {
    let mut h = hoofd();
    h.key_press(Key::Enter);
    stap(&mut h);
    let eerste = h
        .state()
        .0
        .hoofd
        .wachtwoord
        .as_ref()
        .map(|g| g.tekst.to_string());
    assert!(eerste.is_some());
    h.get_by_label("Kopieer").focus();
    stap(&mut h);
    h.key_press(Key::Enter);
    stap(&mut h);
    let nu = h
        .state()
        .0
        .hoofd
        .wachtwoord
        .as_ref()
        .map(|g| g.tekst.to_string());
    assert_eq!(
        eerste, nu,
        "Enter op een knop activeert die knop, en genereert niet"
    );
    assert_eq!(h.state().0.geschiedenis.items().len(), 1);
}

#[test]
fn eigen_tekst_wordt_gewist_bij_tabwissel_en_leegmaken() {
    let mut h = hoofd();
    h.state_mut().0.hoofd.tab = Tab::EigenTekst;
    h.state_mut().0.hoofd.eigen_tekst = "geheim".to_string();
    stap(&mut h);
    assert_eq!(h.state().0.hoofd.eigen_tekst, "geheim");
    h.state_mut().0.hoofd.tab = Tab::Wachtwoord;
    stap(&mut h);
    assert!(h.state().0.hoofd.eigen_tekst.is_empty());
    assert!(h.state().0.hoofd.cache.is_none());
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
