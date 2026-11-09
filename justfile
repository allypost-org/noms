set dotenv-load := true
set positional-arguments := true

rustflags := "-C target-feature=+crt-static"
rust_target := "x86_64-unknown-linux-musl"

default:
    @just --list

build:
    RUSTFLAGS='{{rustflags}}' \
    cargo build \
    --release \
    --target '{{rust_target}}' \

test-all:
    RUSTFLAGS='{{rustflags}}' \
    cargo test \
    --target '{{rust_target}}' \
    --workspace \
    -- \
    --nocapture \

test package:
    RUSTFLAGS='{{rustflags}}' \
    cargo test \
    --target '{{rust_target}}' \
    --package '{{package}}' \
    -- \
    --nocapture \

test-watch package:
    RUSTFLAGS='{{rustflags}}' \
    cargo watch \
        --clear \
        --quiet \
        --watch './crates' \
        --watch './bins' \
        --exec 'test --target "{{rust_target}}" --package "{{package}}" -- --nocapture' \

dev-watch package *args:
    RUSTFLAGS='{{rustflags}}' \
    cargo watch \
        --clear \
        --quiet \
        --watch './crates' \
        --watch './bins' \
        --exec 'run --target "{{rust_target}}" --package "{{package}}" -- '"$(printf "%s" "{{args}}" | xargs printf "%q ")" \

dev-build package:
    RUSTFLAGS='{{rustflags}}' \
    cargo build \
        --target '{{rust_target}}' \
        --package '{{package}}' \

dev-watch-build package:
    RUSTFLAGS='{{rustflags}}' \
    cargo watch \
        --clear \
        --quiet \
        --watch './crates' \
        --watch './bins' \
        --exec 'build --target "{{rust_target}}" --package "{{package}}"' \

dev-run package *args:
    shift; \
    RUSTFLAGS='{{rustflags}}' \
    cargo run \
        --target '{{rust_target}}' \
        --package '{{package}}' \
        -- \
        "$@"

lint:
    cargo clippy --workspace --all-features -- \

lint-fix:
    cargo clippy --fix --allow-dirty --allow-staged --workspace --all-features -- \

fmt-dev: && fmt
    rustup run nightly cargo fmt --all \

fmt: lint-fix
    cargo fmt --all \

install-cli:
    @for dir in ./bins/*; do \
        name="${dir##*/}"; \
        echo "|> Installing $name"; \
        just install-bin $name; \
    done; \

install-bin package:
    @RUSTFLAGS='{{rustflags}}' \
    cargo install \
        --path "./bins/{{package}}" \
        --profile=release-cli \
        --all-features \
        --target='{{rust_target}}' \
    && if [ -n "${INSTALL_LOCATION:-}" ]; then \
        echo "|=> Moving \`{{package}}\` to $INSTALL_LOCATION"; \
        mv "$HOME/.cargo/bin/{{package}}" "$INSTALL_LOCATION"; \
    else \
        echo "No install location specified, keeping in ~/.cargo/bin"; \
    fi \