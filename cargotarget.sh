#!/bin/bash
set -e

if [ "x$TARGETARCH" = "xamd64" ]; then
    echo "x86_64-unknown-linux-gnu"
elif [ "x$TARGETARCH" = "xarm64" ]; then
    echo "aarch64-unknown-linux-gnu"
else
    echo "TARGET_UNKNOWN"
fi
