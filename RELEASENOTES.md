<p align="center"><img src="docs/assets/petisys-logo.jpeg" alt="Petisys" height="113"></p>

## Over dit bestand

- **Doel:** vertelt de eindgebruiker en de helpdesk in gewone taal wat er per release verandert.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Taak 18: downloads per platform en de eerste-start-waarschuwing. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Fix 2: eigen tekst doorlopend, optie "Cijfer toevoegen", korter splashscreen. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Stand na fase C bijgewerkt. |
| 2026-10-17 | Hans van der Pluijm (Petisys) | Claude Code | Stand na fase B bijgewerkt. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Regel over wachtzinnen in het Nederlands, Frans en Duits toegevoegd. |
| 2026-10-16 | Hans van der Pluijm (Petisys) | Claude Code | Releasenotes aangemaakt. |

---

# Releasenotes

## Nog niet uitgebracht

### Wat er verandert

- PWSpeller wordt opnieuw gebouwd voor Windows (x64 en ARM64), macOS (Apple en Intel, als app met het Petisys-icoon) en Linux; de vorige versie blijft beschikbaar als release v1.0-vb.
- Een wachtzin staat in beeld in hele woorden, met het scheidingsteken ertussen en een klein nummer boven elk woord; een woord wordt niet meer in tweeën gebroken.
- Het startscherm blijft anderhalve seconde staan. Eigen tekst staat in beeld als één doorlopende regel in kleur; alleen een gegenereerd wachtwoord staat in groepjes van vier. De spelling is doorlopend genummerd.
- Wachtzinnen maak je straks ook in het Nederlands, Frans en Duits, naast het Engels; woorden die aanstootgevend kunnen zijn komen er niet in voor.

### Nog te doen

- De eerste release voor Windows, macOS en Linux moet nog gepubliceerd worden; die komt met de tag `v2.0.0`.
- De programma's zijn nog niet ondertekend met een betaald certificaat; zie "Bekende problemen".

### Bekende problemen

- Bij de eerste start waarschuwt Windows (SmartScreen) en macOS (Gatekeeper) dat het programma niet ondertekend is. Windows: kies *Meer informatie* en *Toch uitvoeren*. macOS: rechtsklik op PWSpeller, kies *Open* en bevestig.
- Wordt het programma geforceerd beëindigd (taakbeheer, stroom eraf), dan blijft een gekopieerd wachtwoord op het klembord staan.
- Op Linux met X11 kunnen klembordbeheerders een gekopieerd wachtwoord bewaren.

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
