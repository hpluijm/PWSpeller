// ── Over dit bestand ──────────────────────────────────────
// Doel:           Tests van de wachtzingenerator: aantallen, scheidingsteken, hoofdletters, cijfer en entropie.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Tests aangemaakt voor genereer_wachtzin.
// ──────────────────────────────────────────────────────────

use pwspeller_kern::generator::wachtzin::{
    WachtzinOpties, genereer_wachtzin, genereer_wachtzin_os,
};
use pwspeller_kern::generator::{GeneratorFout, Modus};
use pwspeller_kern::sterkte::wachtzin_bits;
use pwspeller_kern::talen::{Woordenlijst, catalogus};
use rand::SeedableRng;
use rand::rngs::StdRng;

fn rng() -> StdRng {
    StdRng::seed_from_u64(11)
}

fn lijst() -> Woordenlijst {
    Woordenlijst {
        code: "test".into(),
        woorden: (0..50u8)
            .map(|i| {
                format!(
                    "tafel{}{}",
                    (b'a' + i / 10) as char,
                    (b'a' + i % 10) as char
                )
            })
            .collect(),
    }
}

fn opties(woorden: u8) -> WachtzinOpties {
    WachtzinOpties {
        woorden,
        ..WachtzinOpties::default()
    }
}

#[test]
fn standaardwaarden_kloppen() {
    let o = WachtzinOpties::default();
    assert_eq!(o.woorden, 6);
    assert_eq!(o.scheidingsteken, "-");
    assert!(!o.hoofdletters && !o.cijfer);
}

#[test]
fn standaard_zes_woorden_met_streepje() {
    let l = lijst();
    let g = genereer_wachtzin(&WachtzinOpties::default(), &l, &mut rng()).unwrap();
    let w = g.woorden.as_ref().unwrap();
    assert_eq!(w.len(), 6);
    assert_eq!(&*g.tekst, &w.join("-"));
    assert_eq!(g.modus, Modus::Wachtzin);
    assert_eq!(g.scheidingsteken.as_deref(), Some("-"));
    assert!(w.iter().all(|x| l.woorden.contains(x)));
}

#[test]
fn twee_en_eenentwintig_woorden_zijn_fout() {
    let l = lijst();
    for n in [0u8, 2, 21, 255] {
        assert_eq!(
            genereer_wachtzin(&opties(n), &l, &mut rng()).unwrap_err(),
            GeneratorFout::OngeldigAantalWoorden
        );
    }
    for n in [3u8, 20] {
        assert!(genereer_wachtzin(&opties(n), &l, &mut rng()).is_ok());
    }
}

#[test]
fn leeg_scheidingsteken_is_fout() {
    let o = WachtzinOpties {
        scheidingsteken: String::new(),
        ..WachtzinOpties::default()
    };
    assert_eq!(
        genereer_wachtzin(&o, &lijst(), &mut rng()).unwrap_err(),
        GeneratorFout::LeegScheidingsteken
    );
}

#[test]
fn lege_woordenlijst_is_fout() {
    let leeg = Woordenlijst {
        code: "leeg".into(),
        woorden: vec![],
    };
    assert_eq!(
        genereer_wachtzin(&WachtzinOpties::default(), &leeg, &mut rng()).unwrap_err(),
        GeneratorFout::LegeWoordenlijst
    );
    assert!(!GeneratorFout::LegeWoordenlijst.to_string().is_empty());
}

#[test]
fn hoofdletters_maken_eerste_letter_groot() {
    let o = WachtzinOpties {
        hoofdletters: true,
        ..WachtzinOpties::default()
    };
    let g = genereer_wachtzin(&o, &lijst(), &mut rng()).unwrap();
    for w in g.woorden.as_ref().unwrap().iter() {
        let mut c = w.chars();
        assert!(c.next().unwrap().is_ascii_uppercase());
        assert!(c.all(|x| x.is_ascii_lowercase()));
    }
}

#[test]
fn cijfer_staat_achter_precies_een_woord() {
    let o = WachtzinOpties {
        cijfer: true,
        ..WachtzinOpties::default()
    };
    let mut r = rng();
    let mut posities = std::collections::BTreeSet::new();
    for _ in 0..300 {
        let g = genereer_wachtzin(&o, &lijst(), &mut r).unwrap();
        let w = g.woorden.as_ref().unwrap();
        assert_eq!(w.len(), 6);
        let n = w
            .iter()
            .filter(|x| x.ends_with(|c: char| c.is_ascii_digit()))
            .count();
        assert_eq!(n, 1);
        posities.insert(
            w.iter()
                .position(|x| x.ends_with(|c: char| c.is_ascii_digit()))
                .unwrap(),
        );
        assert_eq!(&*g.tekst, &w.join("-"));
    }
    assert_eq!(posities.len(), 6, "elke positie komt voor");
}

#[test]
fn scheidingsteken_7_of_dubbel_streepje_blijft_in_woorden_gescheiden() {
    for sep in ["7", "--"] {
        let o = WachtzinOpties {
            scheidingsteken: sep.into(),
            cijfer: true,
            ..WachtzinOpties::default()
        };
        let g = genereer_wachtzin(&o, &lijst(), &mut rng()).unwrap();
        let w = g.woorden.as_ref().unwrap();
        assert_eq!(w.len(), 6);
        assert!(w.iter().all(|x| !x.contains(sep)));
        assert_eq!(&*g.tekst, &w.join(sep));
        assert_eq!(g.scheidingsteken.as_deref(), Some(sep));
    }
}

#[test]
fn entropie_zonder_en_met_cijfer() {
    let l = catalogus().woordenlijsten["en-eff"].clone();
    let n = l.woorden.len();
    let g = genereer_wachtzin(&WachtzinOpties::default(), &l, &mut rng()).unwrap();
    assert!((g.entropie_bits - 6.0 * (n as f64).log2()).abs() < 1e-9);
    let o = WachtzinOpties {
        cijfer: true,
        ..WachtzinOpties::default()
    };
    let g = genereer_wachtzin(&o, &l, &mut rng()).unwrap();
    assert!((g.entropie_bits - (6.0 * (n as f64).log2() + 60f64.log2())).abs() < 1e-9);
    assert!((g.entropie_bits - wachtzin_bits(6, n, true)).abs() < 1e-9);
}

#[test]
fn os_bron_geeft_een_wachtzin() {
    let g = genereer_wachtzin_os(&WachtzinOpties::default(), &lijst()).unwrap();
    assert_eq!(g.woorden.as_ref().unwrap().len(), 6);
}

#[test]
fn debug_toont_de_woorden_niet() {
    let g = genereer_wachtzin(&WachtzinOpties::default(), &lijst(), &mut rng()).unwrap();
    let d = format!("{g:?}");
    assert!(!d.contains(&g.woorden.as_ref().unwrap()[0]));
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
