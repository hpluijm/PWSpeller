// ── Over dit bestand ──────────────────────────────────────
// Doel:           Versleutelde geschiedenis van gegenereerde waarden, alleen in het geheugen.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 18 fixronde: sleutel in een eigen, pagina-uitgelijnde en gelockte pagina (geen gedeelde lock meer).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17 fixronde 1: markeer_gebruik (invoer telt als gebruik).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17: zet_time_out toegevoegd (instellingen wijzigen de time-out van een lopende geschiedenis).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Fixronde 1: inactiviteit ook op de wandklok (slaapstand), ontsleutel controleert eerst.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 11: module aangemaakt (XChaCha20-Poly1305, gelockte sleutel, wissen na inactiviteit).
// ──────────────────────────────────────────────────────────

//! Versleutelde geschiedenis (spec §9). De sleutel bestaat alleen in het geheugen, elk item is
//! apart versleuteld met een eigen nonce, en ontsleutelde tekst staat in `Zeroizing`-buffers.

use crate::generator::{Gegenereerd, Modus};
use chacha20poly1305::{AeadInOut, KeyInit, XChaCha20Poly1305, XNonce};
use rand::Rng;
use std::alloc::{self, Layout};
use std::collections::VecDeque;
use std::fmt;
use std::time::{Duration, Instant, SystemTime};
use zeroize::{Zeroize, Zeroizing};

/// Maximaal aantal items; het oudste valt eraf.
pub const MAX_ITEMS: usize = 100;

const NONCE_LENGTE: usize = 24;
const TAG_LENGTE: usize = 16;

/// Wat zonder ontsleuteling zichtbaar is van een item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemInfo {
    pub id: u64,
    pub modus: Modus,
    pub tijdstip: SystemTime,
    pub spellingstaal: String,
}

struct Item {
    info: ItemInfo,
    nonce: [u8; NONCE_LENGTE],
    ciphertext: Vec<u8>,
}

const SLEUTEL_LENGTE: usize = 32;

/// Een eigen geheugenpagina voor de sleutel, pagina-uitgelijnd en zo groot als één pagina.
///
/// Waarom een eigen pagina: het besturingssysteem vergrendelt per pagina en telt niet mee.
/// Deelden twee sleutels (bijv. twee `Box`-allocaties van 32 bytes) een pagina, dan hief het
/// ontgrendelen van de ene de vergrendeling van de andere op; op Windows faalde de tweede
/// `VirtualUnlock` daarna met ERROR_NOT_LOCKED (158).
struct SleutelPagina {
    ptr: std::ptr::NonNull<u8>,
    layout: Layout,
    gelockt: bool,
}

// SAFETY: de pagina is uitsluitend van deze waarde; er bestaan geen andere verwijzingen naar.
unsafe impl Send for SleutelPagina {}
// SAFETY: gedeelde toegang geeft alleen leestoegang (`&[u8; 32]`).
unsafe impl Sync for SleutelPagina {}

impl SleutelPagina {
    fn nieuw() -> Self {
        let pagina = region::page::size();
        let layout =
            Layout::from_size_align(pagina, pagina).expect("paginagrootte is een macht van twee");
        // SAFETY: de layout heeft een grootte groter dan nul.
        let ruw = unsafe { alloc::alloc_zeroed(layout) };
        let Some(ptr) = std::ptr::NonNull::new(ruw) else {
            alloc::handle_alloc_error(layout);
        };
        let gelockt = match region::lock(ptr.as_ptr().cast_const(), pagina) {
            Ok(guard) => {
                // De guard ontgrendelt bij drop met een debug_assert; dat doen we zelf in `Drop`.
                std::mem::forget(guard);
                true
            }
            Err(fout) => {
                log::warn!("Geheugen met de sleutel kon niet worden vergrendeld: {fout}");
                false
            }
        };
        Self {
            ptr,
            layout,
            gelockt,
        }
    }

    fn sleutel(&self) -> &[u8; SLEUTEL_LENGTE] {
        // SAFETY: de pagina is minstens SLEUTEL_LENGTE bytes, geïnitialiseerd en uitgelijnd.
        unsafe { &*self.ptr.as_ptr().cast::<[u8; SLEUTEL_LENGTE]>() }
    }

    fn sleutel_mut(&mut self) -> &mut [u8; SLEUTEL_LENGTE] {
        // SAFETY: als hierboven, en `&mut self` geeft unieke toegang.
        unsafe { &mut *self.ptr.as_ptr().cast::<[u8; SLEUTEL_LENGTE]>() }
    }
}

impl Drop for SleutelPagina {
    fn drop(&mut self) {
        // Eerst wissen, dan ontgrendelen, dan vrijgeven.
        // SAFETY: de hele pagina is van ons en geïnitialiseerd.
        let alles =
            unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.layout.size()) };
        alles.zeroize();
        if self.gelockt {
            // SAFETY: deze pagina is alleen door ons vergrendeld; niemand anders deelt hem.
            if let Err(fout) =
                unsafe { region::unlock(self.ptr.as_ptr().cast_const(), self.layout.size()) }
            {
                log::warn!("Geheugen met de sleutel kon niet worden ontgrendeld: {fout}");
            }
        }
        // SAFETY: toegewezen met precies deze layout in `nieuw`.
        unsafe { alloc::dealloc(self.ptr.as_ptr(), self.layout) };
    }
}

/// De geschiedenis. De sleutel staat in een eigen, gelockte geheugenpagina.
pub struct Geschiedenis {
    sleutel: SleutelPagina,
    items: VecDeque<Item>,
    volgende_id: u64,
    laatst_gebruikt: Instant,
    /// Wandklok bij hetzelfde moment: `Instant` loopt op macOS en Linux niet door tijdens slaapstand.
    laatst_gebruikt_wand: SystemTime,
    time_out: Option<Duration>,
}

/// Geen sleutel en geen ciphertext in de Debug-uitvoer, alleen het aantal items.
impl fmt::Debug for Geschiedenis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Geschiedenis")
            .field("aantal_items", &self.items.len())
            .finish()
    }
}

impl Geschiedenis {
    /// Maakt een lege geschiedenis met een nieuwe willekeurige sleutel. `time_out` is de duur
    /// van inactiviteit waarna alles gewist wordt; `None` betekent alleen bij sluiten.
    pub fn nieuw(time_out: Option<Duration>) -> Self {
        let mut sleutel = SleutelPagina::nieuw();
        rand::rng().fill_bytes(sleutel.sleutel_mut());
        Self {
            sleutel,
            items: VecDeque::new(),
            volgende_id: 0,
            laatst_gebruikt: Instant::now(),
            laatst_gebruikt_wand: SystemTime::now(),
            time_out,
        }
    }

    fn cipher(&self) -> XChaCha20Poly1305 {
        XChaCha20Poly1305::new_from_slice(self.sleutel.sleutel()).expect("sleutel is 32 bytes")
    }

    /// Versleutelt en bewaart een waarde; geeft het id terug. Is de geschiedenis vol, dan valt
    /// het oudste item eraf.
    pub fn voeg_toe(&mut self, g: &Gegenereerd, spellingstaal: &str, nu: Instant) -> u64 {
        let mut buffer = codeer(g);
        let mut nonce = [0u8; NONCE_LENGTE];
        rand::rng().fill_bytes(&mut nonce);
        self.cipher()
            .encrypt_in_place(&XNonce::from(nonce), b"", &mut *buffer)
            .expect("versleutelen faalt alleen bij een onmogelijk lange invoer");
        let id = self.volgende_id;
        self.volgende_id += 1;
        self.items.push_front(Item {
            info: ItemInfo {
                id,
                modus: g.modus,
                tijdstip: SystemTime::now(),
                spellingstaal: spellingstaal.to_string(),
            },
            nonce,
            ciphertext: buffer.to_vec(),
        });
        self.items.truncate(MAX_ITEMS);
        self.laatst_gebruikt = nu;
        self.laatst_gebruikt_wand = SystemTime::now();
        id
    }

    /// De items, nieuwste eerst, zonder inhoud.
    pub fn items(&self) -> Vec<ItemInfo> {
        self.items.iter().map(|i| i.info.clone()).collect()
    }

    /// Ontsleutelt één item. `None` bij een onbekend id of een beschadigd item. Een geslaagde
    /// ontsleuteling telt als gebruik.
    pub fn ontsleutel(&mut self, id: u64, nu: Instant) -> Option<Gegenereerd> {
        if self.controleer_inactiviteit(nu) {
            return None;
        }
        let item = self.items.iter().find(|i| i.info.id == id)?;
        let mut buffer = Zeroizing::new(Vec::with_capacity(item.ciphertext.len()));
        buffer.extend_from_slice(&item.ciphertext);
        self.cipher()
            .decrypt_in_place(&XNonce::from(item.nonce), b"", &mut *buffer)
            .ok()?;
        let resultaat = decodeer(&buffer)?;
        self.laatst_gebruikt = nu;
        self.laatst_gebruikt_wand = SystemTime::now();
        Some(resultaat)
    }

    /// Wist alle items en vervangt de sleutel door nieuwe willekeurige bytes (de oude wordt
    /// eerst overschreven). Id's worden niet hergebruikt.
    pub fn wis(&mut self) {
        self.items.clear();
        self.sleutel.sleutel_mut().zeroize();
        rand::rng().fill_bytes(self.sleutel.sleutel_mut());
    }

    /// Telt als gebruik: zet de teller van de inactiviteit op `nu` (en de wandklok op nu).
    /// De app roept dit aan bij elke invoer van de gebruiker.
    pub fn markeer_gebruik(&mut self, nu: Instant) {
        self.laatst_gebruikt = nu;
        self.laatst_gebruikt_wand = SystemTime::now();
    }

    /// Wijzigt de time-out van een lopende geschiedenis; de teller van de inactiviteit wordt
    /// niet teruggezet en loopt door vanaf het laatste gebruik.
    pub fn zet_time_out(&mut self, time_out: Option<Duration>) {
        self.time_out = time_out;
    }

    /// Wist als de inactiviteit de time-out heeft bereikt (`>=`), gemeten met `nu` én met de
    /// wandklok. Geeft `true` als er gewist is.
    pub fn controleer_inactiviteit(&mut self, nu: Instant) -> bool {
        self.controleer_inactiviteit_op(nu, SystemTime::now())
    }

    /// Als `controleer_inactiviteit`, met een opgegeven wandklok (testbaar). Een van beide
    /// klokken die de time-out haalt, wist: de wandklok loopt door tijdens slaapstand. Staat
    /// de wandklok achteruit, dan telt alleen `nu`.
    pub fn controleer_inactiviteit_op(&mut self, nu: Instant, wand: SystemTime) -> bool {
        let Some(time_out) = self.time_out else {
            return false;
        };
        if self.items.is_empty() {
            return false;
        }
        let instant_verstreken = nu.saturating_duration_since(self.laatst_gebruikt);
        let wand_verstreken = wand.duration_since(self.laatst_gebruikt_wand).ok();
        if instant_verstreken < time_out && wand_verstreken.is_none_or(|d| d < time_out) {
            return false;
        }
        self.wis();
        true
    }

    #[cfg(test)]
    fn ruwe_bytes(&self, id: u64) -> Option<&[u8]> {
        self.items
            .iter()
            .find(|i| i.info.id == id)
            .map(|i| i.ciphertext.as_slice())
    }
}

impl Drop for Geschiedenis {
    fn drop(&mut self) {
        self.wis();
    }
}

fn modus_byte(m: Modus) -> u8 {
    match m {
        Modus::Wachtwoord => 0,
        Modus::Wachtzin => 1,
    }
}

fn schrijf_lengte(buf: &mut Vec<u8>, n: usize) {
    buf.extend_from_slice(
        &u32::try_from(n)
            .expect("te lang voor de codering")
            .to_le_bytes(),
    );
}

/// Codering: [modus][u32 len][tekst][u8 heeft_wachtzin][u32 len sep][sep][u32 aantal]
/// {[u32 len][woord]}… [f64 entropie]. Er is ruimte voor de tag gereserveerd, zodat de buffer
/// bij het versleutelen niet verhuist (een verhuizing laat een ongewiste kopie achter).
fn codeer(g: &Gegenereerd) -> Zeroizing<Vec<u8>> {
    let woorden: &[String] = g.woorden.as_ref().map_or(&[], |w| w.as_slice());
    let sep = g.scheidingsteken.as_deref().unwrap_or("");
    let grootte = 1
        + 4
        + g.tekst.len()
        + 1
        + 4
        + sep.len()
        + 4
        + woorden.iter().map(|w| 4 + w.len()).sum::<usize>()
        + 8;
    let mut buf = Zeroizing::new(Vec::with_capacity(grootte + TAG_LENGTE));
    buf.push(modus_byte(g.modus));
    schrijf_lengte(&mut buf, g.tekst.len());
    buf.extend_from_slice(g.tekst.as_bytes());
    buf.push(u8::from(g.woorden.is_some() || g.scheidingsteken.is_some()));
    schrijf_lengte(&mut buf, sep.len());
    buf.extend_from_slice(sep.as_bytes());
    schrijf_lengte(&mut buf, woorden.len());
    for w in woorden {
        schrijf_lengte(&mut buf, w.len());
        buf.extend_from_slice(w.as_bytes());
    }
    buf.extend_from_slice(&g.entropie_bits.to_le_bytes());
    buf
}

struct Lezer<'a>(&'a [u8]);

impl<'a> Lezer<'a> {
    fn bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        let (kop, rest) = self.0.split_at_checked(n)?;
        self.0 = rest;
        Some(kop)
    }
    fn byte(&mut self) -> Option<u8> {
        Some(self.bytes(1)?[0])
    }
    fn lengte(&mut self) -> Option<usize> {
        let b: [u8; 4] = self.bytes(4)?.try_into().ok()?;
        usize::try_from(u32::from_le_bytes(b)).ok()
    }
    fn tekst(&mut self) -> Option<&'a str> {
        let n = self.lengte()?;
        std::str::from_utf8(self.bytes(n)?).ok()
    }
}

/// Keert `codeer` om. Elke lengte wordt gecontroleerd; corrupte data geeft `None`.
fn decodeer(data: &[u8]) -> Option<Gegenereerd> {
    let mut l = Lezer(data);
    let modus = match l.byte()? {
        0 => Modus::Wachtwoord,
        1 => Modus::Wachtzin,
        _ => return None,
    };
    let tekst = Zeroizing::new(l.tekst()?.to_string());
    let heeft_wachtzin = match l.byte()? {
        0 => false,
        1 => true,
        _ => return None,
    };
    let sep = l.tekst()?.to_string();
    let aantal = l.lengte()?;
    // Elk woord kost minstens 4 bytes; zo kan een verzonnen aantal geen geheugen opeisen.
    if aantal > l.0.len() / 4 {
        return None;
    }
    let mut woorden = Zeroizing::new(Vec::with_capacity(aantal));
    for _ in 0..aantal {
        woorden.push(l.tekst()?.to_string());
    }
    let entropie: [u8; 8] = l.bytes(8)?.try_into().ok()?;
    if !l.0.is_empty() {
        return None;
    }
    Some(Gegenereerd {
        tekst,
        modus,
        woorden: heeft_wachtzin.then_some(woorden),
        scheidingsteken: heeft_wachtzin.then_some(sep),
        entropie_bits: f64::from_le_bytes(entropie),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn voorbeeld() -> Gegenereerd {
        Gegenereerd {
            tekst: Zeroizing::new("UNIEKE-GEHEIME-TEKST-XYZ".to_string()),
            modus: Modus::Wachtzin,
            woorden: Some(Zeroizing::new(vec!["a".to_string(), "b".to_string()])),
            scheidingsteken: Some("-".to_string()),
            entropie_bits: 12.5,
        }
    }

    #[test]
    fn ciphertext_bevat_de_tekst_niet() {
        let mut h = Geschiedenis::nieuw(None);
        let id = h.voeg_toe(&voorbeeld(), "nl", Instant::now());
        let ruw = h.ruwe_bytes(id).unwrap();
        let naald = b"UNIEKE-GEHEIME-TEKST-XYZ";
        assert!(!ruw.windows(naald.len()).any(|w| w == naald));
        assert_eq!(ruw.len(), codeer(&voorbeeld()).len() + TAG_LENGTE);
    }

    #[test]
    fn gemanipuleerde_ciphertext_geeft_none() {
        let mut h = Geschiedenis::nieuw(None);
        let nu = Instant::now();
        let id = h.voeg_toe(&voorbeeld(), "nl", nu);
        h.items[0].ciphertext[0] ^= 1;
        assert!(h.ontsleutel(id, nu).is_none());
    }

    #[test]
    fn corrupte_codering_geeft_none_en_geen_panic() {
        let goed = codeer(&voorbeeld());
        assert!(decodeer(&goed).is_some());
        for n in 0..goed.len() {
            assert!(decodeer(&goed[..n]).is_none(), "afgekapt op {n}");
        }
        let mut te_lang = goed.to_vec();
        te_lang.push(0);
        assert!(decodeer(&te_lang).is_none());
        let mut slechte_modus = goed.to_vec();
        slechte_modus[0] = 9;
        assert!(decodeer(&slechte_modus).is_none());
        // Verzonnen enorme lengte van de tekst.
        let mut enorm = goed.to_vec();
        enorm[1..5].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(decodeer(&enorm).is_none());
    }

    #[test]
    fn sleutels_staan_elk_op_een_eigen_pagina() {
        let pagina = region::page::size();
        let a = Geschiedenis::nieuw(None);
        let b = Geschiedenis::nieuw(None);
        let pa = a.sleutel.ptr.as_ptr() as usize;
        let pb = b.sleutel.ptr.as_ptr() as usize;
        assert_eq!(pa % pagina, 0);
        assert_eq!(pb % pagina, 0);
        assert_ne!(pa, pb);
    }

    #[test]
    fn meerdere_geschiedenissen_droppen_in_wisselende_volgorde() {
        // Vroeger deelden kleine sleutels een pagina en paniekte de tweede unlock op Windows.
        for volgorde in [[0, 1, 2], [2, 1, 0], [1, 0, 2], [1, 2, 0]] {
            let mut alle: Vec<Option<Geschiedenis>> =
                (0..3).map(|_| Some(Geschiedenis::nieuw(None))).collect();
            let nu = Instant::now();
            for h in alle.iter_mut().flatten() {
                let id = h.voeg_toe(&voorbeeld(), "nl", nu);
                assert!(h.ontsleutel(id, nu).is_some());
            }
            for i in volgorde {
                drop(alle[i].take());
                for h in alle.iter_mut().flatten() {
                    let id = h.voeg_toe(&voorbeeld(), "nl", nu);
                    assert!(h.ontsleutel(id, nu).is_some());
                }
            }
        }
    }

    #[test]
    fn wis_verandert_de_sleutel() {
        let mut h = Geschiedenis::nieuw(None);
        let oud = *h.sleutel.sleutel();
        h.wis();
        assert_ne!(oud, *h.sleutel.sleutel());
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
