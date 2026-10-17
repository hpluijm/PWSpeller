#!/usr/bin/env bash
# ── Over dit bestand ──────────────────────────────────────
# Doel:           Haalt de sectie van één versie uit RELEASENOTES.md voor de GitHub-release; faalt als die ontbreekt.
# Klant:          Petisys
# Gemaakt vanuit: Petisys
# Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
# Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
#
# 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
#   Taak 18 fixronde: script aangemaakt (exacte kopmatch, geen stille terugval).
# ──────────────────────────────────────────────────────────
#
# Gebruik: releasenotes.sh <tag> <RELEASENOTES.md>
#   Zoekt een kop die precies "## v2.0.0" of "## 2.0.0" is, of daarmee begint gevolgd door een
#   spatie (bv. "## v2.0.0 (17 oktober 2026)"), en drukt de tekst tot de volgende "## "-kop af.
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
sectie="$(awk -v a="## v$kaal" -v b="## $kaal" '
    function kop_past(r, k) { return r == k || substr(r, 1, length(k) + 1) == k " " }
    /^## / {
        if (binnen) exit
        if (kop_past($0, a) || kop_past($0, b)) { binnen = 1; gevonden = 1 }
        next
    }
    binnen { print }
    END { if (!gevonden) exit 3 }
' "$bestand")" || {
    echo "::error::Geen sectie '## $tag' of '## $kaal' in $bestand; hernoem 'Nog niet uitgebracht' naar de versie vóór het taggen." >&2
    exit 1
}

if [ -z "$(printf '%s' "$sectie" | tr -d '[:space:]')" ]; then
    echo "::error::De sectie voor $tag in $bestand is leeg." >&2
    exit 1
fi
printf '%s\n' "$sectie"

# © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
# Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
