# syntax=docker/dockerfile:1.4
FROM --platform=$BUILDPLATFORM rust:1.74 as builder

ARG BUILDPLATFORM
ARG TARGETARCH

WORKDIR /usr/src/

ADD cross-compilation.sh .

RUN <<EOF
. ./cross-compilation.sh
apt-get update && apt-get install -y \
    protobuf-compiler \
    $CROSS_PKGS
rm -rf /var/lib/apt/lists/*

rustup target add $RUST_TARGET
EOF

ENV RUSTFLAGS="-C target-feature=+crt-static"
ARG FEATURES="helloworld,metrics"

COPY . .

RUN --mount=type=cache,id=rustcache,target=/usr/local/cargo/registry --mount=type=cache,id=rustcache,target=./target <<EOF
. ./cross-compilation.sh
cargo build --release --target $RUST_TARGET --no-default-features --features="$FEATURES"
EOF

RUN --mount=type=cache,id=rustcache,target=/usr/local/cargo/registry --mount=type=cache,id=rustcache,target=./target <<EOF
. ./cross-compilation.sh
cargo test --release --target $RUST_TARGET --no-default-features --features="$FEATURES"
EOF

RUN --mount=type=cache,id=rustcache,target=./target <<EOF
. ./cross-compilation.sh
mv /usr/src/target/$RUST_TARGET/release/actix-microservice ./microservice
EOF

# Production image
FROM gcr.io/distroless/static-debian11

COPY --from=builder /usr/src/microservice /

CMD ["/microservice"]
