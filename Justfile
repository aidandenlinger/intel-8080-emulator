build:
    cargo build

check:
    cargo fmt --check
    # fail on warnings in CI, check everything including tests
    cargo clippy --all-targets -- -D warnings
    cargo test

test:
    cargo test

disassemble +FILES:
    cargo run -p disassembler {{FILES}}
