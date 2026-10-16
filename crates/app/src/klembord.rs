// ── Over dit bestand ──────────────────────────────────────
// Doel:           Het klembord: kopiëren als vertrouwelijk en automatisch wissen na een wachttijd of bij sluiten, alleen als ons wachtwoord er nog op staat.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 15: Bord, Systeembord (arboard) en Klembordbeheer aangemaakt.
// ──────────────────────────────────────────────────────────

use crate::instellingen::Instellingen;
use sha2::{Digest, Sha256};
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

/// Het klembord waar het beheer op werkt. Het echte bord is `Systeembord`; tests gebruiken een nepbord.
pub trait Bord {
    /// Zet tekst op het bord, gemarkeerd als vertrouwelijk waar het platform dat kent.
    fn zet_vertrouwelijk(&mut self, tekst: &str) -> Result<(), String>;
    /// De tekst die nu op het bord staat. De aanroeper wist die zelf.
    fn lees(&mut self) -> Option<String>;
    fn wis(&mut self) -> Result<(), String>;
}

impl Bord for Box<dyn Bord> {
    fn zet_vertrouwelijk(&mut self, tekst: &str) -> Result<(), String> {
        (**self).zet_vertrouwelijk(tekst)
    }
    fn lees(&mut self) -> Option<String> {
        (**self).lees()
    }
    fn wis(&mut self) -> Result<(), String> {
        (**self).wis()
    }
}

/// Het klembord van het besturingssysteem, via arboard.
pub struct Systeembord {
    klembord: arboard::Clipboard,
}

impl Systeembord {
    /// `None` als er geen klembord is (bijvoorbeeld Linux zonder display).
    pub fn nieuw() -> Option<Self> {
        match arboard::Clipboard::new() {
            Ok(klembord) => Some(Self { klembord }),
            Err(e) => {
                log::warn!("systeemklembord niet beschikbaar: {e}");
                None
            }
        }
    }
}

impl Bord for Systeembord {
    fn zet_vertrouwelijk(&mut self, tekst: &str) -> Result<(), String> {
        let zetten = self.klembord.set();
        // macOS: org.nspasteboard.ConcealedType. Windows: buiten de klembordgeschiedenis en de
        // cloudsynchronisatie. Linux (Wayland): de hint voor klembordbeheerders.
        #[cfg(target_os = "macos")]
        let zetten = arboard::SetExtApple::exclude_from_history(zetten);
        #[cfg(windows)]
        let zetten = arboard::SetExtWindows::exclude_from_cloud(
            arboard::SetExtWindows::exclude_from_history(zetten),
        );
        #[cfg(all(
            unix,
            not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
        ))]
        let zetten = arboard::SetExtLinux::exclude_from_history(zetten);
        zetten.text(tekst.to_string()).map_err(|e| e.to_string())
    }

    fn lees(&mut self) -> Option<String> {
        self.klembord.get_text().ok()
    }

    fn wis(&mut self) -> Result<(), String> {
        self.klembord.clear().map_err(|e| e.to_string())
    }
}

fn hash(tekst: &str) -> [u8; 32] {
    Sha256::digest(tekst.as_bytes()).into()
}

/// Bewaakt wat wij op het klembord zetten. Onthoudt alleen een SHA-256, nooit de tekst zelf.
pub struct Klembordbeheer<B: Bord> {
    bord: B,
    hash: Option<[u8; 32]>,
    gezet_op: Option<Instant>,
    mislukt: bool,
}

impl<B: Bord> Klembordbeheer<B> {
    pub fn nieuw(bord: B) -> Self {
        Self {
            bord,
            hash: None,
            gezet_op: None,
            mislukt: false,
        }
    }

    /// Zet de tekst op het bord. Bij een fout: logregel zonder inhoud en `mislukt()` wordt waar.
    pub fn kopieer(&mut self, tekst: &str, nu: Instant) {
        if tekst.is_empty() {
            return;
        }
        match self.bord.zet_vertrouwelijk(tekst) {
            Ok(()) => {
                self.mislukt = false;
                self.hash = Some(hash(tekst));
                self.gezet_op = Some(nu);
            }
            Err(e) => {
                log::warn!("kopiëren naar het klembord mislukt: {e}");
                self.mislukt = true;
                self.hash = None;
                self.gezet_op = None;
            }
        }
    }

    /// Of de laatste klembordhandeling mislukte (voor de melding in de interface).
    pub fn mislukt(&self) -> bool {
        self.mislukt
    }

    /// Het moment waarop gewist wordt, zolang dat openstaat.
    pub fn wis_moment(&self, i: &Instellingen) -> Option<Instant> {
        if !i.klembord_wissen {
            return None;
        }
        Some(self.gezet_op? + Duration::from_secs(u64::from(i.klembord_seconden)))
    }

    /// Wist na de wachttijd, als dat aanstaat en ons wachtwoord er nog op staat.
    pub fn tik(&mut self, nu: Instant, i: &Instellingen) {
        let Some(moment) = self.wis_moment(i) else {
            return;
        };
        if nu < moment {
            return;
        }
        self.wis_als_het_van_ons_is();
        self.gezet_op = None;
    }

    /// Wist bij het sluiten, als dat aanstaat en ons wachtwoord er nog op staat.
    pub fn bij_sluiten(&mut self, i: &Instellingen) {
        if !i.klembord_wissen {
            return;
        }
        self.wis_als_het_van_ons_is();
        self.gezet_op = None;
    }

    fn wis_als_het_van_ons_is(&mut self) {
        let Some(onze_hash) = self.hash.take() else {
            return;
        };
        let huidig = self.bord.lees().map(Zeroizing::new);
        let is_van_ons = huidig.as_ref().is_some_and(|t| hash(t) == onze_hash);
        drop(huidig);
        if is_van_ons && let Err(e) = self.bord.wis() {
            log::warn!("klembord wissen mislukt: {e}");
            self.mislukt = true;
        }
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
