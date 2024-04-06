[![Rust](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/rust.yml/badge.svg)](https://github.com/timKraeuter/RustBPMNAnalyzer/actions/workflows/rust.yml)

[Benchmarks using Bencher🐰](https://bencher.dev/console/projects/rust-bpmn-analyzer/perf)

# Rust BPMN Analyzer

This is a simple BPMN analyzer written in Rust. It is able to parse BPMN files and analyze them for
certain properties. The analyzer is able to detect deadlocks, livelocks, and other properties of
BPMN models.

# BPMN coverage

The BPMN elements in green are supported by the analyzer.

## Participants

Multiple pools are supported. The Analyzer will start one process for each pool and then analyze the given properties.

![Pool](./documentation/assets/bpmn-symbols/pool.svg)

## Subprocesses

Call Activities are handled as tasks, i.e., we assume they terminate after a certain amount of time.

![Call activity](./documentation/assets/bpmn-symbols/call-activity.svg)
![Embedded subprocess](./documentation/assets/bpmn-symbols/embedded-subprocess.svg)
![Event subprocess](./documentation/assets/bpmn-symbols/event-subprocess.svg)
![Transactional subprocess](./documentation/assets/bpmn-symbols/transactional-subprocess.svg)

## Tasks

All tasks are handled identically expect Send/Receive tasks which send/receive messages.

![Service Task](./documentation/assets/bpmn-symbols/service-task.svg)
![User Task](./documentation/assets/bpmn-symbols/user-task.svg)
![Receive Task](./documentation/assets/bpmn-symbols/receive-task.svg)
![Send Task](./documentation/assets/bpmn-symbols/send-task.svg)
![Business Rule Task](./documentation/assets/bpmn-symbols/business-rule-task.svg)
![Script Task](./documentation/assets/bpmn-symbols/script-task.svg)
![Manual Task](./documentation/assets/bpmn-symbols/manual-task.svg)
![Undefined Task](./documentation/assets/bpmn-symbols/undefined-task.svg)
![Receive Task Instantiated](./documentation/assets/bpmn-symbols/receive-task-instantiated.svg)

## Gateways

![Exclusive Gateway](./documentation/assets/bpmn-symbols/exclusive-gateway.svg)
![Parallel Gateway](./documentation/assets/bpmn-symbols/parallel-gateway.svg)
![Event-based Gateway](./documentation/assets/bpmn-symbols/event-based-gateway.svg)
![Inclusive Gateway](./documentation/assets/bpmn-symbols/inclusive-gateway.svg)
![Complex Gateway](./documentation/assets/bpmn-symbols/complex-gateway.svg)

## Markers, Data & Artifacts

Markers, data and artifacts are ignored.

## Events



# Build release for the current platform

```bash
cargo build --release
```

Use WSL to build for linux on windows.
