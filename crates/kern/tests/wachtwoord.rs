// ── Over dit bestand ──────────────────────────────────────
// Doel:           Tests van de wachtwoordgenerator: minima, grenzen, uitsluitingen, entropie en verdeling.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
//   Tests aangemaakt voor genereer_wachtwoord.
// ──────────────────────────────────────────────────────────

use pwspeller_kern::generator::wachtwoord::{
    DUBBELZINNIG, SPECIALE_TEKENS, WachtwoordOpties, effectieve_lengte, genereer_wachtwoord,
};
use pwspeller_kern::generator::{GeneratorFout, Modus};
use rand::SeedableRng;
use rand::rngs::StdRng;

fn rng() -> StdRng {
    StdRng::seed_from_u64(7)
}

fn tel(tekst: &str, f: impl Fn(char) -> bool) -> usize {
    tekst.chars().filter(|c| f(*c)).count()
}

#[test]
fn standaard_geeft_14_tekens_met_hoofd_klein_en_cijfer() {
    let mut r = rng();
    for _ in 0..200 {
        let g = genereer_wachtwoord(&WachtwoordOpties::default(), &mut r).unwrap();
        let t: &str = &g.tekst;
        assert_eq!(t.chars().count(), 14);
        assert!(t.chars().any(|c| c.is_ascii_uppercase()));
        assert!(t.chars().any(|c| c.is_ascii_lowercase()));
        assert!(t.chars().any(|c| c.is_ascii_digit()));
        assert_eq!(g.modus, Modus::Wachtwoord);
        assert!(g.woorden.is_none() && g.scheidingsteken.is_none());
    }
}

#[test]
fn standaardwaarden_kloppen() {
    let o = WachtwoordOpties::default();
    assert_eq!(o.lengte, 14);
    assert!(o.hoofdletters && o.kleine_letters && o.cijfers);
    assert!(!o.speciaal && !o.vermijd_dubbelzinnig);
    assert_eq!((o.min_cijfers, o.min_speciaal), (1, 1));
}

#[test]
fn minima_worden_gehaald() {
    let o = WachtwoordOpties {
        lengte: 20,
        speciaal: true,
        min_cijfers: 9,
        min_speciaal: 9,
        ..Default::default()
    };
    let mut r = rng();
    for _ in 0..1000 {
        let g = genereer_wachtwoord(&o, &mut r).unwrap();
        assert_eq!(g.tekst.chars().count(), 20);
        assert!(tel(&g.tekst, |c| c.is_ascii_digit()) >= 9);
        assert!(tel(&g.tekst, |c| SPECIALE_TEKENS.contains(&c)) >= 9);
    }
}

#[test]
fn set_die_aanstaat_komt_minstens_een_keer_voor() {
    let o = WachtwoordOpties {
        speciaal: true,
        min_speciaal: 0,
        min_cijfers: 0,
        ..Default::default()
    };
    let mut r = rng();
    for _ in 0..1000 {
        let g = genereer_wachtwoord(&o, &mut r).unwrap();
        assert!(tel(&g.tekst, |c| SPECIALE_TEKENS.contains(&c)) >= 1);
        assert!(tel(&g.tekst, |c| c.is_ascii_digit()) >= 1);
    }
}

#[test]
fn minima_groter_dan_lengte_verhogen_lengte() {
    let o = WachtwoordOpties {
        lengte: 5,
        speciaal: true,
        min_cijfers: 9,
        min_speciaal: 9,
        ..Default::default()
    };
    // 9 + 9 + 1 hoofdletter + 1 kleine letter: elke set die aanstaat telt mee.
    assert_eq!(effectieve_lengte(&o), 20);
    let g = genereer_wachtwoord(&o, &mut rng()).unwrap();
    assert_eq!(g.tekst.chars().count(), 20);
}

#[test]
fn lengte_onder_5_wordt_5() {
    let o = WachtwoordOpties {
        lengte: 1,
        min_cijfers: 1,
        ..Default::default()
    };
    assert_eq!(effectieve_lengte(&o), 5);
    let g = genereer_wachtwoord(&o, &mut rng()).unwrap();
    assert_eq!(g.tekst.chars().count(), 5);
}

#[test]
fn lengte_boven_128_is_fout() {
    let o = WachtwoordOpties {
        lengte: 129,
        ..Default::default()
    };
    let r = genereer_wachtwoord(&o, &mut rng());
    assert_eq!(r.unwrap_err(), GeneratorFout::OngeldigeLengte);
}

#[test]
fn lengte_128_mag() {
    let o = WachtwoordOpties {
        lengte: 128,
        ..Default::default()
    };
    let g = genereer_wachtwoord(&o, &mut rng()).unwrap();
    assert_eq!(g.tekst.chars().count(), 128);
}

#[test]
fn geen_set_aan_is_fout() {
    let o = WachtwoordOpties {
        hoofdletters: false,
        kleine_letters: false,
        cijfers: false,
        speciaal: false,
        ..Default::default()
    };
    let r = genereer_wachtwoord(&o, &mut rng());
    assert_eq!(r.unwrap_err(), GeneratorFout::GeenTekensetAan);
}

#[test]
fn dubbelzinnig_vermijden_sluit_i_o_l_0_1_uit() {
    let o = WachtwoordOpties {
        lengte: 128,
        speciaal: true,
        vermijd_dubbelzinnig: true,
        ..Default::default()
    };
    let mut r = rng();
    for _ in 0..1000 {
        let g = genereer_wachtwoord(&o, &mut r).unwrap();
        assert!(!g.tekst.chars().any(|c| DUBBELZINNIG.contains(&c)));
    }
}

#[test]
fn dubbelzinnig_vermijden_met_minimum_cijfers_blijft_werken() {
    // Zonder 0 en 1 blijven 8 cijfers over.
    let o = WachtwoordOpties {
        lengte: 9,
        min_cijfers: 9,
        hoofdletters: false,
        kleine_letters: false,
        vermijd_dubbelzinnig: true,
        ..Default::default()
    };
    let g = genereer_wachtwoord(&o, &mut rng()).unwrap();
    assert!(g.tekst.chars().all(|c| ('2'..='9').contains(&c)));
}

#[test]
fn alleen_speciale_tekens_uit_de_acht() {
    let o = WachtwoordOpties {
        lengte: 128,
        hoofdletters: false,
        kleine_letters: false,
        cijfers: false,
        speciaal: true,
        ..Default::default()
    };
    let mut r = rng();
    for _ in 0..100 {
        let g = genereer_wachtwoord(&o, &mut r).unwrap();
        assert!(g.tekst.chars().all(|c| SPECIALE_TEKENS.contains(&c)));
    }
}

#[test]
fn entropie_is_lengte_maal_log2_pool() {
    let g = genereer_wachtwoord(&WachtwoordOpties::default(), &mut rng()).unwrap();
    assert!((g.entropie_bits - 14.0 * 62f64.log2()).abs() < 1e-9);
}

#[test]
fn entropie_telt_dubbelzinnig_en_speciaal_mee() {
    let o = WachtwoordOpties {
        lengte: 10,
        speciaal: true,
        vermijd_dubbelzinnig: true,
        ..Default::default()
    };
    // 62 + 8 = 70, min I, O, l, 0, 1 = 65
    let g = genereer_wachtwoord(&o, &mut rng()).unwrap();
    assert!((g.entropie_bits - 10.0 * 65f64.log2()).abs() < 1e-9);
}

#[test]
fn debug_toont_de_tekst_niet() {
    let g = genereer_wachtwoord(&WachtwoordOpties::default(), &mut rng()).unwrap();
    let s = format!("{g:?}");
    assert!(!s.contains(&*g.tekst));
}

#[test]
fn verdeling_is_gelijkmatig() {
    // 200_000 tekens, alleen kleine letters; chi-kwadraat over 26 vakjes
    // blijft onder 52,6 (p = 0,001 bij 25 vrijheidsgraden). Vaste seed.
    let o = WachtwoordOpties {
        lengte: 100,
        hoofdletters: false,
        cijfers: false,
        ..Default::default()
    };
    let mut r = rng();
    let mut telling = [0u32; 26];
    for _ in 0..2000 {
        let g = genereer_wachtwoord(&o, &mut r).unwrap();
        for c in g.tekst.chars() {
            telling[(c as u8 - b'a') as usize] += 1;
        }
    }
    let totaal: u32 = telling.iter().sum();
    assert_eq!(totaal, 200_000);
    let verwacht = totaal as f64 / 26.0;
    let chi: f64 = telling
        .iter()
        .map(|&n| (n as f64 - verwacht).powi(2) / verwacht)
        .sum();
    assert!(chi < 52.6, "chi-kwadraat {chi}");
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
