build:
    cargo build

test:
    cargo test --all

# Enforces lints, tests, formatting
[group('check')]
check: clippy test
    nix flake check

# Fails on warnings, lints everything including tests
[group('check')]
clippy:
    cargo clippy --all-targets --all-features -- --deny warnings

# Update dependencies with 3 day cooldown
update:
    cargo update

# Run the Intel 8080 program disassembler
[group('run')]
@disassemble +FILES:
    cargo run --package disassembler -- {{ FILES }}
