FROM rust:1.43

WORKDIR /usr/src/markdown-extract
COPY . .

RUN cargo install --path .

ENTRYPOINT ["markdown-extract"]
