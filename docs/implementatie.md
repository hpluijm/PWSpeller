## Over dit bestand

- **Doel:** beschrijft hoe PWSpeller 2 in elkaar zit en welke alternatieven bewust zijn afgewezen.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 19: taaldata, generator, spelling, app-onderdelen en licenties toegevoegd; verouderde verwijzing naar Taak 17 weg. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 18 fixronde: sleutelpagina, release-workflow met controle vooraf. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 18: sectie Bouwen en uitleveren. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fix 2: groepjes van vier alleen bij het wachtwoord, eigen tekst doorlopend, spellinglijst doorlopend genummerd, splashscreen 1,5 seconde. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 17 fixronde 1: wissen wist ook het resultaat, invoer telt als gebruik, instellingen begrensd, grenzen vastgelegd. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 17: sectie Geschiedenis- en instellingenpaneel, wissen bij sluiten en na inactiviteit. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 15 fixronde 1: hash wordt vóór het lezen weggenomen (bewust, geen lus). |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 15: sectie Klembord, met de markering per platform. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 14 fixronde 1: sectie Hoofdvenster, met de beperkte garantie voor het wissen van eigen tekst. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fixronde 1: inactiviteit ook op de wandklok; ontsleutel controleert eerst. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 11: sectie Geschiedenis en afgewezen alternatieven voor de geschiedenis. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Opbouw en afgewezen alternatieven opgenomen. |

---

# Implementatie

## Opbouw

Een Cargo-werkruimte (`edition = "2024"`, `resolver = "3"`) met drie crates:

| Crate | Rol |
|---|---|
| `pwspeller-data` (lib `pwspeller_data`) | Ingebakken woordenlijsten en teksten |
| `pwspeller-kern` (lib `pwspeller_kern`) | Genereren en spellen, zonder interface |
| `pwspeller` (bin) | De egui-toepassing |

Windows-bouwsels linken statisch (`.cargo/config.toml`, `crt-static`). De test
`crates/kern/tests/kopblok.rs` bewaakt dat elke historieregel in een kopblok een merkteken heeft.
Het ontwerp staat in [het ontwerpdocument](superpowers/specs/2026-10-16-pwspeller-rust-design.md).

## Taaldata en datamodel (`pwspeller-data`, `data/`)

- `data/` bevat alles wat per taal verschilt, als TOML en tekst: `alfabetten/<code>.toml` (codewoord
  per letter a-z), `omschrijvingen/<code>.toml` (sjablonen, cijfers, leestekens, lastige klanken),
  `woordenlijsten/<code>.txt` met `<code>-geblokkeerd.txt`, `interface/<code>.toml` en
  `spellingstalen.toml` (welk alfabet, welke omschrijvingen en welke woordenlijst bij een taal horen).
  Zes spellingstalen: nl, vl, fr, de, en en navo. Vlaams gebruikt de omschrijvingen van het
  Nederlands; Engels en NAVO volgen de taal van de interface (`omschrijvingen` ontbreekt).
- De crate `pwspeller-data` heeft geen bestandstoegang: `lees_catalogus(&[(pad, tekst)])` zet de
  bestanden om in een `Catalogus` (`Alfabet`, `Omschrijvingen`, `Woordenlijst`, geblokkeerde
  woorden, `Spellingstaal`, interfaceteksten) en `controleer(&Catalogus)` geeft een lijst `DataFout`
  (bestand plus melding). Dat houdt de logica testbaar met kleine voorbeelden.
- `crates/kern/build.rs` leest alle `.toml`- en `.txt`-bestanden in `data/`, draait beide functies en
  laat de build mislukken met alle fouten tegelijk. Daarna schrijft hij `bestanden.rs` met
  `include_str!` per bestand; `talen::catalogus()` leest die één keer in (`OnceLock`). Zo kan een
  onvolledige taal niet in een programma terechtkomen. De controle eist onder meer: alle letters
  a-z, cijfers 0-9 en ASCII 32-126 omschreven, geen lege teksten, geen onbekende verwijzingen,
  elke sleutel uit `en.toml` ook in elke andere interfacetaal, en woorden van minstens 5 tekens, alleen a-z,
  zonder dubbelen en buiten de blokkeerlijst.
- Teksten met een plaatshouder (`{woord}`, `{naam}`) worden in één doorgang ingevuld, zodat een
  ingevulde waarde niet opnieuw als plaatshouder gelezen wordt.

## Generator en sterkte (kern, `generator/`, `sterkte.rs`)

- **Wachtwoord** (`WachtwoordOpties`: lengte 5-128, vier tekensets, minimum per set 1-9,
  dubbelzinnige tekens `I O l 0 1` vermijden). Eerst trekt het per set het minimum aantal tekens,
  vult daarna aan uit alle sets samen en schudt. `effectieve_lengte` is minstens 5 en minstens de som
  van de minima; de app meldt een verhoogde lengte (`melding.lengte_opgehoogd`). Speciale tekens
  zijn `! @ # $ % ^ & *`, bewust een kleine set die aan de telefoon te geven is.
- **Wachtzin** (`WachtzinOpties`: 3-20 woorden, scheidingsteken, hoofdletter per woord, één cijfer).
  Woorden komen uniform uit de gekozen woordenlijst; het cijfer wordt aan één willekeurig woord
  geplakt. `Gegenereerd.woorden` bewaart de woorden apart, zodat de weergave ze heel kan tonen.
- **Willekeur** komt alleen uit `rand::rng()` (ChaCha12, geseed uit het besturingssysteem). Tests
  gebruiken een `StdRng` met vaste seed via de verborgen functies zonder `_os`.
- Het resultaat is een `Gegenereerd` met `Zeroizing<String>`; `entropie_bits` is een bovengrens
  (lengte × log2(pool), zonder aftrek voor de minima).
- **Sterkte** (`sterkte.rs`): `niveau` geeft Zwak (< 60 bits), Redelijk (60 tot en met 80) of Sterk.
  `schatting_bits` voor eigen tekst telt de aanwezige tekensoorten bij elkaar (26, 26, 10, 33, 100 voor
  niet-ASCII); het is een grove schatting, geen wachtwoordsterkteanalyse.

## Spelling (kern, `spelling.rs`)

- `Speller::nieuw(alfabet, omschrijvingen)` levert `Regel`s: `positie` (1-based, in tekens),
  `fragment`, `omschrijving`, `soort` (hoofdletter, kleine letter, cijfer, speciaal, spatie, woord,
  onbekend) en `lastig`.
- `per_teken` spelt elk teken. Een teken zonder omschrijving krijgt de sjabloon `onbekend` met de
  code `U+xxxx`, zodat onzichtbare of vreemde tekens nooit stil wegvallen.
- `wachtzin` spelt per woord ("woord correct") en het scheidingsteken per teken; een eindcijfer
  wordt als apart cijfer gespeld. Een woord is `lastig` als het een klank bevat of eindigt op een
  klank uit `lastige_klanken` van de taal (Nederlands: ij, ei, au, ou, of een woord op d of t); de app markeert die.
- `omschrijvingen_voor(taal, interfacetaal)` kiest de omschrijvingen van de spellingstaal, of die van
  de interface als de taal er geen heeft.

## App-onderdelen (`pwspeller`, `crates/app/src`)

| Bestand | Rol |
|---|---|
| `main.rs`, `lib.rs`, `app.rs` | Opstarten, logboek, `PwApp` (`eframe::App`), de splash-, hoofd- en panelen in één beeldlus |
| `splash.rs` | Splashscreen (logo, naam, klant, versie, copyright); start nooit het programma tegen |
| `hoofd.rs`, `opties.rs` | Hoofdvenster met drie tabs, de opties per tab, sneltoetsen |
| `resultaat.rs` | Weergave van het resultaat en de spellinglijst |
| `voorlezen.rs` | Voorleesmodus: één teken of woord, groot |
| `klembord.rs` | Kopiëren en na een tijd wissen |
| `geschiedenis_paneel.rs`, `instellingen_paneel.rs` | De twee vensters |
| `instellingen.rs`, `logboek.rs` | Instellingen (TOML) en logboek in de mappen van het besturingssysteem |
| `thema.rs` | Petisys-kleuren, lettertype Atkinson Hyperlegible Mono |

`build.rs` van de app zet het versienummer (`PWSPELLER_VERSIE`, uit de omgeving of anders `git describe --tags --always`) en
bedt het Windows-icoon in. Versie en klant op het splashscreen komen dus uit de release, niet uit de code.

## Geschiedenis (kern, `geschiedenis.rs`)

- Sleutel van 32 willekeurige bytes op een eigen geheugenpagina (`SleutelPagina`: `std::alloc` met
  een `Layout` van één pagina, pagina-uitgelijnd), vergrendeld met `region::lock`; mislukt dat, dan
  volgt één `log::warn!` en werkt de app door. Bij drop: wissen, ontgrendelen (fout alleen gelogd),
  vrijgeven. Een `Box` is afgewezen: kleine allocaties delen een pagina, en vergrendelen gaat per
  pagina zonder teller, dus het ontgrendelen van de ene sleutel hief de lock van de andere op
  (Windows: ERROR_NOT_LOCKED en een paniek in `LockGuard`). `region::alloc` is afgewezen omdat
  `Allocation::drop` dezelfde `debug_assert` heeft.
- Elk item apart met XChaCha20-Poly1305 en een eigen nonce van 24 bytes. De platte tekst is een
  eigen codering (modus, tekst, wachtzin-onderdelen, entropie) in een `Zeroizing<Vec<u8>>`; de
  decodering controleert elke lengte en geeft bij corrupte data `None`.
- Maximaal 100 items (het oudste valt eraf). `wis()` leegt de items, overschrijft de sleutel met
  nieuwe willekeurige bytes en laat de id-teller doorlopen. `controleer_inactiviteit` wist bij
  `nu - laatst_gebruikt >= time_out`; toevoegen en geslaagd ontsleutelen tellen als gebruik.
- `Instant` loopt op macOS en Linux niet door tijdens slaapstand, dus de geschiedenis houdt ook
  een wandklok-tijdstip (`SystemTime`) van het laatste gebruik bij. `controleer_inactiviteit`
  (en testbaar `controleer_inactiviteit_op(nu, wand)`) wist als een van beide klokken de time-out
  haalt. Staat de wandklok achteruit (`duration_since` geeft een fout), dan beslist alleen
  `Instant`. `ontsleutel` controleert eerst de inactiviteit en geeft `None` als er gewist is.
- `Debug` toont alleen het aantal items.

## Hoofdvenster (app, `hoofd.rs`, `resultaat.rs`, `opties.rs`)

- **Weergave van het resultaat:** groepjes van vier met positienummers bestaan alleen bij een
  gegenereerd wachtwoord (tab Wachtwoord, ook uit de geschiedenis). Een wachtzin toont hele woorden;
  eigen tekst is één doorlopende regel in kleur (onzichtbare tekens als `U+xxxx`, breken op elk teken,
  geen ingevoegde spaties of nummers). De spellinglijst nummert doorlopend 1, 2, 3 (weergave), niet
  naar `Regel.positie`, want bij een wachtzin per woord springt die. De voorleesmodus markeert alleen
  bij een wachtwoord een groepje; anders het huidige teken of woord.
- **Splashscreen:** 1,5 seconde (`SPLASHDUUR`), besluit opdrachtgever 17 oktober 2026.
- Spelregels en groepjes staan in een cache (`SpelCache`) die alleen opnieuw wordt opgebouwd bij een
  andere tab, tekst, spellingstaal, interfacetaal of spelwijze. De cache overschrijft zijn teksten met
  nullen als hij vervangen of vrijgegeven wordt.
- Sneltoetsen (Enter, F5, Cmd/Ctrl+C) werken alleen als geen enkel widget focus heeft.
- **Eigen tekst wissen is beperkt.** De eigen tekst staat in een gewone `String` (het tekstveld van egui
  vereist dat). We roepen `zeroize()` aan bij het verlaten van het tabblad en bij leegmaken, en beginnen
  met een ruime capaciteit zodat de buffer niet vaak verhuist. Maar egui houdt eigen kopieen (galleys,
  undo-geschiedenis, accessibility-boom) en een gegroeide buffer laat een oude kopie achter. Die liggen
  buiten ons bereik, dus dit verkleint het risico en sluit het niet uit. Gegenereerde wachtwoorden
  staan wel in `Zeroizing`.

## Klembord (app, `klembord.rs`)

- `Klembordbeheer<B: Bord>` onthoudt alleen een SHA-256 van wat wij zetten, nooit de tekst. Bij
  wissen wordt het bord gelezen, de gelezen `String` gaat in `Zeroizing` en wordt na de
  vergelijking gewist; alleen bij gelijke hash wordt gewist. Heeft de gebruiker intussen iets
  anders gekopieerd, dan blijft dat staan.
- De hash wordt vóór het lezen van het bord weggenomen (`take`): een mislukte lees- of
  wispoging wordt dus niet herhaald. Bewust, om een lus van pogingen elk beeld te voorkomen.
- `tik` (elk beeld) wist na `klembord_seconden`; `bij_sluiten` wist alleen als `klembord_wissen`
  aanstaat. Staat het uit, dan is er geen enkele aanroep op het bord (getest met een telnepbord).
- `PwApp` bezit `Option<Klembordbeheer<Box<dyn Bord>>>` (`None` zonder systeemklembord, zoals
  Linux in CI) en vraagt `request_repaint_after` tot het wis-moment. Fouten: `log::warn!` zonder
  inhoud en de melding `melding.klembord_mislukt`; nooit paniek. `bij_afsluiten` (aangeroepen vanuit
  `on_exit`) roept `bij_sluiten` van het klembord aan.
- **Markering als vertrouwelijk (arboard 3.7.0, `exclude_from_history`):** macOS zet
  `org.nspasteboard.ConcealedType` (arboard doet dat zelf, geen eigen objc-code nodig). Windows
  zet `ExcludeClipboardContentFromMonitorProcessing`-formaten via `exclude_from_history` en
  `exclude_from_cloud` (`CanIncludeInClipboardHistory`, `CanUploadToCloudClipboard` op 0). Linux
  gebruikt `exclude_from_history`, dat alleen onder Wayland (data-control) iets doet; onder X11
  is er geen markering. De Windows- en Linux-takken zijn op macOS niet gecompileerd, dat doet CI.
- `image-data` van arboard staat uit: er wordt alleen tekst gebruikt.

## Geschiedenis- en instellingenpaneel (app, `geschiedenis_paneel.rs`, `instellingen_paneel.rs`)

- Twee knoppen in de bovenbalk openen een venster (`egui::Window`): *Geschiedenis* en
  *Instellingen* (tekst, geen tandwielglyph, want dat staat niet zeker in het lettertype).
- Items staan nieuwste eerst met lokale tijd (`chrono`, alleen feature `clock`, geen netwerk),
  modus en spellingstaal. De tekst is `••••••••` tot *Toon*; dan ontsleutelt `ontsleutel` het item
  en staat het in `PwApp.getoond` (`Gegenereerd` met `Zeroizing`-velden). Verbergen, het paneel
  sluiten, wissen of een time-out leegt die lijst. *Kopieer* zet de ontsleutelde tekst eenmalig
  in `extra_kopie` en gaat zo door de bestaande klembordroute; *Opnieuw spellen* laadt het item
  als huidig resultaat (tab en spellingstaal volgen het item).
- **Inactiviteit:** `tik_geschiedenis` draait elk beeld, ook tijdens de voorleesmodus (die het
  hoofdvenster niet tekent). Wist `controleer_inactiviteit`, dan gaan de voorleesmodus en de
  getoonde items dicht en komt `geschiedenis.gewist_inactief` (tot de volgende handeling). De
  voorleesmodus sluit bij elke wisbeurt, ook als hij het huidige resultaat toont: eenvoudiger en
  veiliger dan onderscheid maken. `request_repaint_after(1 s)` alleen zolang er items zijn en
  een time-out staat.
- Bij een inactiviteitswis gaat ook het huidige resultaat weg (`wis_resultaat`: gegenereerde
  resultaten, spelling-cache, eigen tekst, voorleesmodus, getoonde items); `bij_afsluiten` doet hetzelfde.
- **Invoer telt als gebruik:** toets, tekst, klik, scroll, aanraking en klembordacties roepen
  `Geschiedenis::markeer_gebruik` aan (muisbewegen alleen niet). Dat gebeurt ná de wiscontrole van
  dat beeld, zodat een eerste toetsaanslag na lange stilte de wis niet meer kan voorkomen.
- De time-out volgt de instelling elk beeld (`Geschiedenis::zet_time_out`), dus een wijziging in het
  instellingenpaneel geldt direct. `zet_time_out` zet de teller niet terug: die loopt door vanaf
  het laatste gebruik.
- `Instellingen::laad` begrenst waarden (klembord 5-300 s, geschiedenis 1-60 min met 0 als "alleen
  bij sluiten", lengte 5-128, woorden 3-20, minima 1-9), zodat een met de hand bewerkt bestand
  niets onbruikbaars oplevert.
- **Grenzen, eerlijk:** `on_exit` draait bij het sluiten van het venster en bij normaal afsluiten via
  eframe. Bij kill, crash of geforceerd afmelden niet: het geheugen verdwijnt dan met het proces,
  het klembord niet. Bij een geminimaliseerd venster kan het wissen pas bij het volgende beeld
  gebeuren; dat is nog handmatig te meten. Ontsleutelde items staan ook in egui's galley-cache en
  in de AccessKit-boom, buiten ons bereik.
- **Sluiten:** `eframe::App::on_exit` roept `bij_afsluiten` aan: geschiedenis wissen, getoonde
  items en voorleesmodus loslaten, klembord wissen volgens de instelling.
- **Niets op schijf:** instellingen bevatten alleen opties; het logboek alleen meldingen zonder
  inhoud (`Gegenereerd` toont de tekst als `***`). `nieuw_met_paden` plus `start_logboek`
  (mag vaker draaien; de ene globale logger wisselt van bestand) maken de test mogelijk die 20
  generaties (wachtwoord en wachtzin, woorden apart) tegen beide bestanden controleert.

## Bouwen en uitleveren (Taak 18)

- **Licenties** (`deny.toml`): lijst uit de randvoorwaarden plus `BSL-1.0`; `MPL-2.0` alleen voor `option-ext` (per-crate-uitzondering). `[graph] targets` beperkt de controle tot de vijf uitleverdoelen. HTTP-crates en `openssl` staan onder `[bans]`.
- **CI** (`.github/workflows/ci.yml`): fmt, clippy (`--all-targets -D warnings`) en test op ubuntu, macos en windows; `cargo deny` (licenses, bans, sources) op ubuntu, en advisories in een eigen job met `continue-on-error`. Linux-pakketten: de lijst uit de eframe-README (`libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`, `libxkbcommon-dev`) zonder `libssl-dev` en GTK (geen netwerk, geen bestandsdialoog).
- **Release** (`release.yml`): eerst de tag valideren (achtervoegsel = prerelease) en de sectie uit `RELEASENOTES.md` halen (`packaging/releasenotes.sh`, exacte kop, faalt als hij ontbreekt); dan fmt, clippy en tests op drie OS en een blokkerende `cargo deny`; daarna per platform een job die bouwt en een artifact uploadt; een laatste job (enige met `contents: write`) maakt de release met `gh release create`. De versie komt uit de tag via `PWSPELLER_VERSIE` (zie `build.rs`).
- **Windows**: `crt-static` staat in `.cargo/config.toml`; de job controleert met `dumpbin /dependents` dat `VCRUNTIME` ontbreekt. Het icoon gaat via `winresource` in `build.rs`.
- **macOS**: `lipo` voegt arm64 en x86_64 samen; `packaging/macos/maak-app.sh` maakt de bundel (Info.plist met de versie uit de tag, `AppIcon.icns`, ad-hoc `codesign` met hardened runtime en entitlements: alleen `app-sandbox`).
- **Vastgepind**: actions op commit-SHA van major-versies (checkout v7, upload-artifact v7, download-artifact v8, cargo-deny-action v2); toolchain via `rustup` van de runner.

## Licenties en afhankelijkheden

- Toegestaan: MIT, Apache-2.0, BSD-2/3, ISC, Zlib, Unicode-3.0, OFL-1.1 en CC-BY-3.0 (EFF-lijst),
  bewaakt door `cargo deny check licenses`. Op 17 oktober 2026 keurde de opdrachtgever twee
  uitzonderingen goed: `BSL-1.0` (Boost, permissief; `clipboard-win` en `error-code` via arboard,
  alleen Windows) en `MPL-2.0` alleen voor `option-ext` (via `directories`, onveranderd gebruikt).
  Ze staan in `deny.toml` en in [NOTICE.md](../NOTICE.md).
- Geen netwerk: HTTP-clients, TLS-crates en `openssl` staan onder `[bans]`; eframe draait zonder de
  feature `persistence`; `image-data` van arboard staat uit.
- Hoofdafhankelijkheden: `eframe`/`egui` (interface), `rand` 0.10, `zeroize`, `chacha20poly1305`
  en `region` (geschiedenis), `arboard` en `sha2` (klembord), `serde` en `toml`, `directories`,
  `sys-locale`, `chrono` (alleen feature `clock`), `log`.

## Afgewezen alternatieven

| Alternatief | Waarom niet |
|---|---|
| VB.NET houden op .NET 10 (WinForms) | Alleen Windows |
| C# met Avalonia | VB niet ondersteund, grote bestanden, native bibliotheken naast de exe |
| Go met Fyne | cgo en OpenGL, lastiger kruiselings bouwen |
| Eén HTML-bestand | Geen exe, geen app store-pakket |
| `time` met lokale offset in plaats van `chrono` | `time::UtcOffset::current_local_offset` faalt in multithreaded programma's; `chrono` met alleen `clock` is bruikbaar en klein |
| Geschiedenis als vast zijpaneel | Een venster houdt het hoofdvenster op de minimale breedte intact |
| `softprops/action-gh-release` voor de release | Extra actie van een derde partij; `gh release create` staat op de runner |
| `dtolnay/rust-toolchain` voor de toolchain | Rustup staat op de runners; één actie minder om te vertrouwen |
| Slint | Extra licentievoorwaarde, eigen opmaaktaal |
| Iced | Nog vóór 1.0, API verandert per release |
| Crate `bitwarden-generators` gebruiken | GPL-3.0 zou PWSpeller GPL maken |
| Gebruikers eigen taalbestanden laten importeren | Verschillende versies per medewerker; sandbox |
| Dubbelzinnige Nederlandse woorden schrappen | Kost ruim 2.000 woorden; per letter spellen lost het op |
| Geschiedenis met serde_json als platte tekst | Extra kopieën van het wachtwoord in tussenbuffers die niet gewist worden |
| Geschiedenis versleuteld bewaren op schijf | Spec §9: alleen in het geheugen |
| Franse en Duitse woorden met accent of ß spellen of omzetten (é → e, ü → ue) | Foutgevoelig aan de telefoon; omzetten maakt niet-bestaande woorden |

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
