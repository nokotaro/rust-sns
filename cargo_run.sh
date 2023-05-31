# /bin/bash

cargo fix --allow-dirty --allow-staged

cargo fmt && cargo check && cargo test

DB_USER=rustsns \
DB_PASS=rustsns \
DB_HOST=localhost \
DB_PORT=5432 \
cargo run
