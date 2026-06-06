#!/bin/sh
LANG_CODE=$(cat .lang_code 2>/dev/null || echo en)
TIKITKO=$(cat .tikitko 2>/dev/null || echo 0)
TITLE=$(cat .title 2>/dev/null || echo "")
DESCRIPTION=$(cat .description 2>/dev/null || echo "")
INPUT="$TRUNK_STAGING_DIR/index.html"
TMP="$INPUT.tmp"

json_escape() {
    printf '%s' "$1" | sed 's/\\/\\\\/g; s/"/\\"/g'
}
NAME=$(json_escape "$TITLE")
DESC=$(json_escape "$DESCRIPTION")

THEME_COLOR=$(sed -n 's/.*<meta name="theme-color" content="\([^"]*\)".*/\1/p' "$INPUT" | head -n1)
[ -n "$THEME_COLOR" ] || THEME_COLOR="#242424"

cat > "$TRUNK_STAGING_DIR/manifest.json" <<EOF
{
    "name": "$NAME",
    "short_name": "$NAME",
    "lang": "$LANG_CODE",
    "icons": [
        {
          "purpose": "maskable",
          "sizes": "512x512",
          "src": "icon512_maskable.png",
          "type": "image/png"
        },
        {
          "purpose": "any",
          "sizes": "512x512",
          "src": "icon512_rounded.png",
          "type": "image/png"
        }
    ],
    "start_url": "/",
    "background_color": "$THEME_COLOR",
    "display": "standalone",
    "scope": "/",
    "theme_color": "$THEME_COLOR",
    "description": "$DESC"
}
EOF

BEACON='<script defer src='\''https://beacon.tikitko.dev/script.min.js'\'' data-cf-beacon='\''{"token": "2d2ff34379204b78885fab3983f265c3"}'\''></script>'

awk -v lang="$LANG_CODE" -v tikitko="$TIKITKO" -v beacon="$BEACON" '{
    sub(/<html /, "<html lang=\"" lang "\" ")
    sub(/<meta property="og:locale">/, "<meta property=\"og:locale\" content=\"" lang "\">")
    if (tikitko == "1" && $0 ~ /<\/body>/) {
        print "    " beacon
    }
    print
}' "$INPUT" > "$TMP" && mv "$TMP" "$INPUT"
