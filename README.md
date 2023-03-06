# Rust microservice template

[TOC]

## Users

### curl

```bash
# health
curl -i http://localhost:3000/health

# hello world (only with helloworld feature)
curl -i http://localhost:3000/

# hello logged user (only with helloworld feature)
NAME=$(whoami)
curl -i http://localhost:3000/$NAME

# hello slow world (only with helloworld feature)
curl -i http://localhost:3000/slowworld
curl -i http://localhost:3000/slowworld?times=4 # any number equal or greeter to 0

# prometheus metrics (only with metrics feature)
curl -i http://localhost:3000/metrics

# 405 Method Not Allowed !
curl -i http://localhost:3000/something/else
```



## Developers

### Building and running

#### locally

```bash
# Debug
cargo build
cargo test
cargo run

# Release
cargo build --release
cargo test --release
cargo run --release

# Statically linked + Release + GNU/Linux x86_64
export RUSTFLAGS="-C target-feature=+crt-static"
cargo build --release --target x86_64-unknown-linux-gnu
cargo test --release --target x86_64-unknown-linux-gnu
cargo run --release --target x86_64-unknown-linux-gnu
unset RUSTFLAGS

# See alternative build in Features section
```

#### docker

```bash
# build is deprecated; use buildx: https://docs.docker.com/build/install-buildx/

# default build
docker buildx build -t rust/microservice .

# without any features
docker buildx build \
    --build-arg FEATURES="" \
    -t rust/microservice .

# run it
docker run -it --rm \
    -p 3000:3000 \
    rust/microservice

# control tracing with RUST_LOG
RUST_LOG="actix_server=warn,info"
docker run -it --rm \
    -p 3000:3000 \
    -e RUST_LOG="$RUST_LOG" \
    rust/microservice
unset RUST_LOG
```

### Logs

Use `RUST_LOG` to change the tracing behaviour

```bash
# eg: actix_server warning log and info for anything else
RUST_LOG="actix_server=warn,info" cargo run
```

### OpenTelemetry OTEL

Use built-in OpenTelemetry OTEL collector support by setting at least the environment variable `OTEL_SERVICE_NAME`.

```bash
# start jaeger
docker run -it --rm --name jaeger \
	-e COLLECTOR_OTLP_ENABLED=true \
    -p 16686:16686 \
    -p 4317:4317 \
    -p 4318:4318 \
    jaegertracing/all-in-one

# start your browser
open http://localhost:16686/

# start the microservice
OTEL_SERVICE_NAME="microservice" OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317" cargo run

```

### Features

This microservice exposes 2 features:

| name       | description                                                  |
| ---------- | ------------------------------------------------------------ |
| helloworld | Provides all helloworld endpoints (/, /helloworld, /slowworld, /{name}) |
| metrics    | Provides prometheus metrics with /metrics endpoint           |

```bash
# disable helloworld default feature (see Cargo.toml)
# will remove all endpoints except /health
cargo run --no-default-features

# use metrics and default features
cargo run --features="metrics"

# use metrics without helloworld
cargo run --no-default-features --features="metrics"
```

