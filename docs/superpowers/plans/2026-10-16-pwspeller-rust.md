## Over dit bestand

- **Doel:** het implementatieplan voor PWSpeller 2, taak voor taak, afgeleid van de goedgekeurde spec.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, nog geen release (volgt als v2.0.0)
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Plan opgesteld. |

---

# PWSpeller 2 — implementatieplan

> **Voor agents:** VERPLICHTE SUB-SKILL: gebruik superpowers:subagent-driven-development (aanbevolen) of superpowers:executing-plans om dit plan taak voor taak uit te voeren. Stappen gebruiken checkboxen (`- [ ]`). Alles wat je aan Hans teruggeeft, schrijf je in het Nederlands.

**Doel:** PWSpeller herbouwen in Rust met egui: een wachtwoord- en wachtzingenerator naar het concept van Bitwarden, die meertalig spelt voor telefonisch doorgeven, als één bestand per platform.

**Architectuur:** Een Cargo-werkruimte met drie crates. `pwspeller-data` (model, inlezen en controleren van de databestanden) wordt zowel door het buildscript van de kern als door de kern zelf gebruikt, zodat een onvolledig taalbestand de build laat falen. `pwspeller-kern` bevat generator, spelling, sterkte en geschiedenis zonder interface. `pwspeller` (de app) is een dunne egui-laag.

**Techniek:** Rust stable (via rustup), eframe/egui, egui_kittest, rand, serde, toml, chacha20poly1305, zeroize, region, arboard, directories, sys-locale, chrono, cargo-deny, GitHub Actions.

**Spec:** [docs/superpowers/specs/2026-10-16-pwspeller-rust-design.md](../specs/2026-10-16-pwspeller-rust-design.md). Lees die naast dit plan; de spec beslist bij twijfel.

## Algemene randvoorwaarden

- Alle code, commentaar, commitberichten en meldingen in het Nederlands. Identifiers in het Nederlands, zoals in dit plan.
- Afhankelijkheden toevoegen met `cargo add` (actuele versie). Controleer de API tegen de geïnstalleerde versie op docs.rs; `rand` verschilt tussen 0.9 en 0.10 (`Rng`/`RngExt`).
- Alleen licenties MIT, Apache-2.0, BSD-2/3, ISC, Zlib, Unicode-3.0, OFL-1.1, CC-BY-3.0 (EFF-lijst). Bewaakt door `cargo deny check licenses` (Taak 18).
- Geen netwerkcode, geen telemetrie, geen updatecontrole. Geen crate met een HTTP-client.
- eframe zonder de feature `persistence`.
- Willekeur alleen uit de bron van het besturingssysteem (`rand::rng()` of `OsRng`). In tests een `StdRng` met vaste seed.
- Wachtwoorden en ontsleutelde geschiedenis in `zeroize::Zeroizing<String>`.
- Merk Petisys: zwart `#111312`, goud `#927C43` (alleen accenten), donkergoud `#6D5D32` (tekst), antraciet `#2B2A26`, grijs `#595959`, lichtgrijs `#E8E8E8`.
- Elk bestand dat je maakt of wijzigt en commentaar toestaat, krijgt bovenaan het blok "Over dit bestand" en onderaan het slotblok (afspraak 12). Voor Rust en TOML:

  ```
  // ── Over dit bestand ──────────────────────────────────────
  // Doel:           <één regel: waarvoor dit bestand er is>
  // Klant:          Petisys
  // Gemaakt vanuit: Petisys
  // Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
  // Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
  //
  // 2026-10-16  Hans van der Pluijm (Petisys)  [Claude Code]
  //   <wat>
  // ──────────────────────────────────────────────────────────
  ...
  // © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
  // Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
  ```

  In TOML met `#` in plaats van `//`. Woordenlijsten (`.txt`) en lockfiles krijgen geen blok.
- Elke commit met expliciete paden; controleer met `git show --stat` vóór een push. Bij een commit die naar GitHub gaat, eerst `RELEASENOTES.md` bijwerken (afspraak 22).
- **Stoppunten (afspraak 14):** aan het eind van elke fase (A, B, C, D) stoppen, melden wat er staat, en Hans vragen of de volgende fase mag beginnen.

## Aandachtspunten voor de review

1. **Eigen tekst met vreemde invoer:** emoji, combinerende tekens, tabs, een plaksel van 10.000 tekens. Verwacht: geen crash, elk teken een regel, onbekende tekens als "teken X (U+XXXX)", en geen merkbare vertraging. Test in Taak 9.
2. **Wachtzin met een scheidingsteken dat een letter, een cijfer of meer tekens is** (bijvoorbeeld `7` of `--`). Verwacht: correcte spelling, omdat het spellen de woorden als lijst krijgt en de tekst niet opnieuw splitst. Test in Taak 8 en 9.
3. **Minima groter dan de lengte, of de laatste tekenset uitgezet.** Verwacht: de lengte wordt opgehoogd; de laatste set laat zich niet uitzetten (kern geeft `GeenTekensetAan`). Test in Taak 7 en 14.
4. **Klembord bij het sluiten:** op macOS standaard onaangeroerd; op Windows en Linux alleen wissen als ons wachtwoord er nog op staat. Test in Taak 15.
5. **Geschiedenis vervalt door inactiviteit terwijl een item open staat** (in de voorleesmodus). Verwacht: geen crash, het scherm meldt dat de geschiedenis is gewist. Test in Taak 11 en 17.

---

## Fase A — Voorbereiding

### Taak 1: Toolchain installeren en de VB-versie veiligstellen

**Bestanden:**
- Verwijderen: `PWSpeller/`, `Installer/`, `PWSpeller.sln`

- [ ] **Stap 1: Rust installeren.** `brew install rustup`, daarna `rustup-init -y --no-modify-path` en `rustup default stable`. Controle: `cargo --version` geeft een versie.
- [ ] **Stap 2: Vastleggen in mac-setup.** `~/mac-setup/scripts/sync.sh capture`. Controle: de uitvoer noemt rustup.
- [ ] **Stap 3: Tag de VB-versie.** `git tag -a v1.0-vb 79db8f8 -m "Laatste VB.NET-versie (WinForms, .NET Framework 4.8)"` en `git push origin v1.0-vb`.
- [ ] **Stap 4: GitHub-release.** `gh release create v1.0-vb --title "PWSpeller 1.0 (VB.NET)" --notes "Laatste versie in VB.NET voor Windows, bewaard als backup. Opvolger: PWSpeller 2 (Rust)."` GitHub voegt de broncode als zip toe. Controle: `gh release view v1.0-vb` toont de release.
- [ ] **Stap 5: VB-bestanden verwijderen.** `git rm -r PWSpeller Installer PWSpeller.sln`. `.DS_Store` niet meenemen.
- [ ] **Stap 6: Commit** (nog niet pushen; Taak 2 maakt de releasenotes):

```bash
git commit -m "VB-versie verwijderd; bewaard als tag v1.0-vb" -- PWSpeller Installer PWSpeller.sln
```

### Taak 2: Werkruimte, basisdocumenten en kopbloktest

**Bestanden:**
- Maken: `Cargo.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, `crates/kern/Cargo.toml`, `crates/kern/src/lib.rs`, `crates/kern/tests/kopblok.rs`, `crates/data/Cargo.toml`, `crates/data/src/lib.rs`, `crates/app/Cargo.toml`, `crates/app/src/main.rs`, `README.md`, `RELEASENOTES.md`, `NOTICE.md`, `docs/implementatie.md`, `docs/voortgang.md`, `docs/assets/petisys-logo.jpeg`
- Wijzigen: `.gitignore` (alles van Visual Studio weg, `target/` en `.DS_Store` erin)

**Interfaces:**
- Levert: crates `pwspeller-data` (lib `pwspeller_data`), `pwspeller-kern` (lib `pwspeller_kern`), `pwspeller` (bin) in `[workspace] members = ["crates/*"]`, `resolver = "3"`, `edition = "2024"`.

- [ ] **Stap 1: Schrijf de falende kopbloktest** `crates/kern/tests/kopblok.rs`. Hij loopt door alle bestanden in de repo (behalve `target/`, `.git/` en `*.txt`) en zoekt naar "Over dit bestand". In code moet elke historieregel (`^// \d{4}-\d{2}-\d{2}` of `^# \d{4}-…`) eindigen op `[Claude Code]`, `[Copilot]`, `[Claude Code, Copilot]` of `[onbekend]`. In markdown moet de kolom "Met" van elke tabelrij met een datum `Claude Code`, `Copilot`, `Claude Code, Copilot` of `onbekend` bevatten.

```rust
#[test]
fn elke_historieregel_heeft_een_merkteken() {
    let fouten = controleer_repo(&repo_root());
    assert!(fouten.is_empty(), "Historieregels zonder geldig merkteken:\n{}", fouten.join("\n"));
}
#[test]
fn regel_zonder_merkteken_wordt_gevonden() {
    let fouten = controleer_tekst("x.rs", "// Over dit bestand\n// 2026-10-16  Hans (Petisys)\n");
    assert_eq!(fouten.len(), 1);
}
```

- [ ] **Stap 2:** `cargo test -p pwspeller-kern --test kopblok`. Verwacht: compileert niet (`controleer_repo` bestaat niet).
- [ ] **Stap 3: Implementeer** `fn controleer_tekst(pad: &str, inhoud: &str) -> Vec<String>`, `fn controleer_repo(root: &Path) -> Vec<String>` en `fn repo_root() -> PathBuf` (`CARGO_MANIFEST_DIR/../..`) in het testbestand zelf.
- [ ] **Stap 4: Maak de documenten.**
  - **Logo:** kopieer `~/mac-setup/skills/huisstijl/merken/petisys/logo-kopregel.jpeg` naar `docs/assets/petisys-logo.jpeg`.
  - **Logo-regel:** elk markdownbestand in de hoofdmap begint met `<p align="center"><img src="docs/assets/petisys-logo.jpeg" alt="Petisys" height="113"></p>`.
  - **README:** het logo, de titel, een korte introductie, dan "Over dit bestand", dan wat het is, voor wie, lokaal bouwen (`cargo run -p pwspeller`), uitrollen (zie Taak 18), een overzicht van alle documenten met relatieve links, en de vaste AI-tekst uit afspraak 12.
  - **NOTICE.md:** de vaste AI-tekst plus een lijst met overige rechthebbenden. Die lijst vult zich in latere taken.
  - **RELEASENOTES.md:** onder "Nog niet uitgebracht" de punten "Wat er verandert: PWSpeller wordt opnieuw gebouwd voor Windows, macOS en Linux; de vorige versie blijft beschikbaar als release v1.0-vb", "Nog te doen" en "Bekende problemen".
  - **docs/voortgang.md:** de entry van vandaag.
  - **docs/implementatie.md:** de opbouw en de afgewezen alternatieven uit de spec, hoofdstuk 13.
  - **`.cargo/config.toml`:** `rustflags = ["-C", "target-feature=+crt-static"]` voor `x86_64-pc-windows-msvc` en `aarch64-pc-windows-msvc`.
- [ ] **Stap 5:** `cargo test --workspace`. Verwacht: PASS.
- [ ] **Stap 6: Commit en push**, met expliciete paden, inclusief de commit van Taak 1. Controle: `git show --stat`.

**Stoppunt fase A.**

---

## Fase B — Data en kern

### Taak 3: Datamodel, inlezen en controleren (`pwspeller-data`)

**Bestanden:**
- Maken: `crates/data/src/lib.rs`, `crates/data/src/model.rs`, `crates/data/src/inlezen.rs`, `crates/data/src/controle.rs`
- Test: `crates/data/tests/controle.rs`

**Interfaces:**
- Levert (alles `pub`, `serde::Deserialize` waar het uit TOML komt):

```rust
pub struct Alfabet { pub code: String, pub naam: String, pub letters: BTreeMap<char, String> }
pub struct Sjablonen { pub hoofdletter: String, pub kleine_letter: String, pub cijfer: String,
                       pub woord: String, pub woord_met_hoofdletter: String, pub onbekend: String }
pub struct LastigeKlanken { pub bevat: Vec<String>, pub eindigt_op: Vec<String> }
pub struct Omschrijvingen { pub code: String, pub sjablonen: Sjablonen, pub cijfers: BTreeMap<char, String>,
                            pub tekens: BTreeMap<char, String>, pub lastige_klanken: LastigeKlanken }
pub struct Woordenlijst { pub code: String, pub woorden: Vec<String> }
pub struct Spellingstaal { pub code: String, pub naam: String, pub alfabet: String,
                           pub omschrijvingen: Option<String>, // None = taal van de interface
                           pub woordenlijst: String }
pub struct Catalogus { pub alfabetten: BTreeMap<String, Alfabet>, pub omschrijvingen: BTreeMap<String, Omschrijvingen>,
                       pub woordenlijsten: BTreeMap<String, Woordenlijst>, pub geblokkeerd: BTreeMap<String, BTreeSet<String>>,
                       pub spellingstalen: Vec<Spellingstaal>, pub interface: BTreeMap<String, BTreeMap<String, String>> }
pub struct DataFout { pub bestand: String, pub melding: String }  // Display: "<bestand>: <melding>"
pub fn lees_catalogus(bestanden: &[(&str, &str)]) -> Result<Catalogus, Vec<DataFout>>
pub fn controleer(c: &Catalogus) -> Vec<DataFout>
```

- **Herkenning op pad:** `alfabetten/<code>.toml`, `omschrijvingen/<code>.toml`, `interface/<code>.toml` (plat, sleutel = tekst), `woordenlijsten/<code>.txt` (één woord per regel; lege regels negeren), `woordenlijsten/<code>-geblokkeerd.txt` en `spellingstalen.toml` (`[[taal]]`-tabellen).
- **Vorm van de TOML-bestanden:** zie spec §4. Sleutels in `[letters]`, `[cijfers]` en `[tekens]` zijn strings van precies één teken; een andere lengte is een `DataFout`.

- [ ] **Stap 1: Schrijf de falende tests** in `crates/data/tests/controle.rs`, met kleine databestanden als strings in de test:

```rust
#[test] fn alfabet_zonder_q_geeft_fout_met_bestand_en_letter() {
    // alfabet met a..z behalve q
    let f = controleer(&cat);
    assert!(f.iter().any(|x| x.bestand == "alfabetten/nl.toml" && x.melding.contains("'q'")));
}
#[test] fn omschrijvingen_moeten_ascii_32_tot_126_dekken() {
    // tekens zonder '~'
    assert!(f.iter().any(|x| x.bestand == "omschrijvingen/nl.toml" && x.melding.contains("'~'")));
}
#[test] fn combinatie_naar_onbekend_alfabet_is_fout() { /* spellingstaal alfabet = "xx" */ }
#[test] fn interface_mist_sleutel_uit_en() { /* nl.toml mist "knop.genereer" */ }
#[test] fn woord_korter_dan_5_is_fout() { /* "kat" in nl.txt */ }
#[test] fn woord_met_hoofdletter_of_accent_is_fout() { /* "Appel", "café" */ }
#[test] fn dubbel_woord_is_fout() {}
#[test] fn geblokkeerd_woord_in_lijst_is_fout() { /* "moord" in nl.txt en nl-geblokkeerd.txt */ }
#[test] fn sleutel_van_twee_tekens_is_fout() { /* [letters] ab = "x" */ }
#[test] fn complete_set_geeft_geen_fouten() { assert!(controleer(&cat).is_empty()); }
```

- **Regels voor woordenlijsten:** voor `en-eff` geldt alleen a–z en uniek (de EFF-lijst heeft woorden van 3 en 4 letters). De regel "minimaal 5" geldt voor elke andere lijst.
- **Wat de omschrijvingen moeten dekken:** samen met `[cijfers]` alle ASCII-tekens van 32 tot en met 126 die geen letter of cijfer zijn.

- [ ] **Stap 2:** `cargo test -p pwspeller-data`. Verwacht: FAIL (functies ontbreken).
- [ ] **Stap 3: Implementeer** `lees_catalogus` en `controleer` met `serde` en `toml`. Verzamel alle fouten; stop niet bij de eerste.
- [ ] **Stap 4:** `cargo test -p pwspeller-data`. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 4: Alfabetten, omschrijvingen, interfaceteksten en combinaties

**Bestanden:**
- Maken: `data/alfabetten/{nl,vl,fr,de,navo}.toml`, `data/omschrijvingen/{nl,en,fr,de}.toml`, `data/interface/{nl,en}.toml`
- Test: `crates/data/tests/echte_data.rs`

**Interfaces:**
- Gebruikt: `lees_catalogus` en `controleer` uit Taak 3.
- Levert: alfabetten `nl`, `vl`, `fr`, `de`, `navo`; omschrijvingen `nl`, `en`, `fr`, `de`; interface `nl`, `en`. De combinaties (`spellingstalen.toml`) volgen in Taak 5, omdat ze naar de woordenlijsten verwijzen.

- [ ] **Stap 1: Schrijf de falende test.** Die leest alle bestanden onder `data/` van schijf (`CARGO_MANIFEST_DIR/../../data`):

```rust
#[test] fn echte_data_is_volledig() {
    let fouten = controleer(&lees_catalogus(&bestanden).unwrap());
    assert!(fouten.is_empty(), "{}", fouten.iter().map(|f| f.to_string()).collect::<Vec<_>>().join("\n"));
}
#[test] fn nederlands_a_is_anna_en_navo_a_is_alfa() { /* letters['a'] */ }
```

- [ ] **Stap 2:** `cargo test -p pwspeller-data --test echte_data`. Verwacht: FAIL.
- [ ] **Stap 3: Codewoorden nazoeken** (afspraak 1) en de bestanden schrijven:
  - **nl:** het Nederlandse spellingsalfabet uit de VB-versie (`git show v1.0-vb:PWSpeller/talen.csv`, kolom 1, letters a–z), zonder de voorvoegsels.
  - **vl:** uit kolom 3 van hetzelfde bestand. Let op de Windows-1252-codering: "Desiré".
  - **fr:** het Franse alfabet (Anatole, Berthe, Célestin …).
  - **de:** DIN 5009:2022 (Aachen, Berlin, Chemnitz …).
  - **navo:** ICAO/NAVO (Alfa, Bravo, Charlie …).

  Zoek fr, de en navo na tegen een actuele bron en zet die bron in een commentaarregel van het bestand. De omschrijvingen schrijf je in de eigen taal. `en` gebruikt "capital {woord}" en "small {woord}". De lastige klanken: nl `["ij","ei","au","ou"]` met `eindigt_op = ["d","t"]`. Voor de andere talen leeg, tenzij een bron iets voor de hand liggends geeft. De interfacesleutels volgen in Taak 12–17. Begin met de sleutels die daar genoemd worden, en zet alle sleutels in zowel `nl` als `en`.
- [ ] **Stap 4:** dezelfde test. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 5: Woordenlijsten

**Bestanden:**
- Maken: `data/woordenlijsten/en-eff.txt`, `data/woordenlijsten/{nl,fr,de}.txt`, `data/woordenlijsten/{nl,fr,de}-geblokkeerd.txt`
- Maken: `data/spellingstalen.toml` met de zes combinaties uit spec §4 (codes `nl`, `vl`, `fr`, `de`, `en`, `navo`; `navo` heeft `omschrijvingen = None`; woordenlijsten nl, nl, fr, de, en-eff, en-eff)
- Test: uitbreiden van `crates/data/tests/echte_data.rs`
- Wijzigen: `NOTICE.md`

- [ ] **Stap 1: Schrijf de falende test:**

```rust
#[test] fn eff_lijst_zonder_koppeltekens_en_niet_leeg() { assert!(eff.woorden.len() > 7000 && eff.woorden.iter().all(|w| !w.contains('-'))); }
#[test] fn nl_lijst_bevat_geen_geblokkeerde_woorden() { /* door controle() al gedekt; expliciet: "moord", "porno", "schijt" niet in nl */ }
#[test] fn nl_lijst_heeft_minstens_7500_woorden() {}
#[test] fn fr_en_de_lijsten_hebben_minstens_5500_woorden() {}
#[test] fn frans_en_duits_gebruiken_eigen_woordenlijst() { /* spellingstaal fr → "fr", de → "de" */ }
#[test] fn zes_spellingstalen_in_vaste_volgorde() { assert_eq!(codes, ["nl", "vl", "fr", "de", "en", "navo"]); }
```

- [ ] **Stap 2:** draai de test. Verwacht: FAIL.
- [ ] **Stap 3: Maak de lijsten.**
  - **en-eff.txt:** haal `https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt` op in een eigen map in de scratchpad, haal de dobbelsteennummers eraf (`cut -f2`), en filter de woorden met `-` weg.
  - **Bronnen:** `~/Downloads/nederlandse_woorden_8000_min5.txt`, `~/Downloads/mots_francais_8000_min5.txt` en `~/Downloads/deutsche_woerter_8000_min5.txt` (UTF-8). Kopieer ze eerst naar een eigen map in de scratchpad en bewerk alleen die kopie.
  - **Omzetten naar a–z:** zet de Duitse lijst eerst naar kleine letters. Houd daarna voor fr en de alleen de woorden over die volledig uit a–z bestaan; woorden met é, è, ç, ä, ö, ü, ß enz. vallen weg (spec §4). Verwacht ongeveer 5.743 (fr) en 6.632 (de) woorden.
  - **`<code>-geblokkeerd.txt`:** stel per taal een concept op: geweld, seks, drugs, scheldwoorden (ook Engelse leenwoorden, zoals "bitch" in de Nederlandse lijst), ziekte en dood, en beledigende woorden ("achterlijk"). Zet de woorden alfabetisch.
  - **`nl.txt`, `fr.txt`, `de.txt`:** de omgezette lijst min de geblokkeerde woorden, alfabetisch.
  - **NOTICE.md:** de EFF-lijst (CC BY 3.0, met link) en "Nederlandse, Franse en Duitse woordenlijst: gegenereerd met Claude in opdracht van Petisys".
- [ ] **Stap 4:** draai de test. Verwacht: PASS.
- [ ] **Stap 5: Commit.** Meld Hans dat de drie blokkeerlijsten klaarliggen voor zijn goedkeuring (openstaand punt in de spec).

### Taak 6: Data inbakken in de kern

**Bestanden:**
- Maken: `crates/kern/build.rs`, `crates/kern/src/talen.rs`
- Test: `crates/kern/tests/talen.rs`

**Interfaces:**
- Gebruikt: `pwspeller_data::{lees_catalogus, controleer, Catalogus}`. `pwspeller-data` is zowel een `[build-dependencies]` als een `[dependencies]`.
- Levert:

```rust
pub fn catalogus() -> &'static Catalogus                       // OnceLock, uit ingebakken bestanden
pub fn interfacetekst(taal: &str, sleutel: &str) -> &'static str // taal → "en" → de sleutel zelf
pub fn omschrijvingen_voor(t: &Spellingstaal, interfacetaal: &str) -> &'static Omschrijvingen
    // t.omschrijvingen of anders de interfacetaal, met terugval "en"
```

- [ ] **Stap 1: Schrijf de falende tests:**

```rust
#[test] fn ingebouwde_catalogus_heeft_zes_spellingstalen() { assert_eq!(catalogus().spellingstalen.len(), 6); }
#[test] fn onbekende_interfacetaal_valt_terug_op_engels() { assert_eq!(interfacetekst("xx", "knop.genereer"), interfacetekst("en", "knop.genereer")); }
#[test] fn onbekende_sleutel_geeft_sleutel_terug() { assert_eq!(interfacetekst("nl", "bestaat.niet"), "bestaat.niet"); }
#[test] fn navo_gebruikt_omschrijvingen_van_de_interfacetaal() { /* code "nl" bij interfacetaal nl, "en" bij de */ }
```

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer `build.rs`.**
  1. Lees alles onder `../../data` en roep `lees_catalogus` en `controleer` aan.
  2. Bij fouten: `panic!` met alle fouten, één per regel, zodat `cargo build` faalt met bestand en teken.
  3. Schrijf `$OUT_DIR/bestanden.rs` met `pub static BESTANDEN: &[(&str, &str)] = &[("alfabetten/nl.toml", include_str!("<absoluut pad>")), …];`.
  4. Zet `cargo:rerun-if-changed` op de map `data`.

  `talen.rs` doet `include!(concat!(env!("OUT_DIR"), "/bestanden.rs"))`.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS. Controleer met de hand: haal tijdelijk `q` uit `data/alfabetten/nl.toml`, dan moet `cargo build -p pwspeller-kern` falen met `alfabetten/nl.toml` en `'q'` in de melding. Zet het daarna terug.
- [ ] **Stap 5: Commit.**

### Taak 7: Wachtwoordgenerator

**Bestanden:**
- Maken: `crates/kern/src/generator/mod.rs`, `crates/kern/src/generator/wachtwoord.rs`
- Test: `crates/kern/tests/wachtwoord.rs`

**Interfaces:**
- Gebruikt: `sterkte::wachtwoord_bits` (Taak 10, die vóór deze taak wordt uitgevoerd).
- Levert:

```rust
pub enum GeneratorFout { GeenTekensetAan, OngeldigeLengte, OngeldigAantalWoorden, LeegScheidingsteken }
pub enum Modus { Wachtwoord, Wachtzin }
pub struct Gegenereerd {
    pub tekst: Zeroizing<String>,
    pub modus: Modus,
    pub woorden: Option<Vec<String>>,       // alleen wachtzin: woorden mét eventuele hoofdletter en cijfer
    pub scheidingsteken: Option<String>,    // alleen wachtzin
    pub entropie_bits: f64,
}
pub struct WachtwoordOpties { pub lengte: u8, pub hoofdletters: bool, pub kleine_letters: bool, pub cijfers: bool,
    pub speciaal: bool, pub min_cijfers: u8, pub min_speciaal: u8, pub vermijd_dubbelzinnig: bool }
// Default: 14, true, true, true, false, 1, 1, false
pub const SPECIALE_TEKENS: [char; 8] = ['!', '@', '#', '$', '%', '^', '&', '*'];
pub const DUBBELZINNIG: [char; 5] = ['I', 'O', 'l', '0', '1'];
pub fn effectieve_lengte(o: &WachtwoordOpties) -> u8   // max(lengte, 5, som minima), daarna ≤ 128
pub fn genereer_wachtwoord<R: Rng + ?Sized>(o: &WachtwoordOpties, rng: &mut R) -> Result<Gegenereerd, GeneratorFout>
```

- [ ] **Stap 1: Schrijf de falende tests** (`StdRng::seed_from_u64`):

```rust
#[test] fn standaard_geeft_14_tekens_met_hoofd_klein_en_cijfer() {}
#[test] fn minima_worden_gehaald() { /* min_cijfers 9, min_speciaal 9, lengte 20: ≥9 cijfers, ≥9 speciaal, 1000 keer */ }
#[test] fn set_die_aanstaat_komt_minstens_een_keer_voor() { /* min_speciaal 0 + speciaal aan → ≥1 speciaal */ }
#[test] fn minima_groter_dan_lengte_verhogen_lengte() { /* lengte 5, min 9+9, alle sets aan → effectieve_lengte 20 */ }
#[test] fn lengte_onder_5_wordt_5() {}
#[test] fn lengte_boven_128_is_fout() { assert_eq!(r, Err(GeneratorFout::OngeldigeLengte)); }
#[test] fn geen_set_aan_is_fout() { assert_eq!(r, Err(GeneratorFout::GeenTekensetAan)); }
#[test] fn dubbelzinnig_vermijden_sluit_IOl01_uit() { /* 1000 keer lengte 128 */ }
#[test] fn alleen_speciale_tekens_uit_de_acht() {}
#[test] fn entropie_is_lengte_maal_log2_pool() { /* standaard: 14 * log2(62) */ }
#[test] fn verdeling_is_gelijkmatig() {
    // 200_000 tekens met alleen kleine letters; chi-kwadraat over 26 vakjes < 52,6 (p = 0,001, 25 vrijheidsgraden)
}
```

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer** volgens spec §5: eerst per set het minimum trekken (`max(ingesteld, 1)` als de set aanstaat), dan de rest uit de vereniging, dan `shuffle`. Trek uniform met `random_range`, nooit met `%`.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 8: Wachtzingenerator

**Bestanden:**
- Maken: `crates/kern/src/generator/wachtzin.rs`
- Test: `crates/kern/tests/wachtzin.rs`

**Interfaces:**
- Gebruikt: `Gegenereerd`, `GeneratorFout`, `Modus` (Taak 7); `Woordenlijst` (Taak 3).
- Levert:

```rust
pub struct WachtzinOpties { pub woorden: u8, pub scheidingsteken: String, pub hoofdletters: bool, pub cijfer: bool }
// Default: 6, "-", false, false
pub fn genereer_wachtzin<R: Rng + ?Sized>(o: &WachtzinOpties, lijst: &Woordenlijst, rng: &mut R) -> Result<Gegenereerd, GeneratorFout>
```

- [ ] **Stap 1: Schrijf de falende tests:**

```rust
#[test] fn standaard_zes_woorden_met_streepje() { /* woorden.len() == 6; tekst == woorden.join("-") */ }
#[test] fn twee_en_eenentwintig_woorden_zijn_fout() { /* OngeldigAantalWoorden */ }
#[test] fn leeg_scheidingsteken_is_fout() {}
#[test] fn hoofdletters_maken_eerste_letter_groot() {}
#[test] fn cijfer_staat_achter_precies_een_woord() { /* precies één woord eindigt op [0-9] */ }
#[test] fn scheidingsteken_7_of_dubbel_streepje_blijft_in_woorden_gescheiden() {
    // scheidingsteken "7", cijfer aan: woorden-veld bevat 6 items, geen enkel item bevat het scheidingsteken
}
#[test] fn entropie_zonder_en_met_cijfer() { /* 6*log2(n) ; + log2(10*6) */ }
```

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer.** Kies elk woord uniform met `choose`, het cijfer met `random_range(0..=9)` achter een woord met index `random_range(0..n)`, en zet daarna de hoofdletters.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 9: Spelling

**Bestanden:**
- Maken: `crates/kern/src/spelling.rs`
- Test: `crates/kern/tests/spelling.rs`

**Interfaces:**
- Gebruikt: `Alfabet`, `Omschrijvingen` (Taak 3); `catalogus()` (Taak 6).
- Levert:

```rust
pub enum Soort { Hoofdletter, KleineLetter, Cijfer, Speciaal, Spatie, Woord, Onbekend }
pub struct Regel { pub positie: usize /* 1-based, in tekens (chars) */, pub fragment: String,
                   pub omschrijving: String, pub soort: Soort, pub lastig: bool }
pub struct Speller<'a> { /* alfabet, omschrijvingen */ }
impl<'a> Speller<'a> {
    pub fn nieuw(alfabet: &'a Alfabet, omschrijvingen: &'a Omschrijvingen) -> Self;
    pub fn per_teken(&self, tekst: &str) -> Vec<Regel>;
    pub fn wachtzin(&self, woorden: &[String], scheidingsteken: &str) -> Vec<Regel>;
    pub fn is_lastig(&self, woord: &str) -> bool;
}
```

- [ ] **Stap 1: Schrijf de falende tests**, met de echte catalogus:

```rust
#[test] fn nl_per_teken() {
    assert_eq!(omschr(nl.per_teken("Ab7!")), ["hoofdletter Anna", "kleine letter Bernard", "cijfer zeven", "uitroepteken"]);
}
#[test] fn en_gebruikt_navo_met_capital() { assert_eq!(omschr(en.per_teken("A"))[0], "capital Alfa"); }
#[test] fn spatie_en_onbekend() {
    let r = nl.per_teken("a é");
    assert_eq!(r[1].soort, Soort::Spatie);
    assert_eq!(r[2].omschrijving, "teken é (U+00E9)");
    assert_eq!(r[2].soort, Soort::Onbekend);
}
#[test] fn emoji_en_lange_invoer_crashen_niet() {
    let tekst = "👍🏽\t".repeat(5000);
    assert_eq!(nl.per_teken(&tekst).len(), tekst.chars().count());
}
#[test] fn wachtzin_per_woord() {
    let w = vec!["Zonnig".into(), "tafel7".into(), "berg".into()];
    assert_eq!(omschr(nl.wachtzin(&w, "-")),
        ["Zonnig, met hoofdletter", "streepje", "tafel", "cijfer zeven", "streepje", "berg"]);
}
#[test] fn lastig_woord_wordt_per_letter_gespeld_en_gemarkeerd() {
    // "wijd": bevat "ij" → regels: kleine letter Willem, Izaak, Johan, Dirk; allemaal lastig == true
}
#[test] fn scheidingsteken_van_meer_tekens_wordt_per_teken_gespeld() { /* "--" → twee keer "streepje" */ }
#[test] fn posities_lopen_door_over_woorden_en_scheidingstekens() {}
```

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer.**
  - Vul de sjablonen in via `{woord}`, `{naam}`, `{teken}` en `{code}` (`U+XXXX`, vier of meer hoofdletter-hexcijfers).
  - Een woord in een wachtzin:
    1. Haal een cijfer aan het eind los; dat wordt een eigen regel.
    2. Is het woord lastig, spel het dan per letter met `lastig = true`.
    3. Anders: begint het met een hoofdletter, dan `woord_met_hoofdletter`, anders `woord`.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 10: Sterkte

**Bestanden:**
- Maken: `crates/kern/src/sterkte.rs`
- Test: `crates/kern/tests/sterkte.rs`

**Interfaces:**
- Levert:

```rust
pub enum Niveau { Zwak, Redelijk, Sterk }
pub fn niveau(bits: f64) -> Niveau            // < 60 Zwak, 60..=80 Redelijk, > 80 Sterk
pub fn wachtwoord_bits(lengte: usize, pool: usize) -> f64
pub fn wachtzin_bits(woorden: usize, lijst: usize, met_cijfer: bool) -> f64
pub fn schatting_bits(tekst: &str) -> f64     // pool: +26 kleine, +26 hoofd, +10 cijfers, +33 overige ASCII, +100 niet-ASCII
```

Taak 7 en 8 gebruiken `wachtwoord_bits` en `wachtzin_bits`: **voer deze taak uit vóór Taak 7.**

- [ ] **Stap 1: Schrijf de falende tests:** `niveau(59.9) == Zwak`, `niveau(60.0) == Redelijk`, `niveau(80.0) == Redelijk`, `niveau(80.1) == Sterk`; `wachtwoord_bits(14, 62)` ≈ 83,36; `wachtzin_bits(6, 7776, false)` ≈ 77,55; `schatting_bits("")` == 0; `schatting_bits("abc")` ≈ 3·log₂26.
- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer.**
- [ ] **Stap 4:** draai de tests. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 11: Versleutelde geschiedenis

**Bestanden:**
- Maken: `crates/kern/src/geschiedenis.rs`
- Test: `crates/kern/tests/geschiedenis.rs`

**Interfaces:**
- Gebruikt: `Gegenereerd` en `Modus` (Taak 7).
- Levert:

```rust
pub const MAX_ITEMS: usize = 100;
pub struct ItemInfo { pub id: u64, pub modus: Modus, pub tijdstip: std::time::SystemTime, pub spellingstaal: String }
pub struct Geschiedenis { /* sleutel: Box<Zeroizing<[u8; 32]>> (gelockt met `region::lock` waar mogelijk),
                             items: VecDeque<(ItemInfo, nonce, ciphertext)>, laatst_gebruikt: Instant, time_out: Option<Duration> */ }
impl Geschiedenis {
    pub fn nieuw(time_out: Option<Duration>) -> Self;           // nieuwe willekeurige sleutel
    pub fn voeg_toe(&mut self, g: &Gegenereerd, spellingstaal: &str, nu: Instant) -> u64;
    pub fn items(&self) -> Vec<ItemInfo>;                        // nieuwste eerst
    pub fn ontsleutel(&mut self, id: u64, nu: Instant) -> Option<Gegenereerd>;
    pub fn wis(&mut self);                                       // items weg, sleutel overschreven en vernieuwd
    pub fn controleer_inactiviteit(&mut self, nu: Instant) -> bool; // true als er gewist is
}
impl Drop for Geschiedenis { /* wis */ }
```

- **Versleuteling:** XChaCha20-Poly1305 met een nonce van 24 willekeurige bytes per item. De platte tekst is een eenvoudige eigen codering van `tekst`, `woorden` en `scheidingsteken`, zonder serde_json; schrijf naar `Zeroizing<Vec<u8>>`.

- [ ] **Stap 1: Schrijf de falende tests:**

```rust
#[test] fn toevoegen_en_ontsleutelen_geeft_hetzelfde_terug() { /* ook woorden en scheidingsteken bij wachtzin */ }
#[test] fn ciphertext_bevat_de_tekst_niet() { /* via #[cfg(test)] pub fn ruwe_bytes(&self, id) */ }
#[test] fn honderd_een_items_houdt_de_nieuwste_honderd() {}
#[test] fn wis_maakt_leeg_en_oude_id_geeft_none() {}
#[test] fn inactiviteit_wist_na_time_out() {
    // time_out 15 min; controleer_inactiviteit(nu + 14 min) == false; (nu + 15 min + 1 s) == true; items leeg
}
#[test] fn zonder_time_out_wordt_niet_gewist() {}
#[test] fn ontsleutelen_telt_als_gebruik() { /* ontsleutel op 10 min, controle op 20 min → niet gewist */ }
```

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer.** Mislukt `region::lock`, schrijf dan één `log::warn!` en ga door.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS.
- [ ] **Stap 5: Commit.** Werk `docs/implementatie.md` bij (kern en data) en commit dat mee.

**Stoppunt fase B.**

---

## Fase C — App

### Taak 12: App-basis: instellingen, taal, logboek, versie, lettertype en thema

**Bestanden:**
- Maken: `crates/app/build.rs`, `crates/app/src/main.rs`, `crates/app/src/app.rs`, `crates/app/src/instellingen.rs`, `crates/app/src/logboek.rs`, `crates/app/src/thema.rs`
- Maken: `crates/app/assets/AtkinsonHyperlegibleMono-Regular.ttf`, `-Bold.ttf`, `OFL.txt`, `petisys-logo.png`
- Test: `crates/app/tests/instellingen.rs`

**Interfaces:**
- Levert:

```rust
pub struct Instellingen { pub interfacetaal: Option<String> /* None = besturingssysteem */, pub spellingstaal: String,
    pub wachtwoord: WachtwoordOpties, pub wachtzin: WachtzinOpties, pub woordenlijst: Option<String> /* None = volgt spellingstaal */,
    pub klembord_wissen: bool, pub klembord_seconden: u32, pub geschiedenis_minuten: Option<u32> }
// Default: None, "nl", standaarden, standaarden, None, klembord_wissen = !cfg!(target_os = "macos"), 30, Some(15)
impl Instellingen { pub fn laad(pad: &Path) -> Self /* bij fout: Default + log */; pub fn bewaar(&self, pad: &Path) -> io::Result<()> }
pub fn instellingenpad() -> PathBuf   // directories::ProjectDirs::from("nl", "Petisys", "PWSpeller").config_dir()/instellingen.toml
pub fn interfacetaal(i: &Instellingen) -> String  // i.interfacetaal of sys_locale (eerste twee letters) als die in de catalogus staat, anders "en"
pub const VERSIE: &str = env!("PWSPELLER_VERSIE");
```

`WachtwoordOpties` en `WachtzinOpties` krijgen `Serialize`/`Deserialize` (feature in de kern).

- [ ] **Stap 1: Schrijf de falende tests:** heen en terug bewaren en laden geeft gelijke instellingen; een kapot bestand geeft `Default`; op macOS is `klembord_wissen` standaard false; `interfacetaal` met `Some("de")` (niet in de catalogus) geeft "en".
- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer.**
  - **`build.rs`, het versienummer:** `PWSPELLER_VERSIE` uit de omgeving (CI). Anders `git describe --tags --always`. Anders `CARGO_PKG_VERSION`. Zet het met `cargo:rustc-env`.
  - **Lettertype:** maak de statische instanties met `pip install fonttools` en `fonttools varLib.instancer AtkinsonHyperlegibleMono[wght].ttf wght=400` en `wght=700`. De bron is google/fonts `ofl/atkinsonhyperlegiblemono`. Laad ze als familie `Proportional`/`Monospace` en een eigen familie `"vet"`.
  - **Logo:** zet `logo-kopregel.jpeg` om naar PNG met `sips -s format png` en neem het op met `include_bytes!`.
  - **Thema:** licht en donker volgen het besturingssysteem (`ctx.options` met de `ThemePreference::System`-variant van de geïnstalleerde egui). Kleuren uit de randvoorwaarden. Cijfers blauw (`#1F5FBF` op licht, `#7FB0FF` op donker) en speciale tekens rood (`#B3261E` op licht, `#FF8A80` op donker), elk met contrast van minstens 4,5:1 tegen de achtergrond.
  - **Logboek:** een eigen `log::Log` die regels met tijdstip schrijft naar `ProjectDirs.data_dir()/pwspeller.log`.
  - **`main.rs`:** `#![windows_subsystem = "windows"]`, `eframe::run_native` met minimale venstermaat 480×560.
  - **NOTICE.md:** de OFL-vermelding erbij.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS. `cargo run -p pwspeller` opent een leeg venster in de juiste kleuren.
- [ ] **Stap 5: Commit.**

### Taak 13: Splashscreen

**Bestanden:**
- Maken: `crates/app/src/splash.rs`
- Wijzigen: `crates/app/src/app.rs`
- Test: `crates/app/tests/splash.rs`

**Interfaces:**
- Levert:

```rust
pub const SPLASHDUUR: Duration = Duration::from_secs(3);
pub enum Fase { Splash { start: Instant }, Hoofd }
pub fn volgende_fase(f: &Fase, nu: Instant) -> Fase
pub struct PwApp { pub fase: Fase, /* … */ }
impl PwApp { pub fn nieuw(cc: Option<&eframe::CreationContext>, nu: Instant) -> Self }
```

- [ ] **Stap 1: Schrijf de falende tests** met `egui_kittest::Harness`:

```rust
#[test] fn splash_toont_vijf_gegevens() {
    // eerste frame: labels "PWSpeller", "Klant: Petisys", VERSIE, "© 2026 Petisys. Alle rechten voorbehouden." en het logo (alt "Petisys")
}
#[test] fn splash_komt_voor_hoofdvenster() { /* eerste frame: geen knop met interfacetekst "knop.genereer" */ }
#[test] fn na_drie_seconden_hoofdvenster() {
    assert!(matches!(volgende_fase(&Fase::Splash { start: t0 }, t0 + Duration::from_millis(2999)), Fase::Splash { .. }));
    assert!(matches!(volgende_fase(&Fase::Splash { start: t0 }, t0 + SPLASHDUUR), Fase::Hoofd));
}
```

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer.** Het logo staat gecentreerd. De dunne lijn is goud `#927C43`. Vraag een herteken aan met `ctx.request_repaint_after` tot de splash voorbij is. Lukt het laden van het logo niet, dan gaat er een `log::warn!` uit en loopt de splash zonder logo door. Hij blokkeert het starten nooit.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 14: Hoofdvenster

**Bestanden:**
- Maken: `crates/app/src/hoofd.rs`, `crates/app/src/resultaat.rs`, `crates/app/src/opties.rs`
- Test: `crates/app/tests/hoofd.rs`

**Interfaces:**
- Gebruikt: `genereer_wachtwoord`, `genereer_wachtzin`, `Speller`, `sterkte::*`, `catalogus()`, `interfacetekst`.
- Levert: `pub enum Tab { Wachtwoord, Wachtzin, EigenTekst }` en `pub fn groepjes(tekst: &str) -> Vec<(usize /*startpositie 1-based*/, String)>` (groepjes van 4 tekens).
- **Interfacesleutels** (in `nl` en `en`): `tab.wachtwoord`, `tab.wachtzin`, `tab.eigen`, `knop.genereer`, `knop.kopieer`, `knop.voorlezen`, `knop.per_teken`, `optie.lengte`, `optie.hoofdletters`, `optie.kleine_letters`, `optie.cijfers`, `optie.speciaal`, `optie.min_cijfers`, `optie.min_speciaal`, `optie.dubbelzinnig`, `optie.woorden`, `optie.scheidingsteken`, `optie.woord_hoofdletter`, `optie.woord_cijfer`, `optie.woordenlijst`, `label.spellingstaal`, `sterkte.zwak`, `sterkte.redelijk`, `sterkte.sterk`, `sterkte.schatting`, `melding.moeilijk_teken`, `melding.lengte_opgehoogd`.

- [ ] **Stap 1: Schrijf de falende tests** (kittest, na de splash):

```rust
#[test] fn groepjes_van_vier() { assert_eq!(groepjes("abcdefghij"), [(1,"abcd"),(5,"efgh"),(9,"ij")]); }
#[test] fn genereer_toont_14_tekens_en_spelling() { /* klik knop.genereer; resultaat 14 tekens; 14 spellingregels */ }
#[test] fn laatste_tekenset_laat_zich_niet_uitzetten() { /* zet drie uit; de vierde checkbox is disabled */ }
#[test] fn min_cijfers_alleen_zichtbaar_als_cijfers_aan() {}
#[test] fn minima_groter_dan_lengte_tonen_melding_en_hogere_lengte() {}
#[test] fn wisselen_van_spellingstaal_spelt_opnieuw() { /* nl → en: eerste regel van "capital" */ }
#[test] fn eigen_tekst_met_e_accent_toont_waarschuwing() {}
#[test] fn wachtzin_tab_spelt_per_woord_en_knop_per_teken_schakelt() {}
```

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer** volgens spec §8.
  - **Resultaat:** in `Monospace`. Hoofdletters vet, cijfers blauw, speciale tekens rood. De positienummers staan klein erboven.
  - **Sneltoetsen:** Enter = genereer, F5 = voorlezen, Ctrl/Cmd+C = kopieer, maar alleen als het focus niet in een tekstveld staat.
  - **Bij elke generatie:** `geschiedenis.voeg_toe`. Instellingen worden bewaard bij elke wijziging.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 15: Klembord

**Bestanden:**
- Maken: `crates/app/src/klembord.rs`
- Test: `crates/app/tests/klembord.rs`

**Interfaces:**
- Levert:

```rust
pub trait Bord { fn zet_vertrouwelijk(&mut self, tekst: &str) -> Result<(), String>; fn lees(&mut self) -> Option<String>; fn wis(&mut self) -> Result<(), String>; }
pub struct Systeembord; // arboard
pub struct Klembordbeheer<B: Bord> { /* bord, hash van laatst gezet (SHA-256, geen platte tekst), gezet_op: Option<Instant> */ }
impl<B: Bord> Klembordbeheer<B> {
    pub fn kopieer(&mut self, tekst: &str, nu: Instant);
    pub fn tik(&mut self, nu: Instant, i: &Instellingen);   // wist na i.klembord_seconden als klembord_wissen
    pub fn bij_sluiten(&mut self, i: &Instellingen);        // wist als klembord_wissen
}
```

- [ ] **Stap 1: Schrijf de falende tests** met een nep-`Bord`:

```rust
#[test] fn wist_na_30_seconden_als_ingeschakeld() {}
#[test] fn wist_niet_als_gebruiker_iets_anders_kopieerde() {}
#[test] fn wist_niets_als_uitgeschakeld() { /* ook bij_sluiten raakt het bord niet aan: geen enkele aanroep */ }
#[test] fn bij_sluiten_wist_alleen_ons_wachtwoord() {}
```

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer** `Systeembord` met arboard. Gebruik de uitbreiding die de inhoud uitsluit van de klembordgeschiedenis (`exclude_from_history` in `SetExtApple`/`SetExtWindows`/`SetExtLinux`). Controleer op docs.rs welke platforms de geïnstalleerde versie ondersteunt. Ontbreekt hij op een platform, dan zet `zet_vertrouwelijk` daar gewone tekst, met een opmerking in `docs/implementatie.md`.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS. Controleer met de hand op de Mac: kopieer, en het klembord blijft staan na 30 seconden en na het sluiten.
- [ ] **Stap 5: Commit.**

### Taak 16: Voorleesmodus

**Bestanden:**
- Maken: `crates/app/src/voorlezen.rs`
- Test: `crates/app/tests/voorlezen.rs`

**Interfaces:**
- Levert: `pub struct Voorlezen { pub regels: Vec<Regel>, pub index: usize }` met `volgende()`, `vorige()` (begrensd op 0 en len−1) en `fn voortgang(&self) -> (usize, usize)` (1-based).
- **Interfacesleutels:** `voorlezen.voortgang` ("teken {n} van {totaal}"), `voorlezen.stop`.

- [ ] **Stap 1: Schrijf de falende tests:** spatie en → gaan vooruit, ← gaat terug, de grenzen houden, en Esc keert terug naar het hoofdvenster. De voortgangstekst na twee keer → op 14 tekens is "teken 3 van 14".
- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer.** Het hele venster wordt gebruikt. Bovenaan staat het volledige wachtwoord met de huidige groep gemarkeerd. In het midden staan het teken en de omschrijving groot (minstens 48 en 32 pt). Lastige woorden krijgen een markering.
- [ ] **Stap 4:** draai de tests. Verwacht: PASS.
- [ ] **Stap 5: Commit.**

### Taak 17: Geschiedenispaneel, wissen en niets op schijf

**Bestanden:**
- Maken: `crates/app/src/geschiedenis_paneel.rs`, `crates/app/src/instellingen_paneel.rs`
- Wijzigen: `crates/app/src/app.rs` (`on_exit`, `tik`)
- Test: `crates/app/tests/geschiedenis.rs`

**Interfaces:**
- Gebruikt: `Geschiedenis` (Taak 11), `Klembordbeheer` (Taak 15).
- **Interfacesleutels:** `geschiedenis.titel`, `geschiedenis.toon`, `geschiedenis.verberg`, `geschiedenis.wis`, `geschiedenis.leeg`, `geschiedenis.gewist_inactief`, `instellingen.titel`, `instellingen.interfacetaal`, `instellingen.systeem`, `instellingen.klembord_wissen`, `instellingen.klembord_seconden`, `instellingen.geschiedenis_minuten`, `instellingen.alleen_bij_sluiten`.

- [ ] **Stap 1: Schrijf de falende tests:**

```rust
#[test] fn items_worden_gemaskeerd_getoond() { /* "••••••••" tot klik op toon */ }
#[test] fn wis_knop_maakt_leeg() {}
#[test] fn inactiviteit_tijdens_voorlezen_uit_geschiedenis_crasht_niet() {
    // open item in voorleesmodus, tik op nu + 16 min → melding geschiedenis.gewist_inactief, terug naar hoofdvenster
}
#[test] fn on_exit_wist_geschiedenis_en_volgt_klembordinstelling() {}
#[test] fn geen_wachtwoord_in_instellingen_of_logboek() {
    // genereer 20 wachtwoorden in een tijdelijke map voor instellingen en logboek; sluit;
    // geen van de 20 teksten komt voor in instellingen.toml of pwspeller.log
}
```

Instellingenpad en logboekpad zijn injecteerbaar via `PwApp::nieuw_met_paden` voor tests.

- [ ] **Stap 2:** draai de tests. Verwacht: FAIL.
- [ ] **Stap 3: Implementeer.**
  - **`tik`:** wordt elk frame aangeroepen, met `ctx.request_repaint_after(1 s)` zolang er iets te wissen valt.
  - **`on_exit`:** wist de geschiedenis en roept `klembordbeheer.bij_sluiten` aan.
  - **Instellingenpaneel:** interfacetaal, klembord en de tijd voor de geschiedenis.
- [ ] **Stap 4:** `cargo test --workspace`. Verwacht: PASS. Doorloop daarna de app met de hand op de Mac: splash, genereren, spellen in alle zes de talen, voorlezen, geschiedenis en sluiten.
- [ ] **Stap 5: Commit.** Werk `docs/implementatie.md`, `docs/voortgang.md` en `RELEASENOTES.md` bij, en push.

**Stoppunt fase C.**

---

## Fase D — Bouwstraat en oplevering

### Taak 18: Licentiecontrole, CI en uitlevering per platform

**Bestanden:**
- Maken: `deny.toml`, `.github/workflows/ci.yml`, `.github/workflows/release.yml`, `packaging/macos/Info.plist`, `packaging/macos/PWSpeller.entitlements`, `packaging/macos/maak-app.sh`, `crates/app/assets/AppIcon.icns`, `crates/app/assets/app.ico`
- Wijzigen: `crates/app/build.rs` (Windows-icoon via `winresource`), `README.md` (uitrollen)

- [ ] **Stap 1: Licentiecontrole.** Maak `deny.toml` met de licenties uit de randvoorwaarden. `cargo install cargo-deny` en `cargo deny check licenses bans`. Verwacht: OK. Een HTTP-client (`reqwest`, `ureq`, `hyper`) staat onder `[bans] deny`.
- [ ] **Stap 2: `ci.yml`.** Draait bij elke push en PR op ubuntu, macos en windows: `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace` en `cargo deny check`. Linux heeft de pakketten nodig die eframe vraagt; zoek de actuele lijst na in de README van eframe.
- [ ] **Stap 3: `release.yml`.** Draait bij tag `v*` en zet `PWSPELLER_VERSIE=${{ github.ref_name }}`.
  - **Windows:** `x86_64-pc-windows-msvc` en `aarch64-pc-windows-msvc` geven `PWSpeller-<versie>-windows-x64.exe` en `-arm64.exe`. Controle in de job: `dumpbin /dependents` toont geen `VCRUNTIME`.
  - **macOS:** `aarch64` en `x86_64` worden met `lipo` samengevoegd. Daarna `maak-app.sh`, die `PWSpeller.app` bouwt met `Info.plist`, het icoon en een ad-hoc-ondertekening met `codesign -s - --entitlements` (sandbox aan, geen netwerk). Dat geeft `PWSpeller-<versie>-macos.zip`.
  - **Linux:** `x86_64-unknown-linux-gnu` geeft `pwspeller-<versie>-linux-x64`.
  - **Release:** `gh release create` met de bestanden en de releasenotes van die versie.
- [ ] **Stap 4: Lokaal controleren.** `packaging/macos/maak-app.sh` op de Mac. `codesign -d --entitlements - PWSpeller.app` toont `com.apple.security.app-sandbox` en geen `network.client`. De app start vanuit Finder.
- [ ] **Stap 5: Commit en push.** Controleer of `ci.yml` groen wordt: `gh run watch`.

### Taak 19: Documentatie afronden en v2.0.0 voorbereiden

**Bestanden:**
- Wijzigen: `README.md`, `RELEASENOTES.md`, `NOTICE.md`, `docs/implementatie.md`, `docs/voortgang.md`
- Maken: `SKILLS.md` (via `~/mac-setup/scripts/sync.sh skills ~/Repos/Petisys/PWSpeller`)

- [ ] **Stap 1: Werk de documenten bij.**
  - **README:** downloads, de waarschuwing bij de eerste start zonder ondertekening (macOS: rechtsklik → Open; Windows: SmartScreen → Meer info → Toch uitvoeren), en hoe je een taal toevoegt (stappen plus een verwijzing naar `data/`).
  - **implementatie.md:** volledig.
  - **RELEASENOTES:** "Nog niet uitgebracht" wordt "2.0.0 — <datum>", in gewone taal voor helpdesk en engineers.
- [ ] **Stap 2:** `cargo test --workspace`, inclusief de kopbloktest. Verwacht: PASS.
- [ ] **Stap 3: Commit en push.**
- [ ] **Stap 4: Stoppunt fase D.** Vraag Hans om akkoord op het plaatsen van tag `v2.0.0`. Na akkoord: `git tag -a v2.0.0 -m "PWSpeller 2.0.0"` en `git push origin v2.0.0`. Controleer met `gh run watch` en `gh release view v2.0.0` dat er vijf bestanden bij staan.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
