[![Rust](https://github.com/timKraeuter/RustBPMNAnalyzer/actions/workflows/rust.yml/badge.svg)](https://github.com/timKraeuter/RustBPMNAnalyzer/actions/workflows/rust.yml)

# RustBPMNAnalyzer

This is a simple BPMN analyzer written in Rust. It is able to parse BPMN files and analyze them for
certain properties. The analyzer is able to detect deadlocks, livelocks, and other properties of
BPMN models.

# Build release for the current platform

```bash
cargo build --release
```

Use WSL to build for linux on windows.