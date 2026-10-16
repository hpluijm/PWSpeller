## Over dit bestand

- **Doel:** legt uit waar de originele woordenlijsten vandaan komen en hoe `data/woordenlijsten/` eruit is afgeleid.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, branch pwspeller-2 (nog geen release)
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Originelen als bron vastgelegd. |

---

# Bronnen van de woordenlijsten

De drie bestanden hier zijn de originelen, met Claude gegenereerd in opdracht van Petisys
(8.000 woorden elk, minimaal 5 tekens). Ze worden niet in de app opgenomen; de app gebruikt
de afgeleide lijsten in `data/woordenlijsten/`.

| Origineel | Afgeleid | Bewerking |
|---|---|---|
| `nederlandse_woorden_8000_min5.txt` | `nl.txt` (7.527) | min `nl-geblokkeerd.txt` (473) |
| `mots_francais_8000_min5.txt` | `fr.txt` (5.511) | alleen a–z (5.743), min `fr-geblokkeerd.txt` (232) |
| `deutsche_woerter_8000_min5.txt` | `de.txt` (6.278) | kleine letters, alleen a–z (6.632), min `de-geblokkeerd.txt` (354) |

Woorden met een accent of ß zijn weggelaten omdat ze aan de telefoon foutgevoelig zijn (zie de
spec, §4). Wie dat later anders wil, begint opnieuw vanuit deze originelen.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
