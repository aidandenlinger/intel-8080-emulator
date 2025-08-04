# When running without a parameter, list recipes
[private]
@default:
    just --list --justfile {{justfile()}}

build:
    cargo build

test:
    cargo test --all

# Enforces lints, tests, formatting, and up-to-date packages
[group: 'check']
ci: clippy test
    cargo fmt --all -- --check
    cargo update --locked

# Fails on warnings, lints everything including tests
[group: 'check']
clippy:
    cargo clippy --all-targets --all-features -- --deny warnings

# Run the Intel 8080 program disassembler
[group: 'run']
@disassemble +FILES:
    cargo run --package disassembler -- {{FILES}}
