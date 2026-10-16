## Over dit bestand

- **Doel:** legt per werksessie vast wat er veranderd is en waarom, nieuwste bovenaan.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fix: tabblad Wachtzin toont hele woorden met scheidingsteken en woordnummer, niet groepjes van vier die woorden middenin afbraken; wachtwoord en eigen tekst blijven groepjes. De voorleesmodus breekt een wachtzin alleen na een scheidingsteken. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fase C afgerond: taak 12 t/m 17 (de app). |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fase B afgerond: taak 5 t/m 11. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Taak 3–5 en de overdracht naar de andere Mac. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Eerste entry voor PWSpeller 2. |

---

# Voortgang

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
- Ondertekenen en app stores: zie het ontwerp, hoofdstuk 14.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
