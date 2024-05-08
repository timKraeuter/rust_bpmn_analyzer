[![Core](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/core.yml/badge.svg)](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/core.yml)
[![CLI](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/cli.yml/badge.svg)](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/cli.yml)
[![Webserver](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/webserver.yml/badge.svg)](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/webserver.yml)
[![Docker Hub](https://img.shields.io/docker/pulls/tkra/rust_bpmn_analyzer)](https://hub.docker.com/r/tkra/rust_bpmn_analyzer)


[Benchmarks using Bencher🐰](https://bencher.dev/console/projects/rust-bpmn-analyzer/perf)

# Rust BPMN Analyzer

This is a BPMN analyzer, i.e., a **BPMN-specific model checker** written in Rust. It can parse BPMN files and analyze them for
specific properties. The analyzer can detect deadlocks, livelocks, and other properties of
BPMN models.

The Analyzer is structured into four crates:
- `core`: The core functionality of the analyzer, i.e., analyzing BPMN models **(library)**.
- `cli`: A CLI application providing BPMN analysis **(binary)**.
- `webserver`: A webserver providing BPMN analysis as a service **(binary)**.
- `wasm`: The WebAssembly crate provides bindings to web-assembly to analyze BPMN models on the client-side in the browser.

# Core

This crate contains the core functionality of the analyzer.
See a detailed description on [my website](https://timkraeuter.com//rust-bpmn-analyzer/).

# CLI

This binary crate is a CLI application to analyze BPMN models.
The following command lists the available options:
```bash
cargo run -- -h
```
One can run the CLI application for a specific BPMN model and a set of properties like this:
```bash
cargo run -- -f benchmark_input/p10x01.bpmn -p safeness,option-to-complete,proper-completion,no-dead-activities
```

Build an optimized binary which can be used like shown above:
```bash
cargo build --release
```

# Webserver
This binary crate provides a webserver with a webservice to analyze BPMN models.
The webserver is available locally by running `main.rs`, using [docker](https://hub.docker.com/r/tkra/rust_bpmn_analyzer), or [online](https://bpm-2024.whitefield-c9fed487.northeurope.azurecontainerapps.io/).

The webserver not only serves the analyzer but also a custom [BPMN editor](https://github.com/timKraeuter/bpmn-analyzer-js) to create BPMN models and get instantaneous feedback, counter examples, and quick fixes for the checked properties.
See a detailed description on [my website](https://timkraeuter.com//rust-bpmn-analyzer/).

## Docker

The webserver is available using [docker](https://hub.docker.com/r/tkra/rust_bpmn_analyzer).
Building the image is done using GitHub actions (see release.yml).

Pull the application image:
```bash
docker pull tkra/rust_bpmn_analyzer
```

Run the application image:
```bash
docker run -p 8080:8080 tkra/rust_bpmn_analyzer
```

# Wasm
This crate provides wasm bindings to run BPMN analysis directly in the browser on the clients machine.
A demo using the WASM bindings is available [here](https://timkraeuter.com/bpmn-analyzer-js/). 

Install wasm-pack using (see [documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)):

```bash
cargo install wasm-pack
```

Run the following command in the `./wasm` directory to cross-compile to WASM:
```bash
wasm-pack build --target web --out-dir ../../bpmn-analyzer-js/src/lib/analysis/wasm/generated
```

The output can be found in `../../bpmn-analyzer-js/src/lib/analysis/wasm/generated`.

# BPMN coverage

The BPMN elements in green are supported by the analyzer. We follow the structure of
the [Camunda 8 documentation](https://docs.camunda.io/docs/components/modeler/bpmn/bpmn-coverage/).

## Participants

Multiple pools are supported. The Analyzer will start one process for each pool and then analyze the
given properties.

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

<table>
  <thead>
      <tr>
        <th>Type</th>
        <th colspan="3">Start</th>
        <th colspan="4">Intermediate</th>
        <th>End</th>
      </tr>
      <tr>
        <th></th>
        <th>Normal</th>
        <th>Event Subprocess</th>
        <th>Event Subprocess non-interrupting</th>
        <th>Catch</th>
        <th>Boundary</th>
        <th>Boundary non-interrupting</th>
        <th>Throw</th>
        <th></th>
      </tr>
  </thead>
  <tbody>
    <tr>
        <td>
            None
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/none-start-event.svg" alt="none start event" />
        </td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/none-throw-event.svg" alt="none throw event"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/none-end-event.svg" alt="none end event"/>
        </td>
    </tr>
    <tr>
        <td>
            Message
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/message-start-event.svg" alt="message start event"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/message-event-subprocess.svg" alt="message event subprocess"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/message-event-subprocess-non-interrupting.svg" alt="message event subprocess non-interrupting"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/message-catch-event.svg" alt="message catch event"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/message-boundary-event.svg" alt="message boundary event"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/message-boundary-event-non-interrupting.svg" alt="message boundary event non-interrupting"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/message-throw-event.svg" alt="message throw event"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/message-end-event.svg" alt="message end event"/>
        </td>
    </tr>
    <tr>
        <td>
            Timer
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/timer-start-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/timer-event-subprocess.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/timer-event-subprocess-non-interrupting.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/timer-catch-event.svg"/>
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/timer-boundary-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/timer-boundary-event-non-interrupting.svg" />
        </td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <td>
            Error
        </td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/error-event-subprocess.svg" />
        </td>
        <td></td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/error-boundary-event.svg" />
        </td>
        <td></td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/error-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Signal
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/signal-start-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/signal-event-subprocess.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/signal-event-subprocess-non-interrupting.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/signal-catch-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/signal-boundary-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/signal-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/signal-throw-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/signal-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Conditional
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/conditional-start-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/conditional-event-subprocess.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/conditional-event-subprocess-non-interrupting.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/conditional-catch-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/conditional-boundary-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/conditional-boundary-event-non-interrupting.svg" />
        </td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <td>
            Escalation
        </td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/escalation-event-subprocess.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/escalation-event-subprocess-non-interrupting.svg" />
        </td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/escalation-boundary-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/escalation-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/escalation-throw-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/escalation-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Compensation
        </td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/compensation-event-subprocess.svg" />
        </td>
        <td></td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/compensation-boundary-event.svg" />
        </td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/compensation-throw-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/compensation-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Cancel
        </td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/cancel-boundary-event.svg" />
        </td>
        <td></td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/cancel-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Terminate
        </td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/termination-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Link
        </td>
        <td></td>
        <td></td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/link-catch-event.svg"/>
        </td>
        <td></td>
        <td></td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/link-throw-event.svg"/>
        </td>
        <td></td>
    </tr>
    <tr>
        <td>
            Multiple
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-start-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-event-subprocess.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-catch-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-boundary-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-throw-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Multiple Parallel
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-parallel-start-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-parallel-event-subprocess.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-parallel-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-parallel-catch-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-parallel-boundary-event.svg" />
        </td>
        <td>
            <img alt="" src="./documentation/assets/bpmn-symbols/multiple-parallel-boundary-event-non-interrupting.svg" />
        </td>
        <td></td>
        <td></td>
    </tr>

  </tbody>
</table>