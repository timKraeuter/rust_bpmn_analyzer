[![Rust](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/rust.yml/badge.svg)](https://github.com/timKraeuter/RustBPMNAnalyzer/actions/workflows/rust.yml)

[Benchmarks using Bencher🐰](https://bencher.dev/console/projects/rust-bpmn-analyzer/perf)

# Rust BPMN Analyzer

This is a simple BPMN analyzer written in Rust. It is able to parse BPMN files and analyze them for
certain properties. The analyzer is able to detect deadlocks, livelocks, and other properties of
BPMN models.

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
            <img src="./documentation/assets/bpmn-symbols/timer-start-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/timer-event-subprocess.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/timer-event-subprocess-non-interrupting.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/timer-catch-event.svg"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/timer-boundary-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/timer-boundary-event-non-interrupting.svg" />
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
            <img src="./documentation/assets/bpmn-symbols/error-event-subprocess.svg" />
        </td>
        <td></td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/error-boundary-event.svg" />
        </td>
        <td></td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/error-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Signal
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/signal-start-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/signal-event-subprocess.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/signal-event-subprocess-non-interrupting.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/signal-catch-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/signal-boundary-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/signal-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/signal-throw-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/signal-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Conditional
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/conditional-start-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/conditional-event-subprocess.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/conditional-event-subprocess-non-interrupting.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/conditional-catch-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/conditional-boundary-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/conditional-boundary-event-non-interrupting.svg" />
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
            <img src="./documentation/assets/bpmn-symbols/escalation-event-subprocess.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/escalation-event-subprocess-non-interrupting.svg" />
        </td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/escalation-boundary-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/escalation-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/escalation-throw-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/escalation-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Compensation
        </td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/compensation-event-subprocess.svg" />
        </td>
        <td></td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/compensation-boundary-event.svg" />
        </td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/compensation-throw-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/compensation-end-event.svg" />
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
            <img src="./documentation/assets/bpmn-symbols/cancel-boundary-event.svg" />
        </td>
        <td></td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/cancel-end-event.svg" />
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
            <img src="./documentation/assets/bpmn-symbols/termination-end-event.svg" />
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
            <img src="./documentation/assets/bpmn-symbols/link-catch-event.svg"/>
        </td>
        <td></td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/link-throw-event.svg"/>
        </td>
        <td></td>
    </tr>
    <tr>
        <td>
            Multiple
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-start-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-event-subprocess.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-catch-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-boundary-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-throw-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-end-event.svg" />
        </td>
    </tr>
    <tr>
        <td>
            Multiple Parallel
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-parallel-start-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-parallel-event-subprocess.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-parallel-boundary-event-non-interrupting.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-parallel-catch-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-parallel-boundary-event.svg" />
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/multiple-parallel-boundary-event-non-interrupting.svg" />
        </td>
        <td></td>
        <td></td>
    </tr>

  </tbody>
</table>

# Build release for the current platform

```bash
cargo build --release
```

Use WSL to build for linux on windows.
