#####################
### Builder image ###
#####################
FROM rust:1-slim-buster as builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app

WORKDIR /usr/src/app

COPY Cargo.lock Cargo.toml ./
COPY crates/server/Cargo.toml ./crates/server/Cargo.toml
COPY crates/server/src ./crates/server/src

# Will build all dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo build --release

# Build (install) the actual binaries
RUN cargo install --path ./crates/server

####################
### Server image ###
####################
FROM rust:1-slim-buster

ENV INSTALL_PATH /usr/local/liquid_server

WORKDIR ${INSTALL_PATH}
COPY --from=builder /usr/local/cargo/bin/server ${INSTALL_PATH}

ENV RUST_LOG warn

EXPOSE ${PORT}

CMD ./server