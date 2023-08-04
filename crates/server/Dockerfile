#####################
### Builder image ###
#####################
FROM rust:1-slim-bookworm as builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app

WORKDIR /usr/src/app

RUN apt-get update && apt-get -y install pkg-config libssl-dev

COPY Cargo.lock Cargo.toml ./
COPY crates ./crates

# Will build all dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release

# Build (install) the actual binaries
RUN cargo install --path ./crates/server

####################
### Server image ###
####################
FROM debian:bookworm-slim

ENV INSTALL_PATH /usr/local/tic-tac-5

RUN apt-get update && apt-get -y install ca-certificates

WORKDIR ${INSTALL_PATH}
COPY --from=builder /usr/local/cargo/bin/server ${INSTALL_PATH}

ENV RUST_LOG warn

EXPOSE ${PORT}

CMD ./server