FROM rust:1-buster AS chef
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef

WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo install --path .

# We do not need the Rust toolchain to run the binary!
FROM debian:buster

# https://techoverflow.net/2021/01/13/how-to-use-apt-install-correctly-in-your-dockerfile/
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin

EXPOSE 8080
ENV RUST_LOG=info
CMD [ "crosswords-be" ]