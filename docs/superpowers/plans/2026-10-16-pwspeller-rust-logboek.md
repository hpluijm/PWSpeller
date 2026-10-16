## Over dit bestand

- **Doel:** het uitvoeringslogboek van het implementatieplan: welke taken af zijn, met welke commits en welke beslissingen, zodat het werk op een andere Mac verder kan.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, branch pwspeller-2 (nog geen release)
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
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

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
