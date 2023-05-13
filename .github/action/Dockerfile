FROM sean0x42/markdown-extract:v2 as source

FROM alpine:3.10

COPY --from=source /markdown-extract /markdown-extract
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
