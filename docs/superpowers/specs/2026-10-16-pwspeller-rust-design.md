## Over dit bestand

- **Doel:** het goedgekeurde ontwerp van PWSpeller 2: een wachtwoordgenerator en -speller in Rust, per platform één bestand.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, nog geen release (volgt als v2.0.0)
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Franse en Duitse woordenlijst toegevoegd (alleen a–z). |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Ontwerp vastgelegd na de brainstorm. |

---

# PWSpeller 2 — ontwerp

Status: goedgekeurd per onderdeel in de brainstorm van 16 oktober 2026; deze uitwerking wacht op
review.

## 1. Doel en gebruikers

PWSpeller maakt wachtwoorden en wachtzinnen en spelt ze zo dat ze **telefonisch foutloos door te
geven** zijn. Gebruikers zijn de engineers van Petisys en helpdeskmedewerkers die een wachtwoord
aan een klant doorgeven.

Geslaagd als:

- er per platform (Windows, macOS, Linux) één bestand is dat met dubbelklikken start, zonder losse
  DLL's of andere bestanden ernaast;
- een medewerker binnen een paar seconden een wachtwoord maakt en het in de taal van de klant kan
  voorlezen;
- de generator zich gedraagt als die van Bitwarden;
- de app later in een app store (Mac App Store, Microsoft Store, Flathub) kan zonder ombouw.

### Buiten deze versie

- Publiceren in een app store en het ondertekenen van de bestanden (zie Openstaande punten).
- De gebruikersnaamgenerator van Bitwarden.
- Taalbestanden die de gebruiker zelf importeert: talen komen alleen via de repo.
- Opslag van wachtwoorden op schijf, in welke vorm ook.

## 2. Uitgangspunten

| Keuze | Besluit |
|---|---|
| Taal en interface | Rust met egui (eframe) |
| Generator | Het concept van Bitwarden nagebouwd, **zonder hun code** (de crate `bitwarden-generators` is GPL-3.0 of Bitwarden-licentie) |
| Licenties van afhankelijkheden | Alleen permissief (MIT, Apache-2.0, BSD, Zlib, OFL voor het lettertype, CC BY voor de EFF-lijst) |
| Talen toevoegen | Door de ontwikkelaar, als databestand in de repo, ingebakken bij het bouwen |
| Interfacetaal | Volgt het besturingssysteem; Nederlands en Engels, terugval Engels, met de hand om te zetten |
| Netwerk | Geen. Geen telemetrie, geen updatecontrole |
| Oude VB-versie | Bewaard als tag `v1.0-vb` met GitHub-release; daarna verdwijnt hij uit `master` |
| Merk | Petisys: logo, zwart `#111312`, goud `#927C43` (alleen accenten), donkergoud `#6D5D32` (tekst) |

## 3. Opbouw van de repo

```
PWSpeller/
├── Cargo.toml                  werkruimte
├── crates/
│   ├── kern/                   bibliotheek, geen interface, geen egui
│   │   ├── generator/          wachtwoord en wachtzin
│   │   ├── spelling/           tekst → regels om voor te lezen
│   │   ├── talen/              laadt en ontsluit de ingebakken data
│   │   ├── sterkte/            entropie in bits
│   │   └── geschiedenis/       versleuteld, alleen in het geheugen
│   └── app/                    egui-interface, splashscreen, klembord, instellingen
├── data/
│   ├── spellingstalen.toml     de kant-en-klare combinaties
│   ├── alfabetten/             nl, vl, fr, de, navo
│   ├── omschrijvingen/         nl, en, fr, de
│   ├── interface/              nl, en
│   └── woordenlijsten/         en-eff, nl, fr, de (.txt) + <code>-geblokkeerd.txt
├── assets/                     petisys-logo, app-icoon, Atkinson Hyperlegible Mono
└── docs/
```

De kern hangt niet af van egui. Een ander interfaceframework is daardoor later mogelijk zonder
de kern te raken.

## 4. Talen en data

Spellen is de combinatie van twee lagen:

1. **Alfabet** — het codewoord per letter (`a = "Anna"`).
2. **Omschrijvingen** — de woorden eromheen in een taal: "hoofdletter", "kleine letter", de namen
   van cijfers en leestekens, en de lastige klanken van die taal.

### Combinaties (`data/spellingstalen.toml`)

| Spellingstaal | Alfabet | Omschrijvingen | Woordenlijst wachtzin |
|---|---|---|---|
| Nederlands | nl (Anna, Bernard, Cornelis …) | nl | nl |
| Vlaams | vl (Arthur, Brussel, Carolina …) | nl | nl |
| Frans | fr (Anatole, Berthe, Célestin …) | fr | fr |
| Duits | de (DIN 5009:2022: Aachen, Berlin, Chemnitz …) | de | de |
| Engels | navo | en | en-eff |
| NAVO | navo (Alfa, Bravo, Charlie …) | taal van de interface | en-eff |

De codewoorden voor Frans en Duits worden bij de bouw nagezocht tegen een actuele bron
(afspraak 1); het Duitse alfabet volgt DIN 5009 in de versie van 2022.

### Bestandsvorm

`data/alfabetten/nl.toml`:

```toml
[meta]
code = "nl"
naam = "Nederlands"

[letters]
a = "Anna"
b = "Bernard"
# … t/m z
```

`data/omschrijvingen/nl.toml`:

```toml
[meta]
code = "nl"

[sjablonen]
hoofdletter = "hoofdletter {woord}"
kleine_letter = "kleine letter {woord}"
cijfer = "cijfer {naam}"
woord = "{woord}"
woord_met_hoofdletter = "{woord}, met hoofdletter"
onbekend = "teken {teken} ({code})"

[cijfers]
"0" = "nul"
# … t/m "9"

[tekens]
" " = "spatie"
"!" = "uitroepteken"
"-" = "streepje"
# … alle afdrukbare ASCII-leestekens (33–126) en de spatie

[lastige_klanken]
bevat = ["ij", "ei", "au", "ou"]
eindigt_op = ["d", "t"]
```

`data/interface/<code>.toml` bevat de teksten van knoppen, labels en meldingen.

### Bouwen en controleren

Een buildscript neemt alle bestanden in `data/` op in de binary. Het bouwen **faalt** met
bestandsnaam en ontbrekend teken als:

- een alfabet niet alle 26 letters heeft;
- een omschrijvingenbestand niet alle cijfers, de spatie en alle afdrukbare ASCII-leestekens dekt;
- een combinatie naar een bestand verwijst dat niet bestaat;
- een interfacebestand een sleutel mist die in `en.toml` wel staat.

Een nieuwe taal = één of twee TOML-bestanden en één regel in `spellingstalen.toml`.

### Woordenlijsten

- `en-eff.txt` — de lange woordenlijst van de EFF (CC BY 3.0), zonder woorden met een koppelteken.
- `nl.txt` — bron: `nederlandse_woorden_8000_min5.txt` (8.000 woorden, met Claude gegenereerd in
  opdracht van Petisys), min de geblokkeerde woorden.
- `nl-geblokkeerd.txt` — woorden die niet aan een klant worden voorgelezen (onder meer "moord",
  "zelfmoord", "verkrachting", "porno", "schijt", "doodschieten", "drugs"). Een eerste versie
  wordt opgesteld en door Hans goedgekeurd vóór de release.
- `fr.txt` — bron: `mots_francais_8000_min5.txt` (8.000 woorden, met Claude gegenereerd in
  opdracht van Petisys). Alleen de woorden met a–z blijven over (5.743, 12,5 bits per woord);
  woorden met é, è, ç enz. vallen weg. Min `fr-geblokkeerd.txt`.
- `de.txt` — bron: `deutsche_woerter_8000_min5.txt` (8.000 woorden, idem). Eerst naar kleine
  letters (zelfstandige naamwoorden staan met hoofdletter in de bron), daarna alleen a–z (6.632,
  12,7 bits per woord); woorden met ä, ö, ü of ß vallen weg. Min `de-geblokkeerd.txt`.

Regels, bewaakt door een test: elk woord minimaal 5 tekens, alleen a–z, uniek, en niet in de
blokkeerlijst. Voor `en-eff` gelden alleen a–z en uniek (de EFF-lijst heeft ook kortere woorden).
Accenten en ß zijn uitgesloten omdat ze telefonisch foutgevoelig zijn en de klant ze op zijn
toetsenbord moet vinden.

## 5. Generator

### Wachtwoord

| Instelling | Grenzen | Standaard |
|---|---|---|
| Lengte | 5–128 | 14 |
| Hoofdletters A-Z | aan/uit | aan |
| Kleine letters a-z | aan/uit | aan |
| Cijfers 0-9 | aan/uit | aan |
| Speciale tekens `! @ # $ % ^ & *` | aan/uit | uit |
| Minimaal aantal cijfers | 1–9, alleen zichtbaar als cijfers aanstaan | 1 |
| Minimaal aantal speciale tekens | 1–9, alleen zichtbaar als speciale tekens aanstaan | 1 |

De apps van Bitwarden tonen voor speciale tekens 0 als standaard, maar hun generator maakt daar
bij een set die aanstaat altijd 1 van. PWSpeller toont wat er werkelijk gebeurt: 1.
| Dubbelzinnige tekens vermijden (`I O l 0 1`) | aan/uit | uit |

Werkwijze, zoals Bitwarden:

1. Elke set die aanstaat telt minimaal één keer mee; een ingesteld minimum hoger dan één gaat
   voor.
2. Per set wordt het minimum getrokken; de rest van de lengte komt uit de vereniging van alle
   sets die aanstaan.
3. Het geheel wordt geschud (Fisher-Yates).

- Er staat altijd minstens één set aan: de laatste vink je niet uit.
- Zijn de minima samen groter dan de lengte, dan wordt de lengte opgehoogd tot die som.
- Alle trekkingen komen uit de cryptografisch veilige bron van het besturingssysteem (`getrandom`
  via `rand`), met uniforme trekking zonder modulo-bias.

Bron voor gedrag en standaardwaarden: `bitwarden/sdk-internal`
(`crates/bitwarden-generators/src/password.rs`) en `bitwarden/clients`
(`libs/tools/generator/core/src/data/default-password-generation-options.ts`), gelezen op
16 oktober 2026. Gelezen voor het gedrag, niet overgenomen.

### Wachtzin

| Instelling | Grenzen | Standaard |
|---|---|---|
| Aantal woorden | 3–20 | 6 |
| Scheidingsteken | één of meer tekens, niet leeg | `-` |
| Hoofdletter aan het begin van elk woord | aan/uit | uit |
| Cijfer (0–9) achter één willekeurig woord | aan/uit | uit |
| Woordenlijst | volgt de spellingstaal, met de hand te kiezen | zie tabel in 4 |

## 6. Spellen

Spellen levert een lijst regels: `(positie, fragment, omschrijving, soort, lastig)`.

- **Wachtwoord en eigen tekst, per teken:** "hoofdletter Anna", "kleine letter Bernard",
  "cijfer zeven", "uitroepteken". Tekens genummerd, in groepjes van vier.
- **Wachtzin, per woord:** "Zonnig, met hoofdletter", "streepje", "tafel", "cijfer zeven". Een
  woord met een lastige klank uit de omschrijvingen wordt automatisch per letter gespeld en
  gemarkeerd. De knop *Per teken* spelt de hele wachtzin per teken.
- **Onbekend teken** (é, emoji, enz.): geen crash, maar "teken é (U+00E9)" en een waarschuwing dat
  het moeilijk door te geven is.

## 7. Sterkte

- Gegenereerd wachtwoord: lengte × log₂(aantal mogelijke tekens), exact.
- Wachtzin: aantal woorden × log₂(woorden in de lijst); het cijfer telt mee als log₂(10 × aantal
  woorden).
- Eigen tekst: schatting uit de gebruikte tekensoorten en de lengte, aangeduid als "schatting".
- Niveaus: zwak < 60 bits, redelijk 60–80, sterk > 80.

## 8. Interface

### Splashscreen (afspraak 24)

Drie seconden, vóór het hoofdvenster: Petisys-logo, "PWSpeller", "Klant: Petisys", versienummer,
"© 2026 Petisys. Alle rechten voorbehouden.", een dunne gouden lijn. Het versienummer komt bij het
bouwen uit de git-tag (omgevingsvariabele in de CI, anders `git describe --tags`, anders de versie
uit `Cargo.toml`). Faalt het splashscreen, dan start de app gewoon, met een regel in het logboek.

### Hoofdvenster

1. Bovenbalk: tabbladen *Wachtwoord · Wachtzin · Eigen tekst*, keuze spellingstaal, knop
   *Geschiedenis*, tandwiel voor instellingen.
2. Resultaat groot in Atkinson Hyperlegible Mono, groepjes van vier met positienummers. Cijfers
   blauw, speciale tekens rood, letters in de tekstkleur, hoofdletters vet. Kleur is nooit het enige
   verschil. Daaronder de sterktebalk met bits.
3. Knoppen *Genereer* (Enter), *Kopieer* (Ctrl/Cmd+C), *Voorlezen* (F5).
4. Opties van het actieve tabblad.
5. Spelling als genummerde lijst, lastige woorden gemarkeerd.

### Voorleesmodus

Hele venster. Bovenaan het volledige wachtwoord met de huidige groep gemarkeerd, in het midden
groot het huidige teken met zijn spelling, onderin "teken 7 van 14". Spatie of → volgende, ←
vorige, Esc terug.

### Klembord

| Platform | Automatisch wissen (na 30 s en bij sluiten) |
|---|---|
| macOS | standaard **uit**; de app raakt het klembord verder niet aan |
| Windows, Linux | standaard **aan** |

- Overal in te stellen; ook de wachttijd.
- Er wordt alleen gewist als ons wachtwoord er nog op staat.
- Een gekopieerd wachtwoord wordt als vertrouwelijk gemarkeerd: op macOS
  `org.nspasteboard.ConcealedType`, op Windows buiten de klembordgeschiedenis en de
  cloudsynchronisatie. Dat wist niets en verandert niets aan plakken.

### Verder

- Licht of donker volgens het besturingssysteem, in de Petisys-kleuren.
- Volledig met het toetsenbord te bedienen; schermlezers via AccessKit.
- Venster te verkleinen tot een minimale breedte waarbij niets wegvalt.
- Instellingen (taal, standaardopties, tijden) in de app-map van het besturingssysteem. Nooit
  wachtwoorden of geschiedenis.

## 9. Geschiedenis en beveiliging

- Inhoud: elke gegenereerde waarde met soort, tijdstip en spellingstaal. Maximaal 100 items; het
  oudste valt eraf. Eigen tekst komt er niet in.
- Bij het starten een willekeurige sleutel van 256 bits, alleen in het geheugen. Elk item apart
  versleuteld met XChaCha20-Poly1305 en een eigen nonce.
- Ontsleutelen alleen bij tonen, kopiëren of spellen, in een buffer die zichzelf overschrijft
  (`zeroize`). Waar het besturingssysteem het toestaat, wordt het geheugen met de sleutel
  vergrendeld (`mlock`/`VirtualLock`); lukt dat niet, dan werkt de app door.
- Wissen van sleutel en items: bij sluiten (venster, Cmd+Q, Alt+F4, afmelden, afsluiten), na
  inactiviteit (standaard 15 minuten, in te stellen 1–60 of "alleen bij sluiten") en met de knop
  *Wis geschiedenis*.
- Weergave gemaskeerd, met tonen, kopiëren en opnieuw spellen per item.
- De persistentie van egui staat uit voor alles behalve gewone instellingen.
- Grens: wat al met beheerdersrechten in het geheugen van de draaiende app meekijkt, houdt niets
  tegen. Een crashdump van Windows kan een item bevatten dat op dat moment open staat; de tijd dat
  iets leesbaar in het geheugen staat, wordt zo kort mogelijk gehouden.

## 10. Bouwen en uitleveren

| Platform | Uitlevering | Afhankelijkheden |
|---|---|---|
| Windows x64 en ARM64 | `PWSpeller.exe` | geen; C-runtime statisch (`crt-static`) |
| macOS universeel (arm64 + x86_64) | `PWSpeller.app`, gezipt | geen |
| Linux x64 | `pwspeller` | libc, X11/Wayland en OpenGL van de desktop |

- GitHub Actions bouwt bij een tag `v*` op een Windows-, macOS- en Linux-runner, draait alle tests
  en zet de bestanden bij de GitHub-release.
- Lokaal bouwen op de Mac met `rustup` uit Homebrew; na installatie `sync.sh capture`.
- De macOS-build krijgt sandbox-entitlements zonder netwerk, zodat de App Store later geen
  ombouw vraagt.

### Volgorde in git

1. Tag `v1.0-vb` op `79db8f8` en een GitHub-release met de VB-broncode.
2. De VB-bestanden (`PWSpeller/`, `Installer/`, `PWSpeller.sln`) uit `master`.
3. De Rust-versie bouwen.
4. Release `v2.0.0`.

## 11. Tests

- Generator: lengtes, minima, dubbelzinnige tekens, altijd minstens één set; chi-kwadraattoets op
  de verdeling met een vaste seed.
- Wachtzin: aantallen, scheidingsteken, hoofdletters, cijfer.
- Woordenlijsten: minimaal 5 tekens, a–z, uniek, niets uit de blokkeerlijst.
- Data: elk teken dat de generator of eigen tekst kan opleveren (ASCII 32–126) heeft een spelling
  in elke combinatie.
- Spellen: voorbeelden per taal, lastige klanken, onbekende tekens.
- Geschiedenis: versleutelen en terughalen, wissen bij sluiten en na inactiviteit; geen wachtwoord
  in instellingen of logboek.
- Splashscreen: toont de vijf gegevens en komt vóór het hoofdvenster (`egui_kittest`).
- Kopblok: elke historieregel heeft een geldig merkteken (afspraak 12).

## 12. Documentatie

In de hoofdmap, met het Petisys-logo bovenaan: `README.md`, `RELEASENOTES.md`, `NOTICE.md`,
`SKILLS.md`. In `docs/`: `implementatie.md`, `voortgang.md`. `NOTICE.md` noemt de licenties van
egui en de overige crates, de EFF-lijst (CC BY 3.0), Atkinson Hyperlegible Mono (SIL OFL) en de
Nederlandse lijst.

## 13. Afgewezen alternatieven

| Alternatief | Waarom niet |
|---|---|
| VB.NET houden op .NET 10 (WinForms) | Alleen Windows |
| C# met Avalonia | VB niet ondersteund, grote bestanden, native bibliotheken naast de exe |
| Go met Fyne | cgo en OpenGL, lastiger kruiselings bouwen |
| Eén HTML-bestand | Geen exe, geen app store-pakket |
| Slint | Extra licentievoorwaarde, eigen opmaaktaal |
| Iced | Nog vóór 1.0, API verandert per release |
| Crate `bitwarden-generators` gebruiken | GPL-3.0 zou PWSpeller GPL maken |
| Gebruikers eigen taalbestanden laten importeren | Verschillende versies per medewerker; sandbox |
| Dubbelzinnige Nederlandse woorden schrappen | Kost ruim 2.000 woorden; per letter spellen lost het op |
| Franse en Duitse woorden met accent of ß spellen of omzetten (é → e, ü → ue) | Foutgevoelig aan de telefoon; omzetten maakt niet-bestaande woorden |

## 14. Openstaande punten

- Ondertekenen: Apple Developer-account (macOS, ook notarisatie) en een certificaat voor
  codeondertekening (Windows). Zonder: waarschuwing bij de eerste start.
- Publiceren in de app stores.
- De blokkeerlijsten `nl-`, `fr-` en `de-geblokkeerd.txt` laten goedkeuren door Hans.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
