<p align="center"><img src="docs/assets/petisys-logo.jpeg" alt="Petisys" height="113"></p>

# PWSpeller

PWSpeller maakt wachtwoorden en wachtzinnen en spelt ze zo dat ze telefonisch foutloos door te
geven zijn. Versie 2 wordt opnieuw gebouwd in Rust voor Windows, macOS en Linux.

## Over dit bestand

- **Doel:** de ingang van het project: wat het is, hoe je het bouwt en waar de rest van de documentatie staat.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | README opnieuw opgezet voor PWSpeller 2 (Rust). |

---

## Wat is het en voor wie

Een programma voor de engineers van Petisys en helpdeskmedewerkers die een wachtwoord aan een
klant doorgeven. De vorige versie (VB.NET, alleen Windows) staat als release
[v1.0-vb](https://github.com/hpluijm/PWSpeller/releases/tag/v1.0-vb).

## Lokaal bouwen

```bash
cargo run -p pwspeller
cargo test --workspace
```

## Uitrollen

Zie de uitrolstappen in het [implementatiedocument](docs/implementatie.md); die komen er in een latere taak.

## Documentatie

- [RELEASENOTES.md](RELEASENOTES.md): wat er voor de gebruiker verandert
- [NOTICE.md](NOTICE.md): AI-tools en overige rechthebbenden
- [docs/implementatie.md](docs/implementatie.md): hoe het in elkaar zit en afgewezen alternatieven
- [docs/voortgang.md](docs/voortgang.md): verloop en beslissingen
- [docs/superpowers/specs/2026-10-16-pwspeller-rust-design.md](docs/superpowers/specs/2026-10-16-pwspeller-rust-design.md): het ontwerp

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

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
