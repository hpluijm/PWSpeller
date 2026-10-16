<p align="center"><img src="docs/assets/petisys-logo.jpeg" alt="Petisys" height="113"></p>

## Over dit bestand

- **Doel:** noemt de AI-tools en de overige rechthebbenden van PWSpeller.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Lettertype Atkinson Hyperlegible Mono (OFL 1.1) toegevoegd (Taak 12). |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Woordenlijsten toegevoegd: EFF-lijst (CC BY 3.0) en de gegenereerde Nederlandse, Franse en Duitse lijsten (Taak 5). |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | NOTICE.md aangemaakt. |

---

# Notice

## AI-ontwikkeltools

Deze code is gebouwd met AI-ontwikkeltools die rechtstreeks in de projectmap en de
git-repository werken: Claude Code (Anthropic) en GitHub Copilot in Visual Studio Code
(GitHub). Welke tool aan een bestand gewerkt heeft, staat in het slotblok van dat bestand.

Waarom zo gewerkt wordt:

- Wijzigingen landen direct in de bestanden en worden vastgelegd in git, zodat elke stap te
  volgen, te reviewen en terug te draaien is.
- Code wordt lokaal gedraaid en getest voordat die gecommit wordt, in plaats van overgenomen
  uit een chatvenster.
- Het hele project (structuur, stijl, bestaande scripts) wordt als context gebruikt, waardoor
  nieuwe code aansluit op wat er al staat.

Eindverantwoordelijkheid voor inhoud, controle en publicatie ligt bij Petisys; de
AI-tools zijn hulpmiddelen, geen auteurs.

## Overige rechthebbenden

- **Woordenlijst Engels (`data/woordenlijsten/en-eff.txt`):** de EFF Large Wordlist for
  Passphrases van de Electronic Frontier Foundation, licentie CC BY 3.0
  (https://creativecommons.org/licenses/by/3.0/), bron
  https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt. De dobbelsteennummers en de
  woorden met een koppelteken zijn eruit gehaald.
- **Woordenlijsten Nederlands, Frans en Duits (`data/woordenlijsten/{nl,fr,de}.txt`) en de
  bijbehorende blokkeerlijsten:** gegenereerd met Claude in opdracht van Petisys.

- **Lettertype Atkinson Hyperlegible Mono (`crates/app/assets/AtkinsonHyperlegibleMono-*.ttf`):**
  Copyright 2020-2024 The Atkinson Hyperlegible Mono Project Authors
  (https://github.com/googlefonts/atkinson-hyperlegible-next-mono), licentie SIL Open Font
  License 1.1 (volledige tekst in `crates/app/assets/OFL.txt`), bron
  https://github.com/google/fonts/tree/main/ofl/atkinsonhyperlegiblemono. De statische
  gewichten Regular (400) en Bold (700) zijn met fonttools uit het variabele lettertype gemaakt.

Verder nog geen meegeleverde bibliotheken. Deze lijst vult zich in latere taken.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
