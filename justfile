set dotenv-load := false

# Build workspace
default: build

# Debug build
build:
    cargo build --workspace

# Release binary
build-release:
    cargo build --release -p cli

# Run tests
test:
    cargo test --workspace

# CLI tests only
test-cli:
    cargo test -p cli

# Install release binary
install: build-release
    install -Dm755 target/release/cosmic-wicara "$HOME/.local/bin/cosmic-wicara"
    @echo "installed: $HOME/.local/bin/cosmic-wicara"

# Check formatting
fmt:
    cargo fmt --check

# Lint warnings errors
clippy:
    cargo clippy --workspace -- -D warnings

# Clean artifacts
clean:
    cargo clean

# List recipes
list:
    @just --list
