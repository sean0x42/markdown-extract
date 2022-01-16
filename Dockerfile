# Builder image
FROM rust:1.57 as builder
WORKDIR /usr/src/markdown-extract
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/markdown-extract /usr/local/bin/markdown-extract
ENTRYPOINT ["markdown-extract"]
