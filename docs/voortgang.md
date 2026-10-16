# Voortgang

## Over dit bestand

- **Doel:** legt per werksessie vast wat er veranderd is en waarom, nieuwste bovenaan.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Eerste entry voor PWSpeller 2. |

---

## 2026-10-16 (Claude Code)

- VB-versie bewaard als tag en GitHub-release `v1.0-vb` (commit 79db8f8) en uit de werkmap verwijderd, zodat er één bron is.
- Rust-toolchain geïnstalleerd via Homebrew (`rustup`, stable 1.99.1) en met `sync.sh capture` in mac-setup vastgelegd.
- Werkruimte met drie crates (`pwspeller-data`, `pwspeller-kern`, `pwspeller`), basisdocumenten en de kopbloktest toegevoegd. De test is eerst rood (functies bestonden niet) en daarna groen gemaakt.
- De kopbloktest accepteert ook markdowntabellen `| Wanneer | Wie | Met | Wat |` en slaat `target/`, `.git/`, `.superpowers/` en `.txt` over.
- Gemeten: `rustup-init` staat niet los op het pad na `brew install rustup`; `rustup default stable` installeert de toolchain direct en volstaat.

## Openstaande punten

- Taken 3 en verder uit het implementatieplan.
- Ondertekenen en app stores: zie het ontwerp, hoofdstuk 14.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
