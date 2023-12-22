#!/bin/bash
set -e

if [ "x$TARGETARCH" = "xamd64" ]; then
    export RUST_TARGET=x86_64-unknown-linux-gnu
elif [ "x$TARGETARCH" = "xarm64" ]; then
    export RUST_TARGET=aarch64-unknown-linux-gnu
else
    echo "$TARGETARCH is unsupported !"
    exit 1
fi

export RUST_TARGET

if [ "x$TARGETARCH" != "x$BUILDPLATFORM" ]; then
    # cross compilation
    if [ "x$TARGETARCH" = "xamd64" ]; then
        export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc
        export CROSS_PKGS="binutils-x86-64-linux-gnu gcc-x86-64-linux-gnu"
    elif [ "x$TARGETARCH" = "xarm64" ]; then
        export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
        export CROSS_PKGS="binutils-aarch64-linux-gnu gcc-aarch64-linux-gnu"
    fi
fi
