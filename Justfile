build:
    cargo build

check:
    cargo fmt --check
    cargo clippy -- -Wclippy::pedantic
    cargo test

test:
    cargo test

disassemble +FILES:
    cargo run -p disassembler {{FILES}}
