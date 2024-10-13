# Dockerfile for creating statically-linked Rust applications.

# Step 0: Initialise builder
FROM rust:1.81 as builder
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update
RUN apt-get install -y musl-tools gcc-x86-64-linux-gnu
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'

# Step 1: Build and install binary
WORKDIR /usr/src/markdown-extract
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo install \
  --target x86_64-unknown-linux-musl \
  --path ./crates/markdown-extract-cli

# Step 2: Copy bin to an empty Docker image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/markdown-extract .
USER 1000
ENTRYPOINT ["./markdown-extract"]
