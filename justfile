export PATH := "./node_modules/.bin:" + env_var('PATH')

dev: 
    cd ./crates/dev/ && cargo run

prod: build 
    cargo run -p prod-server

shuttle *ARGS: build
    cargo shuttle {{ARGS}}

build :
    cd client && vite build && \
    cd ../ && cargo build --all

build-release:
    cd client && vite build && \
    cd ../ && cargo build --all --release

# Run vite
vite *ARGS:
    cd client && vite {{ARGS}}

# Run npm
npm *ARGS:
    cd client && npm {{ARGS}}

# Run npx
npx *ARGS:
    cd client && npx {{ARGS}}

# Run npx
prettier *ARGS:
    cd client && npx prettier . --write

lint: client-lint cargo-lint

# Lint Client 
client-lint: 
    cd client && eslint . --ext ts,tsx \
    --report-unused-disable-directives \
    --max-warnings 0

# Lint rust project
cargo-lint:
    cargo checkmate

# Run clippy on everything
clippy:
    cargo clippy --workspace --all-targets
# Run check on everything
check:
    cargo check --workspace --all-targets

# Run a suite of cargo commands
cargo-verbose:
    cargo check --workspace --verbose && \
    cargo build --workspace --verbose && \
    cargo doc --workspace --verbose && \
    cargo test --workspace --verbose && \
    cargo clippy --workspace --verbose
