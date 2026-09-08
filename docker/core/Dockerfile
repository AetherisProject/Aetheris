# Core Rust CI image — fmt, clippy, audit, wasm
FROM rust:1.78-bookworm
RUN apt-get update && apt-get install -y libssl-dev pkg-config curl wget
RUN rustup component add rustfmt clippy
RUN cargo install cargo-audit cargo-tarpaulin 2>/dev/null || true
WORKDIR /aetheris
COPY . .
CMD ["bash","-c","cargo fmt -- --check && cargo clippy --all-targets --all-features -- -D warnings && cargo audit"]
