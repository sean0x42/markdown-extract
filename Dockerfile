FROM rust:1.41

WORKDIR /usr/src/markdown-extract
COPY . .

RUN cargo install --path .

CMD ["markdown-extract"]
