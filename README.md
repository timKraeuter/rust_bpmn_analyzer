[![Rust](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/rust.yml/badge.svg)](https://github.com/timKraeuter/Rustrust_bpmn_analyzer/actions/workflows/rust.yml)

[Benchmarks using Bencher🐰](https://bencher.dev/console/projects/rust-bpmn-analyzer/perf)

# Rust BPMN Analyzer

This is a simple BPMN analyzer written in Rust. It is able to parse BPMN files and analyze them for
certain properties. The analyzer is able to detect deadlocks, livelocks, and other properties of
BPMN models.

```bash
cd ./target/release && rust_bpmn_analyzer.exe -p 3001
```

# Docker build

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
docker build -t rust-rust_bpmn_analyzer .
```

Run the application image:

```bash
docker run -p 8080:8080 rust-rust_bpmn_analyzer
```

## Deployment to Azure

1. Build the container (see Docker section).

2. Tag container image

```bash
docker tag rust-rust_bpmn_analyzer tg2022.azurecr.io/rust-rust_bpmn_analyzer:v1
```

3. Login to the Container Registry. Environment variables `APP_ID` and `AZURE_PW` are expected to be
   set (export APP_ID=<app-id> etc.).

```bash
docker login tg2022.azurecr.io --username $APP_ID --password $AZURE_PW
```

4. Push the image to the Container Registry

```bash
docker push tg2022.azurecr.io/rust-rust_bpmn_analyzer:v1
```

5. Create a new container app revision in the GUI.