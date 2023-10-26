#!/bin/bash

package_name := `sed -En 's/name[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
package_version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`

set dotenv-load

test TEST:
	cargo test {{TEST}}

tests:
	cargo test --all

bench:
    cargo bench

lint:
    cargo clippy --all-targets --all-features -- -D warnings

profile:
    cargo build
    perf record -g --call-graph dwarf ./target/debug/media_order
    hotspot perf.data

clean:
    cargo clean
    fd --type f .orig . --exec rm {} \;
    fd --type f .bk . --exec rm {} \;
    fd --type f .*~ . --exec rm {} \;
