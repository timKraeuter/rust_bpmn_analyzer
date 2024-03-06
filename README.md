[![Rust](https://github.com/timKraeuter/RustBPMNAnalyzer/actions/workflows/rust.yml/badge.svg)](https://github.com/timKraeuter/RustBPMNAnalyzer/actions/workflows/rust.yml)

# RustBPMNAnalyzer

This is a simple BPMN analyzer written in Rust. It is able to parse BPMN files and analyze them for
certain properties. The analyzer is able to detect deadlocks, livelocks, and other properties of
BPMN models.

```bash
cd ./target/release && bpmnanalyzer.exe -p 3001
```

# Build & Deploy

Build with the correct target for alpine, which is used in the Docker image.
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

Use WSL/Linux to build. If needed add the target:

```bash
rustup target add x86_64-unknown-linux-musl
```


Build the application image:

```bash
docker build -t rust-bpmnanalyzer .
```

Run the application image:

```bash
docker run -p 8080:8080 rust-bpmnanalyzer
```