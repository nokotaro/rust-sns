# /bin/bash

cargo fix --allow-dirty --allow-staged

cargo fmt && cargo check && cargo test

DB_USER=rustsns \
DB_PASS=rustsns \
DB_HOST=34.146.23.50 \
DB_PORT=5432 \
cargo run
