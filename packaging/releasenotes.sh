#!/usr/bin/env bash
# ── Over dit bestand ──────────────────────────────────────
# Doel:           Haalt de sectie van één versie uit RELEASENOTES.md voor de GitHub-release; faalt als die ontbreekt.
# Klant:          Petisys
# Gemaakt vanuit: Petisys
# Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
# Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
#
# 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
#   Eindreview: bij een prerelease-tag terugvallen op de sectie van de versie zonder achtervoegsel.
#
# 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
#   Taak 18 fixronde: script aangemaakt (exacte kopmatch, geen stille terugval).
# ──────────────────────────────────────────────────────────
#
# Gebruik: releasenotes.sh <tag> <RELEASENOTES.md>
#   Zoekt een kop die precies "## v2.0.0" of "## 2.0.0" is, of daarmee begint gevolgd door een
#   spatie (bv. "## v2.0.0 (17 oktober 2026)"), en drukt de tekst tot de volgende "## "-kop af.
#   Bij een prerelease-tag (v2.0.0-rc1) wordt eerst "## v2.0.0-rc1" gezocht en daarna "## v2.0.0".
#   Ontbreekt de kop of is de sectie leeg, dan stopt het script met een fout.
set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "Gebruik: $0 <tag> <RELEASENOTES.md>" >&2
    exit 2
fi
tag="$1"
bestand="$2"
kaal="${tag#v}"

# Vergelijken als tekst, niet als reguliere expressie: de punten in een versie zijn letterlijk.
# Geeft de sectie bij kop a of b; afsluitcode 3 als geen van beide bestaat.
haal_sectie() {
    awk -v a="## v$1" -v b="## $1" '
        function kop_past(r, k) { return r == k || substr(r, 1, length(k) + 1) == k " " }
        /^## / {
            if (binnen) exit
            if (kop_past($0, a) || kop_past($0, b)) { binnen = 1; gevonden = 1 }
            next
        }
        binnen { print }
        END { if (!gevonden) exit 3 }
    ' "$bestand"
}

# Eerst de exacte versie (bv. 2.0.0-rc1), bij een prerelease daarna de versie zonder achtervoegsel.
if sectie="$(haal_sectie "$kaal")"; then
    gebruikt="$kaal"
elif [ "$kaal" != "${kaal%%-*}" ] && sectie="$(haal_sectie "${kaal%%-*}")"; then
    gebruikt="${kaal%%-*}"
else
    extra=""
    [ "$kaal" != "${kaal%%-*}" ] && extra=" en ook niet '## v${kaal%%-*}'"
    echo "::error::Geen sectie '## $tag' of '## $kaal'$extra in $bestand; hernoem 'Nog niet uitgebracht' naar de versie vóór het taggen." >&2
    exit 1
fi

if [ -z "$(printf '%s' "$sectie" | tr -d '[:space:]')" ]; then
    echo "::error::De sectie voor $gebruikt in $bestand is leeg." >&2
    exit 1
fi
printf '%s\n' "$sectie"

# © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
# Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
