# syntax=docker/dockerfile:1.4
FROM rust:1.67 as builder

ENV RUSTFLAGS="-C target-feature=+crt-static"

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

ARG FEATURES="helloworld,metrics"

WORKDIR /usr/src/
COPY . .

RUN --mount=type=cache,id=rustcache,target=/usr/local/cargo/registry --mount=type=cache,id=rustcache,target=./target \
    cargo build --release --target x86_64-unknown-linux-gnu --no-default-features --features="$FEATURES"
RUN --mount=type=cache,id=rustcache,target=/usr/local/cargo/registry --mount=type=cache,id=rustcache,target=./target \
    cargo test --release --target x86_64-unknown-linux-gnu --no-default-features --features="$FEATURES"
RUN --mount=type=cache,id=rustcache,target=./target \
    mv /usr/src/target/x86_64-unknown-linux-gnu/release/actix-microservice ./microservice

# Production image
FROM gcr.io/distroless/static-debian11

COPY --from=builder /usr/src/microservice /

CMD ["/microservice"]
