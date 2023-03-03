FROM rust:1.67 as builder

ENV RUSTFLAGS="-C target-feature=+crt-static"

RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-gnu
RUN cargo test --release --target x86_64-unknown-linux-gnu
RUN mv /usr/src/target/x86_64-unknown-linux-gnu/release/actix-microservice ./microservice

# Production image
FROM gcr.io/distroless/static-debian11

COPY --from=builder /usr/src/microservice /

CMD ["/microservice"]
