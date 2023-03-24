# syntax=docker/dockerfile:1.4
FROM rust:1.67 as builder

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/
ENV RUSTFLAGS="-C target-feature=+crt-static"
ARG TARGETARCH
ARG FEATURES="helloworld,metrics"

COPY . .

RUN --mount=type=cache,id=rustcache,target=/usr/local/cargo/registry --mount=type=cache,id=rustcache,target=./target \
    cargo build --release --target $(./cargotarget.sh) --no-default-features --features="$FEATURES"
RUN --mount=type=cache,id=rustcache,target=/usr/local/cargo/registry --mount=type=cache,id=rustcache,target=./target \
    cargo test --release --target $(./cargotarget.sh) --no-default-features --features="$FEATURES"
RUN --mount=type=cache,id=rustcache,target=./target \
    mv /usr/src/target/$(./cargotarget.sh)/release/actix-microservice ./microservice

# Production image
FROM gcr.io/distroless/static-debian11

COPY --from=builder /usr/src/microservice /

CMD ["/microservice"]
