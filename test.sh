#!/bin/sh
cargo test -p larz --all-targets --all-features --future-incompat-report
cargo test --doc -p larz --all-features --future-incompat-report -- --show-output