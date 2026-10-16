// ── Over dit bestand ──────────────────────────────────────
// Doel:           De app zelf: het PwApp-struct dat de instellingen en de interfacetaal bewaart en het hoofdvenster tekent.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17 fixronde 1: bij_afsluiten wist ook het huidige resultaat.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17: panelen, wissen na inactiviteit, on_exit/bij_afsluiten en nieuw_met_paden.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 16: voorleesmodus als eigen weergave (veld voorlezen, opent op het verzoek uit het hoofdvenster).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 15: klembord aangesloten (kopieer-verzoek, wissen per beeld, bij_sluiten_klembord).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 14: hoofdvenster ingebouwd; PwApp kreeg hoofdstate en geschiedenis; instellingenpad is optioneel (tests schrijven niet).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13 fixronde 1: nieuw() krijgt de geladen instellingen; logo vrijgegeven na de splash; tijdelijk titel in het hoofdvenster.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 13: fase Splash/Hoofd en teken(); nieuw() kreeg een CreationContext en een tijdstip.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: PwApp aangemaakt met een leeg hoofdvlak.
// ──────────────────────────────────────────────────────────

use crate::hoofd::HoofdState;
use crate::instellingen::{Instellingen, interfacetaal};
use crate::klembord::{Bord, Klembordbeheer, Systeembord};
use crate::splash::{self, Fase, volgende_fase};
use crate::thema;
use crate::voorlezen::{Actie, Voorlezen};
use eframe::egui;
use pwspeller_kern::generator::Gegenereerd;
use pwspeller_kern::geschiedenis::Geschiedenis;
use std::path::Path;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

pub struct PwApp {
    pub instellingen: Instellingen,
    /// `None` betekent: instellingen worden niet bewaard (tests).
    pub instellingenpad: Option<PathBuf>,
    pub interfacetaal: String,
    pub fase: Fase,
    /// De instellingen zoals ze het laatst bewaard zijn; een verschil betekent: opnieuw bewaren.
    pub(crate) bewaard: Instellingen,
    pub hoofd: HoofdState,
    pub geschiedenis: Geschiedenis,
    /// De voorleesmodus; als die open is, wordt het hoofdvenster niet getekend.
    pub voorlezen: Option<Voorlezen>,
    /// Het geschiedenispaneel en het instellingenpaneel staan open.
    pub geschiedenis_open: bool,
    pub instellingen_open: bool,
    /// De geschiedenis is na inactiviteit gewist; een korte melding tot de volgende handeling.
    pub gewist_melding: bool,
    /// Items uit de geschiedenis waarvan de tekst nu ontsleuteld in beeld staat.
    pub(crate) getoond: Vec<(u64, Gegenereerd)>,
    /// Een kopieerverzoek uit het geschiedenispaneel, voor één keer.
    pub(crate) extra_kopie: Option<Zeroizing<String>>,
    /// `None` als er geen systeemklembord is (of in tests).
    klembord: Option<Klembordbeheer<Box<dyn Bord>>>,
    logo: Option<egui::TextureHandle>,
    logo_geprobeerd: bool,
}

impl PwApp {
    /// Maakt de app. Met een `CreationContext` (de echte start) worden ook thema en lettertype gezet.
    pub fn nieuw(
        cc: Option<&eframe::CreationContext>,
        nu: Instant,
        instellingen: Instellingen,
        instellingenpad: PathBuf,
    ) -> Self {
        if let Some(cc) = cc {
            thema::zet_thema(&cc.egui_ctx);
        }
        let mut app = Self::van(instellingen, Some(instellingenpad), nu);
        app.klembord =
            Systeembord::nieuw().map(|b| Klembordbeheer::nieuw(Box::new(b) as Box<dyn Bord>));
        app
    }

    /// App voor tests: standaardinstellingen, Nederlandse interface en een pad dat nooit beschreven wordt.
    pub fn voor_test(nu: Instant) -> Self {
        let instellingen = Instellingen {
            interfacetaal: Some("nl".to_string()),
            ..Instellingen::default()
        };
        Self::van(instellingen, None, nu)
    }

    /// App voor tests van wat er op schijf komt: eigen instellingenpad en eigen logboek,
    /// zonder systeemklembord.
    pub fn nieuw_met_paden(instellingenpad: PathBuf, logpad: &Path, nu: Instant) -> Self {
        if let Err(e) = crate::logboek::start_logboek(logpad) {
            eprintln!("logboek niet gestart: {e}");
        }
        let instellingen = Instellingen {
            interfacetaal: Some("nl".to_string()),
            ..Instellingen::default()
        };
        Self::van(instellingen, Some(instellingenpad), nu)
    }

    fn van(instellingen: Instellingen, instellingenpad: Option<PathBuf>, nu: Instant) -> Self {
        let interfacetaal = interfacetaal(&instellingen);
        let time_out = instellingen
            .geschiedenis_minuten
            .map(|m| Duration::from_secs(u64::from(m) * 60));
        Self {
            bewaard: instellingen.clone(),
            instellingen,
            instellingenpad,
            interfacetaal,
            fase: Fase::Splash { start: nu },
            hoofd: HoofdState::default(),
            geschiedenis: Geschiedenis::nieuw(time_out),
            voorlezen: None,
            geschiedenis_open: false,
            instellingen_open: false,
            gewist_melding: false,
            getoond: Vec::new(),
            extra_kopie: None,
            klembord: None,
            logo: None,
            logo_geprobeerd: false,
        }
    }

    /// Vervangt het klembord (tests gebruiken een nepbord).
    pub fn zet_bord(&mut self, bord: Box<dyn Bord>) {
        self.klembord = Some(Klembordbeheer::nieuw(bord));
    }

    /// Eén klembordstap per beeld: kopieerverzoek afhandelen, wissen als het moment er is.
    /// Geeft het moment terug waarop opnieuw getekend moet worden om te wissen.
    pub fn klembord_stap(&mut self, nu: Instant) -> Option<Instant> {
        let extra = self.neem_extra_kopie();
        let gevraagd = self.neem_kopieer_verzoek() || extra.is_some();
        let tekst = if extra.is_some() {
            extra
        } else if gevraagd {
            self.huidige_tekst().map(|t| Zeroizing::new(t.to_owned()))
        } else {
            None
        };
        let Some(beheer) = self.klembord.as_mut() else {
            if gevraagd {
                self.hoofd.klembord_mislukt = true;
            }
            return None;
        };
        if let Some(tekst) = tekst {
            beheer.kopieer(&tekst, nu);
        }
        beheer.tik(nu, &self.instellingen);
        self.hoofd.klembord_mislukt = beheer.mislukt();
        beheer.wis_moment(&self.instellingen)
    }

    /// Wist alles wat bij het sluiten weg moet: geschiedenis, ontsleutelde items, voorleesmodus
    /// en (volgens de instelling) het klembord. `on_exit` roept dit aan.
    pub fn bij_afsluiten(&mut self) {
        self.geschiedenis.wis();
        self.getoond.clear();
        self.extra_kopie = None;
        self.wis_resultaat();
        self.bij_sluiten_klembord();
        log::info!("afgesloten");
    }

    /// Wist het klembord bij het sluiten, volgens de instelling.
    pub fn bij_sluiten_klembord(&mut self) {
        if let Some(beheer) = self.klembord.as_mut() {
            beheer.bij_sluiten(&self.instellingen);
        }
    }

    /// Zet de logo-textuur klaar (of laat hem weg als het laden mislukt) en tekent het splashscreen.
    pub fn zet_logo(&mut self, logo: Option<egui::TextureHandle>) {
        self.logo = logo;
        self.logo_geprobeerd = true;
    }

    /// Tekent één beeld: het splashscreen of het hoofdvenster.
    pub fn teken(&mut self, ui: &mut egui::Ui, nu: Instant) {
        self.fase = volgende_fase(&self.fase, nu);
        match self.fase {
            Fase::Splash { start } => {
                if !self.logo_geprobeerd {
                    let logo = splash::laad_logo(ui.ctx());
                    self.zet_logo(logo);
                }
                splash::teken(ui, &self.interfacetaal, self.logo.as_ref(), start, nu);
            }
            Fase::Hoofd => {
                // De textuur is niet meer nodig.
                self.logo = None;
                self.teken_hoofd_of_voorlezen(ui, nu);
            }
        }
    }
}

impl PwApp {
    /// De voorleesmodus als die open is, anders het hoofdvenster; een verzoek om voor te lezen opent hem.
    fn teken_hoofd_of_voorlezen(&mut self, ui: &mut egui::Ui, nu: Instant) {
        // Ook tijdens de voorleesmodus: daar wordt het hoofdvenster niet getekend.
        self.tik_geschiedenis(ui.ctx(), nu);
        if let Some(v) = self.voorlezen.as_mut() {
            if v.teken(ui, &self.interfacetaal) == Actie::Stop {
                // Drop wist de regels.
                self.voorlezen = None;
                ui.ctx().request_repaint();
            }
            return;
        }
        self.teken_hoofd(ui, nu);
        if std::mem::take(&mut self.hoofd.voorlezen_gevraagd) {
            self.open_voorlezen(ui.ctx());
        }
    }

    /// Opent de voorleesmodus met de regels uit de cache; zonder regels gebeurt er niets.
    fn open_voorlezen(&mut self, ctx: &egui::Context) {
        let per_woord = self.hoofd.tab == crate::hoofd::Tab::Wachtzin && !self.hoofd.per_teken;
        // Groepjes van vier bestaan alleen bij een gegenereerd wachtwoord.
        let wachtwoord = self.hoofd.tab == crate::hoofd::Tab::Wachtwoord;
        let Some(cache) = self.hoofd.cache.as_ref() else {
            return;
        };
        let regels = cache.regels.clone();
        let Some(tekst) = self.huidige_tekst() else {
            return;
        };
        self.voorlezen = Voorlezen::nieuw(regels, tekst, per_woord, !wachtwoord);
        if self.voorlezen.is_some() {
            // Een knop die focus had (Tab-toets) bestaat in dit beeld niet meer.
            ctx.memory_mut(|m| {
                if let Some(id) = m.focused() {
                    m.surrender_focus(id);
                }
            });
            ctx.request_repaint();
        }
    }
}

impl eframe::App for PwApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.teken(ui, Instant::now());
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.bij_afsluiten();
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
