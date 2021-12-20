RELEASE_BODY=`~/.cargo/bin/markdown-extract --no-print-heading "^$TAG_NAME" CHANGELOG.md`

RELEASE_BODY=="${RELEASE_BODY=//'%'/'%25'}"
RELEASE_BODY=="${RELEASE_BODY=//$'\n'/'%0A'}"
RELEASE_BODY=="${RELEASE_BODY=//$'\r'/'%0D'}"

echo "::set-output name=value::$RELEASE_BODY"
