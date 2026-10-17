## Over dit bestand

- **Doel:** legt per werksessie vast wat er veranderd is en waarom, nieuwste bovenaan.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 18 fixronde: sleutel op een eigen pagina (Windows-paniek), valse treffer in de schijftest, release-workflow aangescherpt. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 18: licentiecontrole, CI en uitlevering per platform. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Visuele controle: voorleesmodus, groepjes, splash 1,5 s. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fix: tabblad Wachtzin toont hele woorden met scheidingsteken en woordnummer, niet groepjes van vier die woorden middenin afbraken; wachtwoord en eigen tekst blijven groepjes. De voorleesmodus breekt een wachtzin alleen na een scheidingsteken. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fase C afgerond: taak 12 t/m 17 (de app). |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fase B afgerond: taak 5 t/m 11. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Taak 3–5 en de overdracht naar de andere Mac. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Eerste entry voor PWSpeller 2. |

---

# Voortgang

## 2026-10-17, nacht (Claude Code) — Taak 18 fixronde: CI groen op drie platforms

- **Windows-paniek in `region` (`unlocking region: Err(SystemCall(158))`)**. Oorzaak: de sleutel van 32 bytes stond in een `Box`, en kleine allocaties delen een geheugenpagina (gemeten: drie `Box<[u8; 32]>` op één pagina van 16 KiB). Het besturingssysteem vergrendelt per pagina en telt niet mee: de eerste `VirtualUnlock` hief ook de lock van de andere geschiedenis op, de tweede faalde met ERROR_NOT_LOCKED (158), en de `debug_assert` in `LockGuard::drop` gaf een paniek. Op macOS/Linux faalt `munlock` niet, maar ontgrendelde de eerste drop stilletjes de pagina van de ander. Oplossing: de sleutel krijgt een eigen allocatie van precies één pagina, pagina-uitgelijnd (`std::alloc` met `Layout`), gelockt; bij drop eerst gewist, dan ontgrendeld (fout alleen gelogd), dan vrijgegeven. `region::alloc` is bewust niet gebruikt: `Allocation::drop` heeft dezelfde `debug_assert`.
- **Linux: "gegenereerde tekst in logboek"**. Gemeten met tijdelijke diagnose (lokaal na 40 runs gereproduceerd, ook op macOS): het woord `verzoek` uit de Nederlandse woordenlijst, in de logregel `geschiedenis gewist op verzoek`. Die regel kwam van een andere test: de logger is voor het hele testproces gedeeld. Geen lek, maar toeval (~2 % per run: `verzoek`, `afgesloten`, `geschiedenis` staan in de lijst én in vaste logteksten). Een woordgrens helpt hier niet, want het is een heel woord. Oplossing: het volledige wachtwoord en de volledige wachtzin worden nog steeds in het ruwe logboek gezocht; voor de losse woorden haalt de test eerst de vaste teksten van alle `log::…!`-aanroepen in de bron weg (automatisch uit de broncode gelezen), de ingevulde waarden blijven gecontroleerd.
- **Release-workflow**: tag eerst gevalideerd (`vX.Y.Z` met optioneel achtervoegsel; met achtervoegsel een prerelease), releasenotes met exacte kopmatch via `packaging/releasenotes.sh` (faalt als de sectie ontbreekt, geen terugval meer), fmt/clippy/tests op drie OS en een volledig blokkerende `cargo deny` vóór het bouwen. Geen `${{ }}`-expressies meer in `run:`; alles via `env:`. Overal `persist-credentials: false`.
- **CI**: advisories in een eigen job met `continue-on-error`, zodat een nieuw beveiligingsbericht zichtbaar faalt zonder de rest te blokkeren. `deny.toml` verbiedt nu ook native-tls, rustls, tokio, h2 en hyper-util. `build.rs`: in CI of met `PWSPELLER_VERSIE` laat een mislukte winresource-stap de build falen.
- **Licenties**: BSL-1.0 en de MPL-2.0-uitzondering (option-ext) zijn op 17 oktober 2026 door de opdrachtgever goedgekeurd; vastgelegd in NOTICE.md en deny.toml.

## 2026-10-17, nacht (Claude Code) — Taak 18: licenties, CI en uitlevering

- **`deny.toml`**: licenties zoals de randvoorwaarden, plus `BSL-1.0` (Boost, permissief; `clipboard-win` en `error-code` via arboard, alleen Windows) en een per-crate-uitzondering voor `MPL-2.0` bij `option-ext` (via `directories`). MPL-2.0 is copyleft op bestandsniveau; de crate wordt onveranderd gebruikt. Gemeld in NOTICE.md. Verboden: reqwest, ureq, hyper, curl, openssl. `cargo deny check` is groen (licenses, bans, advisories, sources).
- **Workflows**: `ci.yml` (fmt, clippy, test op drie OS, deny op Linux) en `release.yml` (tag `v*`: Windows x64 en ARM64, macOS universeel met `lipo` en `maak-app.sh`, Linux x64; `gh release create`). Actions staan vast op de commit-SHA van een major-versie; schrijfrechten alleen in de release-job. Toolchain via `rustup` op de runner; alleen `cargo-deny-action` (Embark) is van een derde partij.
- **Alleen gecontroleerd op de Mac**: de workflows zelf draaien pas na de push. Windows-ARM64 wordt gekruist op de x64-runner gebouwd (test draait daar niet) en de `dumpbin`-controle is niet lokaal getest.
- **Icoon**: 1024 px, logo gecentreerd op wit; `AppIcon.icns` via `iconutil`, `app.ico` met PNG-lagen van 16 tot 256 px, ingebed via `winresource` in `build.rs` (alleen bij doel Windows).
- **macOS-bundel** lokaal gebouwd en gestart: ad-hoc ondertekend met sandbox, zonder `network.client`. Instellingen en logboek staan in de sandbox-container, `~/Library/Containers/nl.petisys.pwspeller/Data/Library/Application Support/nl.Petisys.PWSpeller/`.

## 2026-10-17, avond (Claude Code) — visuele controle van de app

Met toestemming voor schermopname kon de app voor het eerst echt bekeken worden (alleen het eigen venster, via `screencapture -l`). Dat vond wat de tests misten:

- **Voorleesmodus toonde het teken en de spelling niet.** De knoppenrij onderin (`with_layout(left_to_right(Align::Center))`) nam alle hoogte en duwde het midden weg. De tests controleerden alleen of alles binnen het venster lag; nu meten ze de hoogte en de plaats van het teken.
- **Groepjes van vier sneden wachtzinnen in woorden**, en bij eigen tekst leek de code `U+0020` van een spatie op extra cijfers. Besluit Hans: groepjes alleen bij een gegenereerd wachtwoord; woorden uit de woordenlijsten blijven heel; eigen tekst doorlopend, met een spatie als grijze `·`-markering.
- **Splashscreen 1,5 seconde** in plaats van drie: afspraak 24 is daarvoor globaal aangepast (mac-setup), op verzoek van Hans.
- Kleinigheden: "Cijfer aan elk woord" heet nu "Cijfer toevoegen" (er komt één cijfer), en de spellinglijst nummert doorlopend.

## 2026-10-17, later (Claude Code) — fase C afgerond: taak 12 t/m 17

De app staat: splashscreen, hoofdvenster met drie tabbladen, klembord, voorleesmodus, geschiedenis- en instellingenpaneel. Elke taak is gereviewd; de belangrijkste vondsten:

- **Splash:** het logo was 2.639 px breed en liet egui vastlopen op de maximale textuurmaat (2.048); het asset is nu 640 px. Het versienummer bleef staan na een nieuwe commit omdat `build.rs` alleen `.git/HEAD` volgde.
- **Hoofdvenster:** de spelling werd elk beeld (60 per seconde) opnieuw opgebouwd, met ongewiste kopieën van het wachtwoord; nu in een cache die zichzelf wist. Enter maakte een nieuw wachtwoord terwijl een knop focus had.
- **Klembord:** op macOS standaard niet gewist (wens van Hans); wel gemarkeerd als vertrouwelijk (`ConcealedType`, nagekeken in de broncode van arboard). Op Linux onder X11 bestaat zo'n markering niet.
- **Voorleesmodus:** paste eerst niet in het kleinste venster (480 × 560); nu scrollt het midden en schaalt het teken mee.
- **Geschiedenis:** na inactiviteit wordt nu ook het wachtwoord op het scherm gewist, en elke toets of klik telt als gebruik. De spec beloofde wissen "bij afmelden"; dat is aangepast: bij normaal afsluiten wordt gewist, bij een geforceerd einde verdwijnt het geheugen met het proces maar blijft het klembord staan.
- **Niet gemeten:** het echte venster is niet visueel bekeken (schermopname is hier geweigerd), en wissen bij een geminimaliseerd venster is niet nagemeten.

## 2026-10-17 (Claude Code) — fase B afgerond: taak 5 t/m 11

Op de Mac mini verder gewerkt, met een subagent en een aparte review per taak.

- **Taak 5:** de review vond vier woorden die niet aan een klant voorgelezen horen te worden (`idioot`, `zwijn`, `stupide`, `satan`); die staan nu op de blokkeerlijsten. Een langere lijst met beladen woorden ligt bij Hans ter beoordeling.
- **Taak 6:** de taaldata wordt bij het bouwen ingebakken; een onvolledig taalbestand laat de build falen met bestand en teken. `interfacetekst` gaf bij een onbekende sleutel eerst geheugen weg met `Box::leak`; de signatuur heeft nu een levensduur.
- **Taak 10 en 7:** sterkte en wachtwoordgenerator. De review vond dat te grote minima een wachtwoord langer dan 128 tekens gaven, dat een tussenkopie niet gewist werd, en dat er geen vaste functie met de OS-bron was. Opgelost: `genereer_wachtwoord_os`, `Zeroizing<Vec<u8>>`, grenzen 1–9 en ≤ 128, rand-feature `unbiased`.
- **Taak 8:** wachtzingenerator; woorden staan in `Zeroizing`. Een scheidingsteken dat een cijfer of letter is, breekt het spellen niet omdat de woorden apart worden bewaard.
- **Taak 9:** spelling per teken en per woord, met lastige klanken per letter.
- **Taak 11:** versleutelde geschiedenis (XChaCha20-Poly1305, sleutel gelockt in het geheugen). De review wees erop dat `Instant` op macOS en Linux niet doorloopt in slaapstand: een dichtgeklapte laptop hield de geschiedenis dan langer vast. Nu telt ook de wandklok; de eerste die de time-out haalt, wist.
- `cargo fmt` over de hele werkruimte gedraaid, zodat de latere CI-controle niet struikelt.

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
- Na de push: kijken of `ci.yml` op alle drie de OS groen wordt, dan tag `v2.0.0`; het eerste echte `release.yml`-resultaat (met name `dumpbin` en ARM64) moet nog blijken.
- Ondertekenen en app stores: zie het ontwerp, hoofdstuk 14.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
