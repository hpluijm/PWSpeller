// ── Over dit bestand ──────────────────────────────────────
// Doel:           Tests van de versleutelde geschiedenis in het geheugen.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17: test voor zet_time_out.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fixronde 1: tests voor wandklok en ontsleutelen na time-out.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 11: tests van de geschiedenis.
// ──────────────────────────────────────────────────────────

use pwspeller_kern::generator::{Gegenereerd, Modus};
use pwspeller_kern::geschiedenis::{Geschiedenis, MAX_ITEMS};
use std::time::{Duration, Instant, SystemTime};
use zeroize::Zeroizing;

fn wachtwoord(t: &str) -> Gegenereerd {
    Gegenereerd {
        tekst: Zeroizing::new(t.to_string()),
        modus: Modus::Wachtwoord,
        woorden: None,
        scheidingsteken: None,
        entropie_bits: 77.5,
    }
}

fn wachtzin() -> Gegenereerd {
    Gegenereerd {
        tekst: Zeroizing::new("Appel-peer7-Boom".to_string()),
        modus: Modus::Wachtzin,
        woorden: Some(Zeroizing::new(vec![
            "Appel".to_string(),
            "peer7".to_string(),
            "Boom".to_string(),
        ])),
        scheidingsteken: Some("-".to_string()),
        entropie_bits: 40.25,
    }
}

#[test]
fn toevoegen_en_ontsleutelen_geeft_hetzelfde_terug() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(None);
    let id1 = h.voeg_toe(&wachtwoord("Geheim!123"), "nl", nu);
    let id2 = h.voeg_toe(&wachtzin(), "en", nu);
    let a = h.ontsleutel(id1, nu).unwrap();
    assert_eq!(a.tekst.as_str(), "Geheim!123");
    assert_eq!(a.modus, Modus::Wachtwoord);
    assert!(a.woorden.is_none() && a.scheidingsteken.is_none());
    assert_eq!(a.entropie_bits, 77.5);
    let b = h.ontsleutel(id2, nu).unwrap();
    assert_eq!(b.tekst.as_str(), "Appel-peer7-Boom");
    assert_eq!(b.modus, Modus::Wachtzin);
    assert_eq!(
        b.woorden.as_ref().unwrap().as_slice(),
        ["Appel", "peer7", "Boom"]
    );
    assert_eq!(b.scheidingsteken.as_deref(), Some("-"));
    assert_eq!(b.entropie_bits, 40.25);
    let info = h.items();
    assert_eq!(info[0].id, id2);
    assert_eq!(info[0].spellingstaal, "en");
    assert_eq!(info[1].modus, Modus::Wachtwoord);
}

#[test]
fn honderd_een_items_houdt_de_nieuwste_honderd() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(None);
    let eerste = h.voeg_toe(&wachtwoord("nummer 0"), "nl", nu);
    let mut laatste = 0;
    for i in 1..=MAX_ITEMS {
        laatste = h.voeg_toe(&wachtwoord(&format!("nummer {i}")), "nl", nu);
    }
    assert_eq!(h.items().len(), MAX_ITEMS);
    assert!(h.ontsleutel(eerste, nu).is_none());
    assert_eq!(h.items()[0].id, laatste);
    assert_eq!(
        h.ontsleutel(laatste, nu).unwrap().tekst.as_str(),
        format!("nummer {MAX_ITEMS}")
    );
}

#[test]
fn wis_maakt_leeg_en_oude_id_geeft_none() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(None);
    let id = h.voeg_toe(&wachtwoord("x"), "nl", nu);
    h.wis();
    assert!(h.items().is_empty());
    assert!(h.ontsleutel(id, nu).is_none());
    let nieuw = h.voeg_toe(&wachtwoord("y"), "nl", nu);
    assert_ne!(nieuw, id, "id's worden niet hergebruikt");
    assert_eq!(h.ontsleutel(nieuw, nu).unwrap().tekst.as_str(), "y");
}

#[test]
fn inactiviteit_wist_na_time_out() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(Some(Duration::from_secs(15 * 60)));
    let id = h.voeg_toe(&wachtwoord("x"), "nl", nu);
    assert!(!h.controleer_inactiviteit(nu + Duration::from_secs(14 * 60)));
    assert_eq!(h.items().len(), 1);
    assert!(h.controleer_inactiviteit(nu + Duration::from_secs(15 * 60 + 1)));
    assert!(h.items().is_empty());
    assert!(h.ontsleutel(id, nu).is_none());
}

#[test]
fn precies_op_de_time_out_wordt_gewist() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(Some(Duration::from_secs(60)));
    h.voeg_toe(&wachtwoord("x"), "nl", nu);
    assert!(h.controleer_inactiviteit(nu + Duration::from_secs(60)));
}

#[test]
fn zonder_time_out_wordt_niet_gewist() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(None);
    h.voeg_toe(&wachtwoord("x"), "nl", nu);
    assert!(!h.controleer_inactiviteit(nu + Duration::from_secs(10_000_000)));
    assert_eq!(h.items().len(), 1);
}

#[test]
fn lege_geschiedenis_meldt_niets_gewist() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(Some(Duration::from_secs(1)));
    assert!(!h.controleer_inactiviteit(nu + Duration::from_secs(100)));
}

#[test]
fn ontsleutelen_telt_als_gebruik() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(Some(Duration::from_secs(15 * 60)));
    let id = h.voeg_toe(&wachtwoord("x"), "nl", nu);
    assert!(
        h.ontsleutel(id, nu + Duration::from_secs(10 * 60))
            .is_some()
    );
    assert!(!h.controleer_inactiviteit(nu + Duration::from_secs(20 * 60)));
    assert_eq!(h.items().len(), 1);
}

#[test]
fn onbekend_id_geeft_none() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(None);
    assert!(h.ontsleutel(42, nu).is_none());
}

#[test]
fn debug_toont_geen_inhoud() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(None);
    h.voeg_toe(&wachtwoord("GeheimeTekst"), "nl", nu);
    let d = format!("{h:?}");
    assert!(!d.contains("GeheimeTekst"));
    assert!(d.contains('1'));
}

#[test]
fn wandklok_wist_ook_als_instant_niet_verstreken_is() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(Some(Duration::from_secs(15 * 60)));
    h.voeg_toe(&wachtwoord("x"), "nl", nu);
    let wand = SystemTime::now() + Duration::from_secs(16 * 60);
    assert!(h.controleer_inactiviteit_op(nu, wand));
    assert!(h.items().is_empty());
}

#[test]
fn wandklok_achteruit_geeft_geen_paniek_en_instant_beslist() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(Some(Duration::from_secs(15 * 60)));
    h.voeg_toe(&wachtwoord("x"), "nl", nu);
    let achteruit = SystemTime::now() - Duration::from_secs(3600);
    assert!(!h.controleer_inactiviteit_op(nu + Duration::from_secs(60), achteruit));
    assert_eq!(h.items().len(), 1);
    assert!(h.controleer_inactiviteit_op(nu + Duration::from_secs(15 * 60), achteruit));
    assert!(h.items().is_empty());
}

#[test]
fn ontsleutelen_na_time_out_geeft_none_en_wist() {
    let nu = Instant::now();
    let mut h = Geschiedenis::nieuw(Some(Duration::from_secs(15 * 60)));
    let id = h.voeg_toe(&wachtwoord("x"), "nl", nu);
    assert!(
        h.ontsleutel(id, nu + Duration::from_secs(16 * 60))
            .is_none()
    );
    assert!(h.items().is_empty());
}

#[test]
fn zet_time_out_geldt_voor_een_lopende_geschiedenis() {
    let t0 = Instant::now();
    let mut h = Geschiedenis::nieuw(None);
    h.voeg_toe(&wachtwoord("abc"), "nl", t0);
    assert!(!h.controleer_inactiviteit_op(t0 + Duration::from_secs(7200), SystemTime::now()));
    h.zet_time_out(Some(Duration::from_secs(60)));
    assert!(h.controleer_inactiviteit_op(t0 + Duration::from_secs(61), SystemTime::now()));
    assert!(h.items().is_empty());
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
