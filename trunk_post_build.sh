#!/bin/sh
LANG_CODE=$(cat .lang_code 2>/dev/null || echo en)
INPUT="$TRUNK_STAGING_DIR/index.html"
TMP="$INPUT.tmp"
awk -v lang="$LANG_CODE" '{
    sub(/<html /, "<html lang=\"" lang "\" ")
    sub(/<meta property="og:locale">/, "<meta property=\"og:locale\" content=\"" lang "\">")
    print
}' "$INPUT" > "$TMP" && mv "$TMP" "$INPUT"
