## Over dit bestand

- **Doel:** het uitvoeringslogboek van het implementatieplan: welke taken af zijn, met welke commits en welke beslissingen, zodat het werk op een andere Mac verder kan.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, branch pwspeller-2 (nog geen release)
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Logboek bijgewerkt tot en met fase B (taak 5–11). |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Logboek tot en met Taak 5 vastgelegd voor de overdracht. |

---

# SDD ledger — plan: docs/superpowers/plans/2026-10-16-pwspeller-rust.md
Branch: pwspeller-2 (vanaf master 516044c). Spec: docs/superpowers/specs/2026-10-16-pwspeller-rust-design.md

## Preflight-scan
| Taken | Gedeeld | Bevinding |
|---|---|---|
| 1↔2 | commit T1 wordt in T2 gepusht | ok |
| 3↔4,5,6 | Catalogus/lees_catalogus/controleer | ok; T4 zonder spellingstalen.toml → controleer() mag geen fout geven op ontbrekende spellingstalen.toml |
| 5↔3 | regel min. 5 behalve en-eff | ok |
| 6↔3 | data als build- en gewone dependency | ok |
| 10→7,8 | sterkte vóór generator | Ruling: volgorde 1-6,10,7,8,9,11 — plan zegt het zelf — kost niets |
| 7↔8,11 | Gegenereerd/Modus | ok |
| 9↔8 | wachtzin(woorden, scheidingsteken) | ok |
| 12↔7,8 | Serialize op opties | ok (feature serde in kern) |
| 13/14/16/17 | PwApp, interfacesleutels | ok; sleutels in T4 alvast aanmaken voor de bekende lijsten |
| 15↔17 | Klembordbeheer | ok |
| per taak | tests vs code | T2 kopbloktest scant ook plan/spec (markdown met Met-kolom) → die voldoen |

Ruling: werken op branch pwspeller-2 i.p.v. master; aan het eind merge na akkoord Hans — skill verbiedt master zonder toestemming — kost een merge.
Ruling: tag v1.0-vb + GitHub-release in T1 uitvoeren — expliciet in goedgekeurde spec en plan — omkeerbaar (tag/release verwijderen).
Ruling: T4 controleer(): ontbrekende spellingstalen.toml is pas een fout zodra het bestand bestaat? Nee: de echte-data-test van T4 draait zonder combinaties; controleer() eist geen spellingstalen.toml, de test in T5 eist zes combinaties — kost niets.

## Taken
BASE taak 1-2: 516044c
Task 1-2: fix round 1/5 (4 addressed, 0 open — blokvolgorde 4 md-bestanden, .gitignore-blok, .DS_Store uit git; commits 4e54910..cc7516a)
Ruling: re-review van fixronde 1 door controller zelf gedaan (head/tail/diff --stat) i.p.v. subagent — Hans vroeg zuinig met tokens, fix was mechanisch en volledig zichtbaar — kost: een gemiste subtiliteit in 22 regels opmaak.
Task 1-2: complete (commits 516044c..cc7516a, review clean)
Fase A: klaar — stoppunt, wacht op akkoord Hans voor fase B.
BASE taak 3: cc7516a
Task 3: fix round 1/5 (8 addressed, 0 open — exacte sleutels, verweesde blokkeerlijst, lege waarden, en.toml verplicht, lege lijst/hoofdletterongevoelig, tests; commits be2283d..0713ec4)
Task 3: complete (commits cc7516a..0713ec4, review clean)
BASE taak 4: 0713ec4
Task 4: fix round 1/5 (1 important + 8 minor addressed, 0 open; commits 0e611ae..74f33f7)
Ruling: re-review fixronde taak 4 door controller met grep — tekstuele datafixes, zuinig met tokens — kost: gemiste tikfout.
Task 4: complete (commits 0713ec4..74f33f7, review clean)
BASE taak 5: 74f33f7
Task 5: geïmplementeerd (commit 6bee37f) — REVIEW NOG NIET GEDAAN. Zorg implementer: drempeltests (nl ≥ 7500, fr/de ≥ 5500) beperkten blokkeren; milde woorden teruggezet; fr 5511.
Overdracht naar de andere Mac op 2026-10-16 op verzoek van Hans. Hervatten: Taak 5 reviewen, dan Taak 10 (vóór 7), 6, 7, 8, 9, 11.
Briefs opnieuw maken: awk -v n=N '$0 ~ "^### Taak "n":" {p=1; print; next} p && /^(### Taak |## Fase |---$|\*\*Stoppunt)/ {exit} p' PLAN > taak-N-brief.md (scripts van de skill zoeken naar "Task N" en werken niet).

Hervat op Mac mini M4 (2026-10-17); branch gelijk aan master 38e7dcf.
Task 5: review — spec ✅; Important: idioot, zwijn (nl), stupide (fr), satan (de) niet geblokkeerd.
Task 5: minor (deferred → voorleggen aan Hans bij goedkeuring blokkeerlijsten): lijst van beladen woorden (doodmoe, varken, enfer, cochon, schrecklich, luftwaffe, …), zie reviewrapport.
Task 5: fix round 1/5 (4 addressed, 0 open; commits 89d229b..0f9c9c1) — re-review door controller (git show, 8 regels)
Task 5: complete (commits 74f33f7..0f9c9c1, review clean)
BASE taak 6: 0f9c9c1
Task 6: review — spec ✅, Important: Box::leak in interfacetekst.
Ruling: interfacetekst krijgt signatuur `fn interfacetekst<'a>(taal: &str, sleutel: &'a str) -> &'a str` i.p.v. `-> &'static str` uit het plan — spec legt de signatuur niet vast, geen lek — kost: latere taken die &'static verwachten moeten een levensduur volgen.
Task 6: minor (deferred): `pub use pwspeller_data::*` breed; dedup() zonder sort/commentaar in build.rs; onbekende extensies in data/ stil genegeerd.
Task 6: fix round 1/5 (1 addressed, 0 open; commits 217c50e..3b6265b) — re-review door controller (git show)
Task 6: complete (commits 0f9c9c1..3b6265b, review clean)
BASE taak 10: 3b6265b
Ruling: review taak 10 door controller (35 regels pure functies + 11 tests gelezen) i.p.v. subagent — Hans vroeg zuinig met tokens, diff klein en volledig gelezen — kost: gemiste subtiliteit in formules.
Task 10: complete (commits 3b6265b..7d12df3, review clean). Concern implementer: log2(10·n) voor cijfer is correct (10 cijfers × n posities).
Task 10: minor (deferred): cargo fmt --check geeft 23 afwijkingen (o.a. build.rs) — Ruling: implementer taak 7 draait cargo fmt in een aparte commit — CI (taak 18) eist fmt.
BASE taak 7: 7d12df3
Task 7: geïmplementeerd (b848d5f fmt, 751bd88); review loopt
Task 7: review — spec ✅, 3 Important: minima onbegrensd (>128 mogelijk), Vec<char> onbeschermd, geen OS-RNG-helper.
Ruling: fixronde neemt ook minor 4 (rand feature unbiased), 5 (doc-comment entropiebenadering) en 6 (positietest shuffle) mee — goedkoop, zelfde bestand — kost een paar regels.
Ruling: publieke API wordt genereer_wachtwoord_os(o) met rand::rng(); generieke variant blijft pub maar #[doc(hidden)] voor tests van andere crates — afwijking van planinterface — kost: app (taak 14) moet _os-variant gebruiken.
Task 7: minor (deferred → taak 8): woorden: Option<Vec<String>> onbeschermd — Zeroizing per woord.
FIX_BASE taak 7: 751bd88
Task 7: fix round 1/5 (6 addressed, 0 open; re-review sonnet) ; fix round 2/5 (slotblok + re-export, gecontroleerd door controller met tail/grep; commit cba71e7)
Task 7: minor (deferred): .expect("alleen ASCII") zou bij panic Vec ongewist droppen — theoretisch.
Task 7: complete (commits b848d5f..cba71e7, review clean)
BASE taak 8: cba71e7
Ruling: Gegenereerd.woorden wordt Option<Zeroizing<Vec<String>>> (Zeroize wist elke String) — deref naar &[String] blijft werken voor taak 9 — kost: wijziging aan type uit taak 7.
Task 8: review spec ✅ Approved; minors (deferred): capaciteit bron.len()+1 bij niet-ASCII; '-'-filter alleen in data; test cijfer+hoofdletter samen; app moet _os-varianten gebruiken (taak 14).
Task 8: complete (commits cba71e7..a5ad918, review clean)
BASE taak 9: a5ad918
Task 9: review spec ✅ Approved; minors (deferred): geen tests fr/de/navo-combinaties; geen tests leeg woord / alleen cijfer / tafel77; leeg woord geeft dubbel scheidingsteken; tab wordt "teken <tab> (U+0009)" met ruwe tab (UI/taak 14).
Task 9: complete (commits a5ad918..e312405, review clean)
BASE taak 11: e312405
Task 11: review (opus) spec ✅ Approved; Important: Instant loopt niet door in slaapstand (macOS/Linux) → geschiedenis overleeft dichtklappen.
Ruling: oplossen in de kern i.p.v. GUI: naast Instant ook SystemTime (wandklok) bijhouden; wissen als een van beide >= time_out (klok achteruit → negeren) — platformonafhankelijk, geen slaap-events nodig — kost: een verzette systeemklok kan vroeg wissen (veilige kant).
Ruling: minor 2 meenemen: ontsleutel() controleert eerst inactiviteit — robuust tegen vergeten aanroep in GUI — kost niets.
Task 11: minor (deferred): scheidingsteken geen Zeroizing; Some("") bij geschonden invariant; munlock zonder telling; sleutelkopie op stack (binnen §9-grens).
FIX_BASE taak 11: 031de09
Task 11: fix round 1/5 (2 addressed, 0 open; re-review door controller: functie gelezen) 
Task 11: complete (commits e312405..c62ccf2, review clean)
Fase B: klaar — stoppunt, wacht op akkoord Hans voor fase C.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
