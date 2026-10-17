<p align="center"><img src="docs/assets/petisys-logo.jpeg" alt="Petisys" height="113"></p>

## Over dit bestand

- **Doel:** vertelt de eindgebruiker en de helpdesk in gewone taal wat er per release verandert.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 19: sectie hernoemd naar v2.0.0 met datum, inhoud herschreven voor gebruiker en helpdesk. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 18: downloads per platform en de eerste-start-waarschuwing. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fix 2: eigen tekst doorlopend, optie "Cijfer toevoegen", korter splashscreen. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Stand na fase C bijgewerkt. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Stand na fase B bijgewerkt. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Regel over wachtzinnen in het Nederlands, Frans en Duits toegevoegd. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Releasenotes aangemaakt. |

---

# Releasenotes

## Nog niet uitgebracht

## v2.0.0 — 2026-10-17

### Wat er verandert

- PWSpeller is opnieuw gebouwd voor Windows (x64 en ARM64), macOS (Apple en Intel) en Linux. De vorige versie blijft beschikbaar als release v1.0-vb.
- Je maakt een wachtwoord, een wachtzin of spelt je eigen tekst, en de spelling staat erbij in het Nederlands, Vlaams, Frans, Duits, Engels of het NAVO-alfabet. Wachtzinnen maak je in het Nederlands, Frans, Duits of Engels; woorden die aanstootgevend kunnen zijn komen er niet in voor.
- Een gegenereerd wachtwoord staat in groepjes van vier met nummers. Een wachtzin staat in hele woorden met het scheidingsteken ertussen. Eigen tekst staat doorlopend in beeld.
- Met *Lees voor* zie je één teken of woord tegelijk, groot; volgende en vorige met spatie en de pijltjes, stoppen met Esc.
- De geschiedenis staat alleen in het geheugen, versleuteld, en wordt gewist bij het sluiten, na 15 minuten inactiviteit (instelbaar) of met *Wis geschiedenis*. Een gekopieerd wachtwoord wordt na 30 seconden van het klembord gehaald (instelbaar; op macOS standaard uit).
- Het programma gebruikt geen internet.

### Nog te doen

- De programma's zijn nog niet ondertekend met een betaald certificaat; zie "Bekende problemen".

### Bekende problemen

- Bij de eerste start waarschuwt Windows (SmartScreen) en macOS (Gatekeeper) dat het programma niet ondertekend is. Windows: kies *Meer informatie* en *Toch uitvoeren*. macOS: open *Systeeminstellingen > Privacy en beveiliging* en kies *Toch openen* (op oudere macOS-versies kan ook rechtsklik en *Open*).
- Wordt het programma geforceerd beëindigd (taakbeheer, stroom eraf), dan blijft een gekopieerd wachtwoord op het klembord staan.
- Op Linux met X11 kunnen klembordbeheerders een gekopieerd wachtwoord bewaren.
- Een tekst die je zelf typt, kan het programma niet volledig uit het geheugen wissen; dat kan alleen door het programma te sluiten.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
