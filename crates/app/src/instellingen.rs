// ── Over dit bestand ──────────────────────────────────────
// Doel:           Instellingen van de app: opslaan en laden als TOML, en de keuze van de interfacetaal.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 17 fixronde 1: laad begrenst waarden buiten het bereik (begrens).
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12 fixronde 1: een kapot instellingenbestand wordt eerst hernoemd naar .kapot.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: instellingen, instellingenpad en interfacetaal aangemaakt.
// ──────────────────────────────────────────────────────────

use pwspeller_kern::generator::{WachtwoordOpties, WachtzinOpties};
use pwspeller_kern::talen::catalogus;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::{Path, PathBuf};

/// Het versienummer, bij het bouwen ingevuld door build.rs.
pub const VERSIE: &str = env!("PWSPELLER_VERSIE");

/// De instellingen van de gebruiker. Hierin staan nooit wachtwoorden of geschiedenis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Instellingen {
    /// `None` betekent: volg het besturingssysteem.
    pub interfacetaal: Option<String>,
    pub spellingstaal: String,
    pub wachtwoord: WachtwoordOpties,
    pub wachtzin: WachtzinOpties,
    /// `None` betekent: de woordenlijst volgt de spellingstaal.
    pub woordenlijst: Option<String>,
    pub klembord_wissen: bool,
    pub klembord_seconden: u32,
    /// `None` betekent: de geschiedenis wordt niet automatisch gewist. In TOML staat dat als 0,
    /// want TOML kent geen lege waarde.
    #[serde(with = "nul_is_geen")]
    pub geschiedenis_minuten: Option<u32>,
}

mod nul_is_geen {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(w: &Option<u32>, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u32(w.unwrap_or(0))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u32>, D::Error> {
        Ok(Some(u32::deserialize(d)?).filter(|&m| m > 0))
    }
}

impl Default for Instellingen {
    fn default() -> Self {
        Self {
            interfacetaal: None,
            spellingstaal: "nl".to_string(),
            wachtwoord: WachtwoordOpties::default(),
            wachtzin: WachtzinOpties::default(),
            woordenlijst: None,
            klembord_wissen: !cfg!(target_os = "macos"),
            klembord_seconden: 30,
            geschiedenis_minuten: Some(15),
        }
    }
}

impl Instellingen {
    /// Leest de instellingen. Ontbreekt het bestand of is het kapot, dan komt de
    /// standaard terug; een kapot bestand wordt gelogd.
    pub fn laad(pad: &Path) -> Self {
        let tekst = match std::fs::read_to_string(pad) {
            Ok(t) => t,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Self::default(),
            Err(e) => {
                log::warn!(
                    "instellingen {} onleesbaar: {e}; standaard gebruikt",
                    pad.display()
                );
                return Self::default();
            }
        };
        toml::from_str::<Self>(&tekst)
            .map(Self::begrens)
            .unwrap_or_else(|e| {
                let kapot = pad.with_extension("toml.kapot");
                match std::fs::rename(pad, &kapot) {
                    Ok(()) => log::warn!(
                        "instellingen {} kapot: {e}; bewaard als {}, standaard gebruikt",
                        pad.display(),
                        kapot.display()
                    ),
                    Err(r) => log::warn!(
                        "instellingen {} kapot: {e}; kopie als {} mislukt: {r}; standaard gebruikt",
                        pad.display(),
                        kapot.display()
                    ),
                }
                Self::default()
            })
    }

    /// Brengt waarden die buiten het bereik van de interface liggen terug naar het bereik.
    /// Een handmatig bewerkt bestand kan dus geen onzin of een bijna oneindige tijd opleveren.
    pub fn begrens(mut self) -> Self {
        self.klembord_seconden = self.klembord_seconden.clamp(5, 300);
        self.geschiedenis_minuten = self.geschiedenis_minuten.map(|m| m.clamp(1, 60));
        self.wachtwoord.lengte = self.wachtwoord.lengte.clamp(5, 128);
        self.wachtwoord.min_cijfers = self.wachtwoord.min_cijfers.clamp(1, 9);
        self.wachtwoord.min_speciaal = self.wachtwoord.min_speciaal.clamp(1, 9);
        self.wachtzin.woorden = self.wachtzin.woorden.clamp(3, 20);
        self
    }

    /// Schrijft de instellingen atomisch: eerst een tijdelijk bestand, dan een rename.
    pub fn bewaar(&self, pad: &Path) -> io::Result<()> {
        if let Some(map) = pad.parent() {
            std::fs::create_dir_all(map)?;
        }
        let tekst = toml::to_string_pretty(self).map_err(io::Error::other)?;
        let tijdelijk = pad.with_extension("toml.tmp");
        std::fs::write(&tijdelijk, tekst)?;
        std::fs::rename(&tijdelijk, pad).inspect_err(|_| {
            let _ = std::fs::remove_file(&tijdelijk);
        })
    }
}

fn projectmappen() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("nl", "Petisys", "PWSpeller")
}

/// Plek van het instellingenbestand in de app-map van het besturingssysteem.
pub fn instellingenpad() -> PathBuf {
    projectmappen()
        .map(|d| d.config_dir().join("instellingen.toml"))
        .unwrap_or_else(|| PathBuf::from("instellingen.toml"))
}

/// Plek van het logboek in de datamap van het besturingssysteem.
pub fn logboekpad() -> PathBuf {
    projectmappen()
        .map(|d| d.data_dir().join("pwspeller.log"))
        .unwrap_or_else(|| PathBuf::from("pwspeller.log"))
}

/// De interfacetaal: de gekozen taal, of die van het besturingssysteem, of "en".
pub fn interfacetaal(i: &Instellingen) -> String {
    kies_interfacetaal(i.interfacetaal.as_deref(), sys_locale::get_locale())
}

/// De keuze zelf, met de taal van het besturingssysteem als parameter zodat hij te testen is.
pub fn kies_interfacetaal(gekozen: Option<&str>, systeem: Option<String>) -> String {
    let interface = &catalogus().interface;
    if let Some(t) = gekozen
        && interface.contains_key(t)
    {
        return t.to_owned();
    }
    systeem
        .map(|l| l.chars().take(2).collect::<String>().to_lowercase())
        .filter(|t| interface.contains_key(t.as_str()))
        .unwrap_or_else(|| "en".to_owned())
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
