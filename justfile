export PATH := "./node_modules/.bin:" + env_var('PATH')

cargo-dev: 
    cargo run -p dev-server

# Run vite
vite *ARGS:
    cd client && vite {{ARGS}}

# Run npm
npm *ARGS:
    cd client && npm {{ARGS}}

lint: client-lint cargo-lint

# Lint Client 
client-lint: 
    cd client && eslint . --ext ts,tsx \
    --report-unused-disable-directives \
    --max-warnings 0

# Lint rust project
cargo-lint:
    cargo checkmate

# Run a suite of cargo commands
cargo-verbose:
    cargo check --workspace --verbose && \
    cargo build --workspace --verbose && \
    cargo doc --workspace --verbose && \
    cargo test --workspace --verbose && \
    cargo clippy --workspace --verbose
