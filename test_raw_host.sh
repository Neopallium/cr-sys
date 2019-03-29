#!/bin/sh

cargo clean && \
	cargo build --example raw_host && \
	cargo build --features guest --example raw_guest && \
	cargo run --example raw_host

