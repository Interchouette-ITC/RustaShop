#!/bin/sh
set -e

# Rewrite <base href> in the built index.html at container start.
# Default: /. Override with BASE_HREF or RUSTASHOP_BASE_HREF (e.g. /shop/).
BASE_HREF=${RUSTASHOP_BASE_HREF:-${BASE_HREF:-/}}
case "$BASE_HREF" in
  /) ;;
  */) ;;
  *) BASE_HREF="${BASE_HREF}/" ;;
esac

INDEX_FILE=${INDEX_FILE:-/usr/share/nginx/html/index.html}
echo "Entrypoint: BASE_HREF=$BASE_HREF index=$INDEX_FILE"

if [ -f "$INDEX_FILE" ]; then
  if grep -q '<base href' "$INDEX_FILE"; then
    sed -i -E "s|<base href=\"[^\"]*\"[[:space:]]*/?>|<base href=\"$BASE_HREF\" />|g" "$INDEX_FILE"
  else
    sed -i "/<head>/a\\
    <base href=\"$BASE_HREF\" />" "$INDEX_FILE"
  fi
else
  echo "warning: missing $INDEX_FILE" >&2
fi

exec "$@"
