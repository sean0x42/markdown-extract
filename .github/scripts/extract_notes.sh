#!/bin/bash

if [[ -z $RELEASE_NAME ]]; then
   echo "Missing env var RELEASE_NAME"
   exit 1
fi

pattern="^$RELEASE_NAME"

notes=$(docker run -v $PWD:/opt -it sean0x42/markdown-extract:v2 \
            --no-print-matched-heading \
            $pattern \
            /opt/CHANGELOG.md)

notes=="${notes=//'%'/'%25'}"
notes=="${notes=//$'\n'/'%0A'}"
notes=="${notes=//$'\r'/'%0D'}"

echo "::set-output name=value::$notes"
