#!/bin/sh

cargo release $@ --execute
cargo build --release
./target/release/release
git add README.md && git commit -m "docs(readme.md) update version"
