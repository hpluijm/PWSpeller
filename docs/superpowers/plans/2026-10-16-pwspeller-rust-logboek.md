## Over dit bestand

- **Doel:** het uitvoeringslogboek van het implementatieplan: welke taken af zijn, met welke commits en welke beslissingen, zodat het werk op een andere Mac verder kan.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, branch pwspeller-2 (nog geen release)
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Logboek bijgewerkt tot en met fase C (taak 12–17). |
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
Hans: blokkeerlijsten goedgekeurd; akkoord fase C (2026-10-17).
BASE taak 12: 303214a
Task 12: review spec ✅ Approved; Important: versienummer veroudert (rerun-if-changed alleen .git/HEAD).
Ruling: fixronde neemt ook mee: Windows-foutpad zichtbaar (fallback-logbestand in temp_dir; afspraak 3) en kapot instellingenbestand eerst hernoemen naar .kapot — goedkoop, voorkomt stil falen — kost een paar regels.
Task 12: minor (deferred): log zonder rotatie; UTC-tijden in log; hover-contrast; set_boxed_logger tweede aanroep; geen test VERSIE; font mist ✓ ↑ ▲ □ (pictogrammen als vorm tekenen in taak 14/15).
FIX_BASE taak 12: fd74d65
Task 12: fix round 1/5 (3 addressed, 0 open; re-review door controller: diff gelezen)
Task 12: complete (commits 303214a..2e4c5df, review clean)
BASE taak 13: 2e4c5df
Task 13: review spec ✅, 3 Important: logo-asset 2639px elke start verkleind; volgordetest bewijst niets; faalpad logo niet getest.
Ruling: fixronde neemt minors mee: merkgrijs i.p.v. Color32::GRAY, logo-textuur vrijgeven na splash, instellingen niet twee keer laden — goedkoop.
Task 13: minor (deferred): KLANT hardcoded; versie zonder label; splash niet gemeten op 480x560.
FIX_BASE taak 13: 8e0e5cb
Task 13: fix round 1/5 (6 addressed; re-review controller: test gelezen — titel 'PWSpeller' ook op splash, dus volgorde vooral via Fase bewezen)
Ruling: taak 14 breidt splash_komt_voor_hoofdvenster uit: knop 'knop.genereer' ontbreekt in frame 1 en is er na SPLASHDUUR — dan bewijst de test de volgorde echt — kost niets.
Task 13: complete (commits 2e4c5df..ab0f829, review clean)
BASE taak 14: ab0f829
Task 14: review spec ❌ (eigen tekst zonder groepjes), 4 Important: eigen_tekst-commentaar onjuist / geen zeroize; groepjes ontbreken bij eigen tekst; spelregels elke frame opnieuw (prestatie + kopieën); Enter overschrijft gefocuste knop.
Ruling: fixronde neemt minors mee: unwrap_or("-") weg, woordenlijsten sorteren, verhoog_lengte één keer per frame.
Task 14: minor (deferred): teken_midden/genereer te groot; testgaten (woordenlijstkeuze, taalwissel wachtzin, vet/kleur); 480 px niet gemeten (fase-einde visueel nameten).
FIX_BASE taak 14: a2da753
Task 14: fix round 1/5 (5 addressed, 0 open; re-review sonnet)
Task 14: complete (commits ab0f829..f08baf4, review clean)
BASE taak 15: f08baf4
Task 15: review spec ✅ Approved; minor 1+2 direct verwerkt (a7c3332, controller: diff gelezen)
Task 15: minor (deferred): geen test opnieuw kopiëren binnen termijn; geen test app zonder bord; klembord_mislukt blijft staan; Windows/Linux-takken niet gecompileerd (CI taak 18).
Task 15: complete (commits f08baf4..a7c3332, review clean)
BASE taak 16: a7c3332
Task 16: review spec ✅, 2 Important: midden past niet op 480x560 (geen scroll); gemarkeerde groep buiten beeld bij lange wachtwoorden.
Ruling: fixronde neemt mee: zuiver zwart (#000) op goud (5,2:1 i.p.v. 4,6:1), dode code in test.
Task 16: deferred naar taak 17: time-out/wis van geschiedenis moet ook in voorleesmodus werken (voorlezen = None) + test.
Task 16: minor (deferred): per-frame kopieën in egui-weergave; geen Drop-test.
FIX_BASE taak 16: 01128be
Task 16: fix round 1/5 (3 addressed; re-review controller: commits + 176 tests groen gecontroleerd)
Task 16: complete (commits a7c3332..3f0d2eb, review clean)
BASE taak 17: 3f0d2eb
Task 17: review spec ✅, 4 Important: wis bij geminimaliseerd venster onzeker; on_exit niet bij kill/geforceerd afmelden (spec te sterk); huidig resultaat overleeft inactiviteitswis; test geen-wachtwoord-op-schijf zwak.
Ruling: bij inactiviteitswis en bij_afsluiten ook het huidige resultaat (hoofd.wachtwoord/wachtzin/cache, eigen tekst) wissen — spec §9 bedoelt dat na inactiviteit niets leesbaar blijft — kost: medewerker ziet na 15 min een leeg scherm.
Ruling: elke invoer (toets/muis) telt als gebruik (markeer_gebruik in kern) — "inactiviteit" = niemand bedient de app — kost: geschiedenis blijft langer als iemand bezig is.
Ruling: spec §9 wordt verzacht: on_exit dekt venster sluiten en normaal afsluiten; bij kill/geforceerd afmelden verdwijnt het geheugen met het proces, het klembord niet — eerlijk i.p.v. te sterke belofte — melden aan Hans.
Ruling: geminimaliseerd venster: grens vastleggen in implementatie.md + handmatige meting bij fase-einde — meten kan niet in CI.
FIX_BASE taak 17: 02d62f5
Task 17: fix round 1/5 (6 addressed, 0 open; re-review sonnet)
Task 17: minor (deferred): resultaat in beeld na handmatige 'Wis geschiedenis' valt niet onder inactiviteitswis (lege geschiedenis → controle stopt).
Task 17: complete (commits 3f0d2eb..7ab7da9, review clean)
Fase C: klaar — stoppunt, wacht op akkoord Hans voor fase D. Visuele controle niet mogelijk (screencapture geweigerd) → bij Hans.
Open vraag aan Hans (2026-10-17): schermafbeeldingen — optie 1 VS Code rechten (Schermopname + Toegankelijkheid), optie 2 egui_kittest-snapshots (wgpu, alleen in tests) aan begin fase D. Hans start VS Code opnieuw (vermoedelijk voor optie 1). Fase D wacht op akkoord.
Hervatten: lees dit logboek; volgende = fase D (taak 18, 19); briefs maken met awk (zie boven).
Visuele controle (screencapture -l, 2026-10-17): voorleesmodus toont GEEN teken en GEEN omschrijving — middenzone ingeklapt (ScrollArea uit fixronde taak 16), in 560x668 en 480x560. Tests misten het.
Ruling: taak 16 fixronde 2 met verse implementer + test op werkelijke hoogte/zichtbaarheid van het tekenlabel — echte fout in kernfunctie voor de helpdesk.
FIX_BASE taak 16 r2: f2fa7f3
Task 16: fix round 2/5 (voorleesmodus-midden zichtbaar; controller zag schermafbeelding 480x560: OK) — commit 0ba3337
Task 16: complete (opnieuw), review clean na visuele controle.
Hans meldt (visueel): wachtzin wordt in groepjes van 4 opgeknipt, ook midden in woorden. Bevestigd in hoofd.rs:190 (groepjes_weergave voor alle tabs).
Ruling: wachtzin toont hele woorden zonder groepjes (positienummers per woord); groepjes blijven voor wachtwoord en eigen tekst tot Hans beslist — spec §8 bedoelde groepjes voor tekens, niet voor woorden.
FIX_BASE groepjes: 0ba3337
Fix groepjes: wachtzin in hele woorden (5b972bf); controller zag schermafbeelding: OK. Open: label 'Cijfer aan elk woord' misleidend (één cijfer); nummering spelling per woord springt (8→15, telt tekens); Hans beslist over groepjes bij wachtwoord.
Fix 2 (31666c9) + spatie als markering (f48e79f): groepjes alleen gegenereerd wachtwoord, eigen tekst doorlopend, wachtzin hele woorden, splash 1,5 s (afspraak 24 globaal aangepast in mac-setup), 'Cijfer toevoegen', spelling doorlopend genummerd. Controller zag schermafbeeldingen: OK.
Fase C: opnieuw afgerond na visuele controle — stoppunt, wacht op akkoord Hans voor fase D.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
