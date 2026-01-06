test:
    cargo test --workspace

build:
    cargo build --workspace

run *arguments:
    cargo run -- {{arguments}}

check:
    cargo check --workspace

[no-cd]
scc exclude_part:
    scc -i rs --exclude-dir part_{{exclude_part}}

watchexec *arguments:
    watchexec -r -e rs -- bash -c 'clear && just test && clear && just run {{arguments}}'
