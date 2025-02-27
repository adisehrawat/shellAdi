#!/bin/sh
set -e

cd "$(dirname "$0")"
cargo build --release --target-dir=/tmp/rust-build

# Open a new Git Bash window and run the program
mintty -e bash -c "/tmp/rust-build/release/shellAdi.exe; exec bash"
