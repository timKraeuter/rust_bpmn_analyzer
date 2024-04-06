[![Rust](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/rust.yml/badge.svg)](https://github.com/timKraeuter/RustBPMNAnalyzer/actions/workflows/rust.yml)

[Benchmarks using Bencher🐰](https://bencher.dev/console/projects/rust-bpmn-analyzer/perf)

# Rust BPMN Analyzer

This is a simple BPMN analyzer written in Rust. It is able to parse BPMN files and analyze them for
certain properties. The analyzer is able to detect deadlocks, livelocks, and other properties of
BPMN models.

# BPMN coverage

The BPMN elements in green are supported by the analyzer.

## Participants

![Pool](./documentation/assets/bpmn-symbols/pool.svg)

## Subprocesses



## Tasks

## Gateways

# Build release for the current platform

```bash
cargo build --release
```

Use WSL to build for linux on windows.
