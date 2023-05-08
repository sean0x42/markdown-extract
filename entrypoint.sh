#!/bin/sh

EOF=$(dd if=/dev/urandom bs=15 count=1 status=none | base64)
{ echo "markdown<<$EOF"; /markdown-extract "$@"; echo "$EOF"; } >> "$GITHUB_OUTPUT"
