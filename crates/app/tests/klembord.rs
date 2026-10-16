// ── Over dit bestand ──────────────────────────────────────
// Doel:           Test het automatisch wissen van het klembord met een nep-bord (nooit het echte systeemklembord).
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 15: tests voor Klembordbeheer aangemaakt.
// ──────────────────────────────────────────────────────────

use pwspeller::app::PwApp;
use pwspeller::hoofd::Tab;
use pwspeller::instellingen::Instellingen;
use pwspeller::klembord::{Bord, Klembordbeheer};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Default)]
struct Toestand {
    inhoud: Option<String>,
    aanroepen: usize,
    faal: bool,
}

#[derive(Clone, Default)]
struct NepBord(Rc<RefCell<Toestand>>);

impl Bord for NepBord {
    fn zet_vertrouwelijk(&mut self, tekst: &str) -> Result<(), String> {
        let mut t = self.0.borrow_mut();
        t.aanroepen += 1;
        if t.faal {
            return Err("nep".to_string());
        }
        t.inhoud = Some(tekst.to_string());
        Ok(())
    }
    fn lees(&mut self) -> Option<String> {
        let mut t = self.0.borrow_mut();
        t.aanroepen += 1;
        t.inhoud.clone()
    }
    fn wis(&mut self) -> Result<(), String> {
        let mut t = self.0.borrow_mut();
        t.aanroepen += 1;
        t.inhoud = None;
        Ok(())
    }
}

fn aan(seconden: u32) -> Instellingen {
    Instellingen {
        klembord_wissen: true,
        klembord_seconden: seconden,
        ..Instellingen::default()
    }
}

fn uit() -> Instellingen {
    Instellingen {
        klembord_wissen: false,
        ..Instellingen::default()
    }
}

#[test]
fn wist_na_30_seconden_als_ingeschakeld() {
    let bord = NepBord::default();
    let mut beheer = Klembordbeheer::nieuw(bord.clone());
    let begin = Instant::now();
    beheer.kopieer("geheim-wachtwoord", begin);
    assert_eq!(bord.0.borrow().inhoud.as_deref(), Some("geheim-wachtwoord"));
    beheer.tik(begin + Duration::from_secs(29), &aan(30));
    assert!(bord.0.borrow().inhoud.is_some(), "nog niet gewist na 29 s");
    beheer.tik(begin + Duration::from_secs(30), &aan(30));
    assert!(bord.0.borrow().inhoud.is_none(), "gewist na 30 s");
}

#[test]
fn wist_niet_als_gebruiker_iets_anders_kopieerde() {
    let bord = NepBord::default();
    let mut beheer = Klembordbeheer::nieuw(bord.clone());
    let begin = Instant::now();
    beheer.kopieer("geheim-wachtwoord", begin);
    bord.0.borrow_mut().inhoud = Some("iets anders".to_string());
    beheer.tik(begin + Duration::from_secs(31), &aan(30));
    assert_eq!(bord.0.borrow().inhoud.as_deref(), Some("iets anders"));
    assert!(beheer.wis_moment(&aan(30)).is_none(), "wacht niet meer");
}

#[test]
fn wist_niets_als_uitgeschakeld() {
    let bord = NepBord::default();
    let mut beheer = Klembordbeheer::nieuw(bord.clone());
    let begin = Instant::now();
    beheer.kopieer("geheim-wachtwoord", begin);
    let na_kopieer = bord.0.borrow().aanroepen;
    beheer.tik(begin + Duration::from_secs(120), &uit());
    beheer.bij_sluiten(&uit());
    let t = bord.0.borrow();
    assert_eq!(t.aanroepen, na_kopieer, "geen enkele aanroep op het bord");
    assert_eq!(t.inhoud.as_deref(), Some("geheim-wachtwoord"));
}

#[test]
fn bij_sluiten_wist_alleen_ons_wachtwoord() {
    let bord = NepBord::default();
    let mut beheer = Klembordbeheer::nieuw(bord.clone());
    let begin = Instant::now();
    beheer.kopieer("geheim-wachtwoord", begin);
    beheer.bij_sluiten(&aan(30));
    assert!(bord.0.borrow().inhoud.is_none(), "ons wachtwoord is gewist");

    beheer.kopieer("geheim-wachtwoord", begin);
    bord.0.borrow_mut().inhoud = Some("van de gebruiker".to_string());
    beheer.bij_sluiten(&aan(30));
    assert_eq!(bord.0.borrow().inhoud.as_deref(), Some("van de gebruiker"));
}

#[test]
fn mislukte_kopie_meldt_en_onthoudt_niets() {
    let bord = NepBord::default();
    bord.0.borrow_mut().faal = true;
    let mut beheer = Klembordbeheer::nieuw(bord.clone());
    let begin = Instant::now();
    beheer.kopieer("geheim-wachtwoord", begin);
    assert!(beheer.mislukt());
    assert!(beheer.wis_moment(&aan(30)).is_none());
}

#[test]
fn wis_moment_staat_open_tot_gewist() {
    let bord = NepBord::default();
    let mut beheer = Klembordbeheer::nieuw(bord);
    let begin = Instant::now();
    assert!(beheer.wis_moment(&aan(30)).is_none());
    beheer.kopieer("x1", begin);
    assert_eq!(
        beheer.wis_moment(&aan(30)),
        Some(begin + Duration::from_secs(30))
    );
    assert!(beheer.wis_moment(&uit()).is_none());
}

#[test]
fn app_kopieert_huidige_tekst_via_het_verzoek() {
    let nu = Instant::now();
    let mut app = PwApp::voor_test(nu);
    let bord = NepBord::default();
    app.zet_bord(Box::new(bord.clone()));
    app.hoofd.tab = Tab::EigenTekst;
    app.hoofd.eigen_tekst = "Ab3-kleine-test".to_string();
    let tekst = app.huidige_tekst().expect("er is een tekst").to_string();
    app.kopieer_verzoek();
    app.klembord_stap(nu);
    assert_eq!(bord.0.borrow().inhoud.as_deref(), Some(tekst.as_str()));
}

#[test]
fn app_sluiten_met_wissen_uit_raakt_bord_niet_aan() {
    let nu = Instant::now();
    let mut app = PwApp::voor_test(nu);
    app.instellingen.klembord_wissen = false;
    let bord = NepBord::default();
    app.zet_bord(Box::new(bord.clone()));
    app.hoofd.tab = Tab::EigenTekst;
    app.hoofd.eigen_tekst = "Ab3-kleine-test".to_string();
    app.kopieer_verzoek();
    app.klembord_stap(nu);
    let n = bord.0.borrow().aanroepen;
    app.bij_sluiten_klembord();
    assert_eq!(bord.0.borrow().aanroepen, n);
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
