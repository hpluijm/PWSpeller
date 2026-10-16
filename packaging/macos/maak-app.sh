#!/usr/bin/env bash
# ── Over dit bestand ──────────────────────────────────────
# Doel:           Bouwt PWSpeller.app uit een macOS-binary, met Info.plist, icoon en ad-hoc-ondertekening met sandbox.
# Klant:          Petisys
# Gemaakt vanuit: Petisys
# Bron:           https://github.com/hpluijm/PWSpeller, release v2.0.0
# Copyright: onderaan dit bestand. Overige rechthebbenden: NOTICE.md.
#
# 2026-10-17  Hans van der Pluijm (Petisys)  [Claude Code]
#   Taak 18: script aangemaakt.
# ──────────────────────────────────────────────────────────
#
# Gebruik: maak-app.sh <binary> <versie> [uitvoermap]
#   binary      het (universele) macOS-programma, bijvoorbeeld target/release/pwspeller
#   versie      bijvoorbeeld v2.0.0 of 2.0.0-3-gabc1234 (uit de git-tag)
#   uitvoermap  waar PWSpeller.app komt (standaard: de huidige map)
set -euo pipefail

if [ "$#" -lt 2 ] || [ "$#" -gt 3 ]; then
    echo "Gebruik: $0 <binary> <versie> [uitvoermap]" >&2
    exit 2
fi

binary="$1"
versie_ruw="$2"
uitvoer="${3:-.}"
map="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

[ -f "$binary" ] || { echo "Fout: binary niet gevonden: $binary" >&2; exit 1; }
for f in Info.plist PWSpeller.entitlements; do
    [ -f "$map/$f" ] || { echo "Fout: $map/$f ontbreekt" >&2; exit 1; }
done
icoon="$map/../../crates/app/assets/AppIcon.icns"
[ -f "$icoon" ] || { echo "Fout: icoon niet gevonden: $icoon" >&2; exit 1; }

# macOS wil in de plist alleen cijfers en punten (maximaal drie delen): v2.0.0-3-gabc -> 2.0.0.
versie="$(printf '%s' "$versie_ruw" | sed -E 's/^v//; s/^([0-9]+(\.[0-9]+){0,2}).*/\1/')"
case "$versie" in
    [0-9]*) ;;
    *) echo "Fout: versie '$versie_ruw' begint niet met een getal." >&2; exit 1 ;;
esac

app="$uitvoer/PWSpeller.app"
rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"

cp "$binary" "$app/Contents/MacOS/PWSpeller"
chmod 755 "$app/Contents/MacOS/PWSpeller"
cp "$icoon" "$app/Contents/Resources/AppIcon.icns"
sed "s/@VERSIE@/$versie/g" "$map/Info.plist" > "$app/Contents/Info.plist"
plutil -lint "$app/Contents/Info.plist" >/dev/null

# Ad-hoc ondertekening ("-"): zonder Apple-ontwikkelaarscertificaat, wel met sandbox en hardened runtime.
codesign -s - --force --options runtime \
    --entitlements "$map/PWSpeller.entitlements" "$app"
codesign --verify --strict "$app"

echo "Klaar: $app (versie $versie)"

# © 2026 Petisys (opdrachtgever) en Hans van der Pluijm, Petisys (uitvoerder).
# Gemaakt met Claude Code (Anthropic). Overige rechthebbenden: NOTICE.md.
