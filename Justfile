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
    cargo clippy --all-targets --all-features --benches --fix {{ ARGS }} -- -D warnings

fmt *ARGS:
    cargo +nightly fmt {{ ARGS }}

# run pre-commit on all files
lint:
    @just --fmt
    @just fmt
    uv run --with pre-commit-uv pre-commit run --all-files
