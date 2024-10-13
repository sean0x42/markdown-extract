#!/bin/sh

set -e

EOF=$(dd if=/dev/urandom bs=15 count=1 status=none | base64)

# process flags
ARGUMENTS=""
if [ "$FLAG_CASE_SENSITIVE" = "true" ]; then
    ARGUMENTS="${ARGUMENTS} --case-sensitive"
fi
if [ "$FLAG_INCLUDE_ALL" = "true" ]; then
    ARGUMENTS="${ARGUMENTS} --all"
fi
if [ "$FLAG_NO_PRINT_MATCHED_HEADING" = "true" ]; then
    ARGUMENTS="${ARGUMENTS} --no-print-matched-heading"
fi
ARGUMENTS="$ARGUMENTS $*"

/markdown-extract $ARGUMENTS 1>stdout
{ echo "markdown<<$EOF"; cat stdout; echo "$EOF"; } >> "$GITHUB_OUTPUT"
