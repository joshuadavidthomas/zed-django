set dotenv-load := true
set unstable := true

# List all available commands
[private]
default:
    @just --list --list-submodules

bumpver *ARGS:
    uvx bumpver {{ ARGS }}

check *ARGS:
    cargo check {{ ARGS }}

clean:
    cargo clean

clippy *ARGS:
    cargo clippy --all-targets --all-features --fix {{ ARGS }} -- -D warnings

fmt *ARGS:
    cargo +nightly fmt {{ ARGS }}

lint:
    @just --fmt
    @just clippy
    @just fmt
    @just pre-commit
    @just check

pre-commit:
    uv run --with pre-commit-uv pre-commit run --all-files
