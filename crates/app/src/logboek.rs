// ── Over dit bestand ──────────────────────────────────────
// Doel:           Het logboek van de app: een eenvoudige log-implementatie die regels met tijdstip naar een bestand schrijft.
// Klant:          Petisys
// Gemaakt vanuit: Petisys
// Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
// Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
//
// 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
//   Taak 12: logboek aangemaakt.
// ──────────────────────────────────────────────────────────

use log::{Level, LevelFilter, Log, Metadata, Record};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

struct Logboek {
    bestand: Mutex<File>,
}

impl Log for Logboek {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if let Ok(mut f) = self.bestand.lock() {
            let _ = writeln!(
                f,
                "{} {:<5} {}",
                tijdstip(secs),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {
        if let Ok(mut f) = self.bestand.lock() {
            let _ = f.flush();
        }
    }
}

/// UTC-tijdstip als `JJJJ-MM-DD UU:MM:SSZ`, zonder extra crate.
fn tijdstip(secs: u64) -> String {
    let (dagen, rest) = ((secs / 86_400) as i64, secs % 86_400);
    // Burgerlijke datum uit dagen sinds 1970-01-01 (algoritme van Howard Hinnant).
    let z = dagen + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let jee = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doj = doe - (365 * jee + jee / 4 - jee / 100);
    let mp = (5 * doj + 2) / 153;
    let dag = doj - (153 * mp + 2) / 5 + 1;
    let maand = if mp < 10 { mp + 3 } else { mp - 9 };
    let jaar = jee + era * 400 + i64::from(maand <= 2);
    format!(
        "{jaar:04}-{maand:02}-{dag:02} {:02}:{:02}:{:02}Z",
        rest / 3600,
        rest % 3600 / 60,
        rest % 60
    )
}

/// Start het logboek en schrijft naar `pad` (de map wordt aangemaakt). Hierin komen nooit
/// wachtwoorden of geschiedenis.
pub fn start_logboek(pad: &Path) -> io::Result<()> {
    if let Some(map) = pad.parent() {
        fs::create_dir_all(map)?;
    }
    let bestand = OpenOptions::new().create(true).append(true).open(pad)?;
    log::set_boxed_logger(Box::new(Logboek {
        bestand: Mutex::new(bestand),
    }))
    .map_err(io::Error::other)?;
    log::set_max_level(LevelFilter::Info);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::tijdstip;

    #[test]
    fn tijdstip_klopt() {
        assert_eq!(tijdstip(0), "1970-01-01 00:00:00Z");
        assert_eq!(tijdstip(1_791_936_000 + 3_723), "2026-10-14 01:02:03Z");
        assert_eq!(tijdstip(951_782_400), "2000-02-29 00:00:00Z");
    }
}

// © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
// Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
