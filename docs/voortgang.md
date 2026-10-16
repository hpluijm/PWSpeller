## Over dit bestand

- **Doel:** legt per werksessie vast wat er veranderd is en waarom, nieuwste bovenaan.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Taak 3–5 en de overdracht naar de andere Mac. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Eerste entry voor PWSpeller 2. |

---

# Voortgang

## 2026-10-16, later (Claude Code) — taak 3 t/m 5, overdracht

- Taak 3: `pwspeller-data` leest en controleert alle databestanden. De review dwong een strengere
  controle af: exacte sleutels (geen `Q` naast `q`), geen lege teksten, geen verweesde
  blokkeerlijst, `interface/en.toml` verplicht, blokkeren hoofdletterongevoelig, en geen twee
  tekens met dezelfde omschrijving binnen een taal.
- Taak 4: alfabetten (nl en vl uit de VB-tabel, fr na 1939, de volgens DIN 5009:2022, navo volgens
  ICAO) en omschrijvingen in vier talen. Gemeten: `talen.csv` van de VB-versie is cp850, niet
  cp1252; alleen zo komen "Desiré" en "Quotiënt" goed door. "Urecht" in de VB-tabel was een
  typefout voor "Utrecht".
- Taak 5: woordenlijsten en blokkeerlijsten (nl 7.527/473, fr 5.511/232, de 6.278/354, EFF 7.772).
  De drempeltests (nl ≥ 7.500, fr en de ≥ 5.500) begrenzen hoe streng er geblokkeerd kan worden;
  een paar milde woorden zijn daarom teruggezet. De originelen staan in `docs/bronnen/woordenlijsten/`.
- Werk overgedragen naar de andere Mac. mac-setup zet daar nu ook de Rust-toolchain en het PATH.
  Het uitvoeringslogboek staat in `docs/superpowers/plans/2026-10-16-pwspeller-rust-logboek.md`.

## 2026-10-16 (Claude Code)

- VB-versie bewaard als tag en GitHub-release `v1.0-vb` (commit 79db8f8) en uit de werkmap verwijderd, zodat er één bron is.
- Rust-toolchain geïnstalleerd via Homebrew (`rustup`, stable 1.99.1) en met `sync.sh capture` in mac-setup vastgelegd.
- Werkruimte met drie crates (`pwspeller-data`, `pwspeller-kern`, `pwspeller`), basisdocumenten en de kopbloktest toegevoegd. De test is eerst rood (functies bestonden niet) en daarna groen gemaakt.
- De kopbloktest accepteert ook markdowntabellen `| Wanneer | Wie | Met | Wat |` en slaat `target/`, `.git/`, `.superpowers/` en `.txt` over.
- Gemeten: `rustup-init` staat niet los op het pad na `brew install rustup`; `rustup default stable` installeert de toolchain direct en volstaat.

## Openstaande punten

- Review van Taak 5, daarna Taak 10, 6, 7, 8, 9 en 11 (rest van fase B), dan fase C en D.
- Hans keurt de blokkeerlijsten `nl-`, `fr-` en `de-geblokkeerd.txt` goed.
- Ondertekenen en app stores: zie het ontwerp, hoofdstuk 14.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
