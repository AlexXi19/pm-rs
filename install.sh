#!/bin/bash

cd "$(dirname "$0")"
cargo build
cp ./target/debug/pmrs /usr/local/bin
