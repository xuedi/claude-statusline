# List available recipes
default:
    @just --list

# Build the debug binary
build:
    cargo build

# Build the release binary
release:
    cargo build --release

# Install the release binary to ~/.claude/
install: release
    install -Dm755 target/release/claude-statusline ~/.claude/claude-statusline

# Remove the installed binary from ~/.claude/
uninstall:
    rm -f ~/.claude/claude-statusline

# Run the test suite
test:
    cargo test

# Run clippy with warnings as errors
lint:
    cargo clippy --all-targets -- -D warnings

# Format the codebase with rustfmt
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt --check

# Run fmt-check, lint, and tests (pre-commit gate)
check: fmt-check lint test

# Run the binary with ARGS forwarded to cargo run
run *ARGS:
    cargo run -- {{ARGS}}

# Pipe a sample statusline payload through the binary
demo:
    @echo '{"model":{"display_name":"Claude Sonnet 4.6 (1M context)"},"cwd":"/tmp","context_window":{"context_window_size":1000000,"current_usage":{"input_tokens":250000}}}' | cargo run --quiet

# Remove cargo build artifacts
clean:
    cargo clean
