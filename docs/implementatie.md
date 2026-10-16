## Over dit bestand

- **Doel:** beschrijft hoe PWSpeller 2 in elkaar zit en welke alternatieven bewust zijn afgewezen.
- **Klant:** Petisys
- **Gemaakt vanuit:** Petisys
- **Bron:** https://github.com/hpluijm/PWSpeller, release v2.0.0
- **Copyright:** onderaan dit bestand. Overige rechthebbenden: NOTICE.md.

| Wanneer | Wie | Met | Wat |
|---|---|---|---|
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

## Afgewezen alternatieven

| Alternatief | Waarom niet |
|---|---|
| VB.NET houden op .NET 10 (WinForms) | Alleen Windows |
| C# met Avalonia | VB niet ondersteund, grote bestanden, native bibliotheken naast de exe |
| Go met Fyne | cgo en OpenGL, lastiger kruiselings bouwen |
| Eén HTML-bestand | Geen exe, geen app store-pakket |
| Slint | Extra licentievoorwaarde, eigen opmaaktaal |
| Iced | Nog vóór 1.0, API verandert per release |
| Crate `bitwarden-generators` gebruiken | GPL-3.0 zou PWSpeller GPL maken |
| Gebruikers eigen taalbestanden laten importeren | Verschillende versies per medewerker; sandbox |
| Dubbelzinnige Nederlandse woorden schrappen | Kost ruim 2.000 woorden; per letter spellen lost het op |
| Franse en Duitse woorden met accent of ß spellen of omzetten (é → e, ü → ue) | Foutgevoelig aan de telefoon; omzetten maakt niet-bestaande woorden |

---

© 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).<br>
Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
