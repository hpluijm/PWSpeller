<p align="center"><img src="docs/assets/petisys-logo.jpeg" alt="Petisys" height="113"></p>

# PWSpeller

PWSpeller maakt wachtwoorden en wachtzinnen en spelt ze zo dat ze telefonisch foutloos door te
geven zijn. Versie 2 is opnieuw gebouwd in Rust voor Windows, macOS en Linux.

## Over dit bestand

- **Doel:** de ingang van het project: wat het is, hoe je het bouwt en waar de rest van de documentatie staat.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Eindreview: F5 opent de voorleesmodus (niet genereren); Linux-minimum Ubuntu 22.04. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 19: README compleet voor v2.0.0 (gebruik, eerste start, taal toevoegen, uitrollen, alle documenten). |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 18 fixronde: release controleert eerst, releasenotes-sectie verplicht. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 18: sectie Uitrollen/downloaden. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Verwijzingen naar plan, logboek en bronnen van de woordenlijsten. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | README opnieuw opgezet voor PWSpeller 2 (Rust). |

---

## Wat is het en voor wie

Een programma voor de engineers van Petisys en voor helpdeskmedewerkers die een wachtwoord aan een
klant doorgeven. Het maakt een wachtwoord, een wachtzin of spelt een eigen tekst, en toont de
spelling in de taal van de klant ("A van Anton, kleine letter b van Bernard, cijfer 4"). Het
draait op Windows (x64 en ARM64), macOS (Apple en Intel) en Linux (x64), gebruikt geen netwerk en
bewaart geen wachtwoorden op schijf. De vorige versie (VB.NET, alleen Windows) staat als release
[v1.0-vb](https://github.com/hpluijm/PWSpeller/releases/tag/v1.0-vb).

## Downloaden

Elke release op [GitHub Releases](https://github.com/hpluijm/PWSpeller/releases) bevat:

| Platform | Bestand |
|---|---|
| Windows x64 | `PWSpeller-<versie>-windows-x64.exe` |
| Windows ARM64 | `PWSpeller-<versie>-windows-arm64.exe` |
| macOS (Apple en Intel) | `PWSpeller-<versie>-macos.zip`, met daarin `PWSpeller.app` |
| Linux x64 | `pwspeller-<versie>-linux-x64` (`chmod +x`, daarna starten) |

Er is geen installatieprogramma: Windows en Linux starten het bestand zelf, op macOS pak je de zip
uit en sleep je `PWSpeller.app` naar Programma's.

## Eerste start zonder ondertekening

De bestanden zijn (nog) niet ondertekend met een betaald certificaat, dus het besturingssysteem
waarschuwt bij de eerste start. Dat is te verwachten en geen teken dat er iets mis is.

- **Windows:** SmartScreen meldt dat Windows de pc heeft beschermd. Kies *Meer informatie* en
  daarna *Toch uitvoeren*.
- **macOS:** de app is alleen ad-hoc ondertekend en niet genotariseerd; macOS meldt dat de app niet
  geopend kan worden. Open *Systeeminstellingen > Privacy en beveiliging*, scroll naar beneden en kies
  *Toch openen* bij PWSpeller; bevestig met je wachtwoord of Touch ID. Op oudere macOS-versies
  werkt ook rechtsklik op `PWSpeller.app` en dan *Open*. De exacte tekst verschilt per macOS-versie
  en is hier niet op een macOS 26-scherm nagelezen. De app draait in de macOS-sandbox zonder netwerktoegang;
  instellingen en logboek staan dan onder `~/Library/Containers/nl.petisys.pwspeller/`.
- **Linux:** geen waarschuwing; het bestand moet wel uitvoerbaar zijn. Nodig is een desktop met X11
  of Wayland en OpenGL, en een distributie met glibc 2.35 of nieuwer (het bestand wordt op
  Ubuntu 22.04 gebouwd), dus Ubuntu 22.04, Debian 12, Fedora 36 of nieuwer.

Over het ondertekenen en de app stores: zie het [ontwerp](docs/superpowers/specs/2026-10-16-pwspeller-rust-design.md), hoofdstuk 14.

## Gebruik in het kort

Na een splashscreen van anderhalve seconde opent het hoofdvenster met drie tabbladen:

- **Wachtwoord:** kies lengte en tekenreeksen (hoofdletters, kleine letters, cijfers, speciale
  tekens, minimum per soort, dubbelzinnige tekens vermijden) en genereer. Het resultaat staat in
  groepjes van vier met positienummers, zodat je het aan de telefoon kunt doorgeven.
- **Wachtzin:** kies het aantal woorden, het scheidingsteken, hoofdletters en eventueel een cijfer.
  De woorden komen uit de woordenlijst van de gekozen taal en staan heel in beeld.
- **Eigen wachtwoord:** typ of plak een tekst om hem te spellen; de geschatte sterkte staat erbij.

Verder:

- **Spellingstaal:** Nederlands, Vlaams, Frans, Duits, Engels of NAVO-alfabet. De interface zelf is
  Nederlands of Engels (volgt het systeem, instelbaar).
- **Sneltoetsen:** `Enter` genereert, `F5` opent de voorleesmodus, `Cmd+C` (macOS) of `Ctrl+C` kopieert het resultaat. Ze
  werken als geen invoerveld focus heeft.
- **Voorleesmodus** (*Lees voor*): toont één teken of woord tegelijk, groot, met de spelling erbij.
  `Spatie` of pijl rechts is volgende, pijl links vorige, `Esc` stopt.
- **Klembord:** een gekopieerd wachtwoord wordt na 30 seconden gewist (instelbaar; op macOS
  standaard uit) en bij het sluiten, en alleen als het nog onze tekst is.
- **Geschiedenis:** het venster *Geschiedenis* toont de laatste gegenereerde wachtwoorden en
  wachtzinnen, versleuteld in het geheugen en verborgen tot je *Toon* kiest. De geschiedenis staat
  nooit op schijf en wordt **gewist** bij het sluiten, na 15 minuten inactiviteit (instelbaar,
  1 tot 60 minuten) of met *Wis geschiedenis*. Wordt het programma geforceerd beëindigd, dan
  verdwijnt het geheugen mee, maar een gekopieerd wachtwoord op het klembord niet.

Instellingen staan in `instellingen.toml` in de configuratiemap van het besturingssysteem; het
logboek (zonder wachtwoorden) in `pwspeller.log` in de datamap.

## Lokaal bouwen

Nodig is Rust (de toolchain staat vast in `rust-toolchain.toml`).

```bash
cargo run -p pwspeller
cargo test --workspace
```

Op Linux zijn voor eframe enkele pakketten nodig (`libxcb-render0-dev`, `libxcb-shape0-dev`,
`libxcb-xfixes0-dev`, `libxkbcommon-dev`). Een macOS-bundel maak je met
`cargo build --release -p pwspeller` en dan
`packaging/macos/maak-app.sh target/release/pwspeller v2.0.0 .`

## Een taal of woordenlijst toevoegen

Alle taaldata staat in `data/` en wordt bij het bouwen in het programma gebakken; gebruikers kunnen
zelf geen bestanden toevoegen. Een nieuwe spellingstaal, bijvoorbeeld `xx`:

1. Maak `data/alfabetten/xx.toml` met `[meta]` (code, naam) en een codewoord voor elke letter a t/m z.
2. Maak `data/omschrijvingen/xx.toml` met de sjablonen, de cijfers 0-9, alle leestekens van ASCII
   32-126 en de lijst lastige klanken (zie `data/omschrijvingen/nl.toml` als voorbeeld). Hergebruik
   een bestaande taal als de omschrijvingen gelijk zijn (zoals Vlaams die van het Nederlands gebruikt).
3. Maak voor een nieuwe woordenlijst `data/woordenlijsten/xx.txt`: één woord per regel, alleen a-z,
   minstens 5 tekens, geen dubbelen. Aanstootgevende woorden zet je in `xx-geblokkeerd.txt`; die
   mogen niet in de lijst staan.
4. Voeg een `[[taal]]`-blok toe aan `data/spellingstalen.toml` (code, naam, alfabet,
   omschrijvingen, woordenlijst).
5. Voor een nieuwe interfacetaal: `data/interface/xx.toml` met alle sleutels van `en.toml`.
6. Draai `cargo test --workspace`.

**De controle bij het bouwen.** `crates/kern/build.rs` leest alles in `data/` en laat de build
mislukken met een lijst fouten (bestand en melding) als er iets ontbreekt of niet klopt: een letter
of cijfer zonder omschrijving, een lege tekst, een verwijzing naar een onbekend alfabet of
woordenlijst, een interfacesleutel die in `en.toml` staat maar ontbreekt, of een woord dat te kort
is, dubbel staat, andere tekens bevat dan a-z of in de blokkeerlijst staat. Een onvolledige taal
komt dus nooit in een uitgeleverd programma. Waar de woordenlijsten vandaan komen:
[docs/bronnen/woordenlijsten/README.md](docs/bronnen/woordenlijsten/README.md).

## Uitrollen

Een tag `v*` (bijvoorbeeld `v2.0.0`) start
[`.github/workflows/release.yml`](.github/workflows/release.yml). Die controleert de tag, draait
opmaak, clippy, tests en `cargo deny`, bouwt daarna op een Windows-, macOS- en Linux-runner, zet de
tag als versienummer in het programma (`PWSPELLER_VERSIE`) en maakt met `gh release create` de
GitHub-release met de bestanden. De tekst van de release is de sectie van die versie uit
[RELEASENOTES.md](RELEASENOTES.md): **de kop moet de versie dragen**, bijvoorbeeld
`## v2.0.0 — 2026-10-17`. Ontbreekt die, dan faalt de release (`packaging/releasenotes.sh`). Een tag
met achtervoegsel (`v2.0.0-rc1`) wordt een prerelease. Bij elke push en pull request draait
[`ci.yml`](.github/workflows/ci.yml): opmaak, clippy, tests en `cargo deny` (licenties, verboden
netwerkcrates).

```bash
git tag -a v2.0.0 -m "PWSpeller 2.0.0" && git push origin v2.0.0
```

## Documentatie

- [RELEASENOTES.md](RELEASENOTES.md): wat er voor de gebruiker verandert
- [SKILLS.md](SKILLS.md): de Claude Code-skills die in dit project gebruikt zijn
- [NOTICE.md](NOTICE.md): AI-tools en overige rechthebbenden
- [deny.toml](deny.toml): toegestane licenties en verboden crates (`cargo deny check`)
- [docs/implementatie.md](docs/implementatie.md): hoe het in elkaar zit en afgewezen alternatieven
- [docs/voortgang.md](docs/voortgang.md): verloop en beslissingen
- [docs/superpowers/specs/2026-10-16-pwspeller-rust-design.md](docs/superpowers/specs/2026-10-16-pwspeller-rust-design.md): het ontwerp
- [docs/superpowers/plans/2026-10-16-pwspeller-rust.md](docs/superpowers/plans/2026-10-16-pwspeller-rust.md): het implementatieplan, met het [logboek](docs/superpowers/plans/2026-10-16-pwspeller-rust-logboek.md)
- [docs/bronnen/woordenlijsten/README.md](docs/bronnen/woordenlijsten/README.md): de originele woordenlijsten

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
