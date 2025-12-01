test:
    cargo test --workspace

build-rust:
    cargo build --workspace

build: build-rust

run-rust *arguments:
    cargo run -- {{arguments}}

run *arguments:
    just run-rust {{arguments}}

check:
    cargo check --workspace

watchexec *arguments:
    watchexec -r -e rs,jinja -- bash -c 'clear && just test && clear && just run-rust {{arguments}}'
