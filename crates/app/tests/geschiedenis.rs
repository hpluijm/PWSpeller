// ── Over dit bestand ──────────────────────────────────────
// Doel:           Tests van het geschiedenispaneel, het wissen bij sluiten en na inactiviteit, en dat er niets gevoeligs op schijf komt.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17 fixronde 1: tests voor wissen van het resultaat, invoer als gebruik en een sterkere schijftest.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17: aangemaakt.
// ──────────────────────────────────────────────────────────

use eframe::egui::Key;
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pwspeller::app::PwApp;
use pwspeller::hoofd::Tab;
use pwspeller::splash::SPLASHDUUR;
use pwspeller::thema;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

type Harnas = Harness<'static, (PwApp, Instant)>;

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

fn genereer(h: &mut Harnas) {
    h.get_by_label("Genereer").click();
    stap(h);
}

fn kies_tab(h: &mut Harnas, tab: Tab) {
    h.state_mut().0.hoofd.tab = tab;
    stap(h);
}

fn open_paneel(h: &mut Harnas) {
    h.state_mut().0.geschiedenis_open = true;
    stap(h);
}

fn komt_voor(inhoud: &str, tekst: &str, heel_woord: bool) -> bool {
    if heel_woord {
        inhoud
            .split(|c: char| !c.is_alphanumeric())
            .any(|w| w == tekst)
    } else {
        inhoud.contains(tekst)
    }
}

#[test]
fn items_worden_gemaskeerd_getoond() {
    let mut h = hoofd();
    genereer(&mut h);
    open_paneel(&mut h);
    assert_eq!(h.state().0.geschiedenis.items().len(), 1);
    assert_eq!(h.state().0.aantal_getoond(), 0);
    assert!(h.query_by_label("••••••••").is_some());
    h.get_by_label("Toon geschiedenis").click();
    stap(&mut h);
    assert_eq!(h.state().0.aantal_getoond(), 1);
    assert!(h.query_by_label("••••••••").is_none());
    h.get_by_label("Verberg geschiedenis").click();
    stap(&mut h);
    assert_eq!(h.state().0.aantal_getoond(), 0);
    assert!(h.query_by_label("••••••••").is_some());
}

#[test]
fn sluiten_van_het_paneel_verbergt_getoonde_items() {
    let mut h = hoofd();
    genereer(&mut h);
    open_paneel(&mut h);
    h.get_by_label("Toon geschiedenis").click();
    stap(&mut h);
    assert_eq!(h.state().0.aantal_getoond(), 1);
    h.state_mut().0.geschiedenis_open = false;
    stap(&mut h);
    assert_eq!(h.state().0.aantal_getoond(), 0);
}

#[test]
fn wis_knop_maakt_leeg() {
    let mut h = hoofd();
    genereer(&mut h);
    genereer(&mut h);
    open_paneel(&mut h);
    h.get_all_by_label("Toon geschiedenis")
        .next()
        .unwrap()
        .click();
    stap(&mut h);
    h.get_by_label("Wis geschiedenis").click();
    stap(&mut h);
    assert!(h.state().0.geschiedenis.items().is_empty());
    assert_eq!(h.state().0.aantal_getoond(), 0);
    assert!(h.query_by_label("De geschiedenis is leeg.").is_some());
}

#[test]
fn opnieuw_spellen_laadt_het_item_in_het_hoofdvenster() {
    let mut h = hoofd();
    genereer(&mut h);
    let eerste = h.state().0.huidige_tekst().unwrap().to_owned();
    kies_tab(&mut h, Tab::Wachtzin);
    genereer(&mut h);
    kies_tab(&mut h, Tab::Wachtwoord);
    genereer(&mut h);
    assert_ne!(h.state().0.huidige_tekst().unwrap(), eerste);
    open_paneel(&mut h);
    // Het oudste item staat onderaan: het eerste wachtwoord.
    let knoppen = h.get_all_by_label("Opnieuw spellen");
    assert_eq!(knoppen.count(), 3);
    h.get_all_by_label("Opnieuw spellen")
        .last()
        .unwrap()
        .click();
    stap(&mut h);
    assert_eq!(h.state().0.huidige_tekst().unwrap(), eerste);
}

#[test]
fn inactiviteit_tijdens_voorlezen_uit_geschiedenis_crasht_niet() {
    let mut h = hoofd();
    genereer(&mut h);
    h.key_press(Key::F5);
    stap(&mut h);
    assert!(h.state().0.voorlezen.is_some());
    let nu = h.state().1;
    h.state_mut().1 = nu + Duration::from_secs(16 * 60);
    stap(&mut h);
    let app = &h.state().0;
    assert!(app.voorlezen.is_none(), "terug naar het hoofdvenster");
    assert!(app.geschiedenis.items().is_empty());
    assert!(app.gewist_melding);
    assert!(
        h.query_by_label("De geschiedenis is gewist na inactiviteit.")
            .is_some()
    );
}

#[test]
fn inactiviteit_wist_ook_het_huidige_resultaat() {
    let mut h = hoofd();
    genereer(&mut h);
    assert!(h.state().0.huidige_tekst().is_some());
    assert!(h.state().0.hoofd.cache.is_some());
    let nu = h.state().1;
    h.state_mut().1 = nu + Duration::from_secs(16 * 60);
    stap(&mut h);
    let app = &h.state().0;
    assert!(app.huidige_tekst().is_none());
    assert!(app.hoofd.cache.is_none());
    assert!(app.hoofd.wachtwoord.is_none() && app.hoofd.wachtzin.is_none());
    assert!(
        h.query_by_label("De geschiedenis is gewist na inactiviteit.")
            .is_some()
    );
}

#[test]
fn bij_afsluiten_wist_ook_het_huidige_resultaat() {
    let mut h = hoofd();
    genereer(&mut h);
    h.state_mut().0.bij_afsluiten();
    let app = &h.state().0;
    assert!(app.huidige_tekst().is_none());
    assert!(app.hoofd.cache.is_none());
}

#[test]
fn invoer_telt_als_gebruik() {
    let mut h = hoofd();
    genereer(&mut h);
    let t = h.state().1;
    h.state_mut().1 = t + Duration::from_secs(14 * 60);
    h.key_press(Key::A);
    stap(&mut h);
    assert_eq!(h.state().0.geschiedenis.items().len(), 1);
    // 16 minuten na het begin, maar maar 2 na de laatste invoer: nog niet gewist.
    h.state_mut().1 = t + Duration::from_secs(16 * 60);
    stap(&mut h);
    assert_eq!(h.state().0.geschiedenis.items().len(), 1);
    // 15 minuten na de laatste invoer: wel.
    h.state_mut().1 = t + Duration::from_secs(30 * 60);
    stap(&mut h);
    assert!(h.state().0.geschiedenis.items().is_empty());
}

#[test]
fn inactiviteit_sluit_getoonde_items() {
    let mut h = hoofd();
    genereer(&mut h);
    open_paneel(&mut h);
    h.get_by_label("Toon geschiedenis").click();
    stap(&mut h);
    let nu = h.state().1;
    h.state_mut().1 = nu + Duration::from_secs(16 * 60);
    stap(&mut h);
    assert_eq!(h.state().0.aantal_getoond(), 0);
    assert!(h.state().0.gewist_melding);
}

#[test]
fn zonder_time_out_wordt_niet_gewist() {
    let t0 = Instant::now();
    let mut app = PwApp::voor_test(t0);
    app.instellingen.geschiedenis_minuten = None;
    let mut h = harnas_met(app, t0);
    genereer(&mut h);
    let nu = h.state().1;
    h.state_mut().1 = nu + Duration::from_secs(10 * 3600);
    stap(&mut h);
    assert_eq!(h.state().0.geschiedenis.items().len(), 1);
    assert!(!h.state().0.gewist_melding);
}

#[test]
fn gewijzigde_time_out_geldt_direct() {
    let mut h = hoofd();
    genereer(&mut h);
    h.state_mut().0.instellingen.geschiedenis_minuten = Some(1);
    let nu = h.state().1;
    h.state_mut().1 = nu + Duration::from_secs(61);
    stap(&mut h);
    assert!(h.state().0.geschiedenis.items().is_empty());
}

#[test]
fn instellingenpaneel_opent_en_tekent() {
    let mut h = hoofd();
    h.state_mut().0.instellingen_open = true;
    stap(&mut h);
    assert!(h.query_by_label("Klembord automatisch wissen").is_some());
    assert!(h.query_by_label("Taal van het programma").is_some());
    assert!(
        h.query_by_label("Geschiedenis wissen na (minuten)")
            .is_some()
    );
}

#[test]
fn on_exit_wist_geschiedenis_en_volgt_klembordinstelling() {
    for wissen in [true, false] {
        let t0 = Instant::now();
        let bord = NepBord::default();
        let mut app = PwApp::voor_test(t0);
        app.instellingen.klembord_wissen = wissen;
        app.zet_bord(Box::new(bord.clone()));
        let mut h = harnas_met(app, t0);
        genereer(&mut h);
        h.get_by_label("Kopieer").click();
        stap(&mut h);
        assert!(bord.0.borrow().is_some());
        h.state_mut().0.bij_afsluiten();
        assert!(h.state().0.geschiedenis.items().is_empty());
        assert_eq!(bord.0.borrow().is_none(), wissen);
    }
}

#[test]
fn geen_wachtwoord_in_instellingen_of_logboek() {
    let map = std::env::temp_dir().join(format!("pwspeller-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&map);
    let instellingenpad = map.join("instellingen.toml");
    let logpad = map.join("pwspeller.log");
    let t0 = Instant::now();
    let app = PwApp::nieuw_met_paden(instellingenpad.clone(), &logpad, t0);
    let mut h = harnas_met(app, t0);
    // (tekst, alleen als heel woord vergelijken)
    let mut teksten: Vec<(String, bool)> = Vec::new();
    for i in 0..20 {
        kies_tab(
            &mut h,
            if i % 2 == 0 {
                Tab::Wachtwoord
            } else {
                Tab::Wachtzin
            },
        );
        genereer(&mut h);
        let app = &h.state().0;
        let g = if i % 2 == 0 {
            app.hoofd.wachtwoord.as_ref()
        } else {
            app.hoofd.wachtzin.as_ref()
        }
        .expect("gegenereerd");
        teksten.push((g.tekst.to_string(), false));
        if let Some(w) = &g.woorden {
            // Korte woorden alleen als heel woord, anders geven "true" en dergelijke valse treffers.
            teksten.extend(w.iter().map(|w| (w.clone(), w.chars().count() < 6)));
        }
    }
    // Een instelling wijzigen zodat het bestand zeker geschreven wordt.
    h.state_mut().0.instellingen.klembord_seconden = 61;
    stap(&mut h);
    h.state_mut().0.bij_afsluiten();
    drop(h);
    let instellingen = std::fs::read_to_string(&instellingenpad).expect("instellingen bewaard");
    assert!(instellingen.contains("klembord_seconden = 61"));
    let logboek = std::fs::read_to_string(&logpad).expect("logboek bestaat");
    assert!(!logboek.trim().is_empty(), "het logboek is niet leeg");
    // Alleen de waarden van de instellingen tellen; sleutels als `cijfer` zijn geen lek.
    let waarden: String = instellingen
        .lines()
        .filter_map(|r| r.split_once('=').map(|(_, w)| w))
        .collect::<Vec<_>>()
        .join("\n");
    let mut bestanden = vec![("instellingen", waarden), ("logboek", logboek)];
    for extra in ["instellingen.toml.tmp", "instellingen.toml.kapot"] {
        if let Ok(t) = std::fs::read_to_string(map.join(extra)) {
            bestanden.push((extra, t));
        }
    }
    assert!(!teksten.is_empty());
    for (naam, inhoud) in &bestanden {
        for (t, heel) in &teksten {
            assert!(!komt_voor(inhoud, t, *heel), "gegenereerde tekst in {naam}");
        }
    }
    let _ = std::fs::remove_dir_all(&map);
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
