FROM sean0x42/markdown-extract:fix-build-and-stderr as source

FROM alpine:3.10

COPY --from=source /markdown-extract /markdown-extract
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
