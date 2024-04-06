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

![](./documentation/assets/bpmn-symbols/none-throw-event.svg)
![](./documentation/assets/bpmn-symbols/none-end-event.svg)

![](./documentation/assets/bpmn-symbols/message-start-event.svg)
![](./documentation/assets/bpmn-symbols/message-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/message-event-subprocess-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/message-catch-event.svg)
![](./documentation/assets/bpmn-symbols/message-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/message-boundary-event-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/message-throw-event.svg)
![](./documentation/assets/bpmn-symbols/message-end-event.svg)

![](./documentation/assets/bpmn-symbols/timer-start-event.svg)
![](./documentation/assets/bpmn-symbols/timer-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/timer-event-subprocess-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/timer-catch-event.svg)
![](./documentation/assets/bpmn-symbols/timer-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/timer-boundary-event-non-interrupting.svg)

![](./documentation/assets/bpmn-symbols/error-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/error-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/error-end-event.svg)

![](./documentation/assets/bpmn-symbols/signal-start-event.svg)
![](./documentation/assets/bpmn-symbols/signal-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/signal-event-subprocess-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/signal-catch-event.svg)
![](./documentation/assets/bpmn-symbols/signal-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/signal-boundary-event-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/signal-throw-event.svg)
![](./documentation/assets/bpmn-symbols/signal-end-event.svg)

![](./documentation/assets/bpmn-symbols/conditional-start-event.svg)
![](./documentation/assets/bpmn-symbols/conditional-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/conditional-event-subprocess-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/conditional-catch-event.svg)
![](./documentation/assets/bpmn-symbols/conditional-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/conditional-boundary-event-non-interrupting.svg)

![](./documentation/assets/bpmn-symbols/escalation-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/escalation-event-subprocess-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/escalation-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/escalation-boundary-event-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/escalation-throw-event.svg)
![](./documentation/assets/bpmn-symbols/escalation-end-event.svg)

![](./documentation/assets/bpmn-symbols/compensation-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/compensation-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/compensation-throw-event.svg)
![](./documentation/assets/bpmn-symbols/compensation-end-event.svg)

![](./documentation/assets/bpmn-symbols/cancel-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/cancel-end-event.svg)

![](./documentation/assets/bpmn-symbols/termination-end-event.svg)

![](./documentation/assets/bpmn-symbols/link-catch-event.svg)
![](./documentation/assets/bpmn-symbols/link-throw-event.svg)

![](./documentation/assets/bpmn-symbols/multiple-start-event.svg)
![](./documentation/assets/bpmn-symbols/multiple-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/multiple-event-subprocess-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/multiple-catch-event.svg)
![](./documentation/assets/bpmn-symbols/multiple-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/multiple-boundary-event-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/multiple-throw-event.svg)
![](./documentation/assets/bpmn-symbols/multiple-end-event.svg)

![](./documentation/assets/bpmn-symbols/multiple-parallel-start-event.svg)
![](./documentation/assets/bpmn-symbols/multiple-parallel-event-subprocess.svg)
![](./documentation/assets/bpmn-symbols/multiple-parallel-event-subprocess-non-interrupting.svg)
![](./documentation/assets/bpmn-symbols/multiple-parallel-catch-event.svg)
![](./documentation/assets/bpmn-symbols/multiple-parallel-boundary-event.svg)
![](./documentation/assets/bpmn-symbols/multiple-parallel-boundary-event-non-interrupting.svg)

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
            <img src="./documentation/assets/bpmn-symbols/none-start-event.svg" alt="None Start Event" />
        </td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/none-throw-event.svg" alt="None throw Event"/>
        </td>
        <td>
            <img src="./documentation/assets/bpmn-symbols/none-end-event.svg" alt="None end Event"/>
        </td>
    </tr>
    <tr>
        <td>
            Message
        </td>
        <td>
            <MessageStartEventSvg className="implemented" />
        </td>
        <td>
            <MessageEventSubprocessSvg className="implemented" />
        </td>
        <td>
            <MessageEventSubprocessNonInterruptingSvg className="implemented" />
        </td>
        <td>
            <MessageCatchEventSvg className="implemented" />
        </td>
        <td>
            <MessageBoundaryEventSvg className="implemented" />
        </td>
        <td>
            <MessageBoundaryEventNonInterruptingSvg className="implemented" />
        </td>
        <td>
            <MessageThrowEventSvg className="implemented" />
        </td>
        <td>
            <MessageEndEventSvg className="implemented" />
        </td>
    </tr>
    <tr>
        <td>
            Timer
        </td>
        <td>
            <TimerStartEventSvg className="implemented" />
        </td>
        <td>
            <TimerEventSubprocessSvg className="implemented" />
        </td>
        <td>
            <TimerEventSubprocessNonInterruptingSvg className="implemented" />
        </td>
        <td>
            <TimerCatchEventSvg className="implemented" />
        </td>
        <td>
            <TimerBoundaryEventSvg className="implemented" />
        </td>
        <td>
            <TimerBoundaryEventNonInterruptingSvg className="implemented" />
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
            <ErrorEventSubprocessSvg className="implemented" />
        </td>
        <td></td>
        <td></td>
        <td>
            <ErrorBoundaryEventSvg className="implemented" />
        </td>
        <td></td>
        <td></td>
        <td>
            <ErrorEndEventSvg className="implemented" />
        </td>
    </tr>
    <tr>
        <td>
            Signal
        </td>
        <td>
            <SignalStartEventSvg className="implemented" />
        </td>
        <td>
            <SignalEventSubprocessSvg className="implemented" />
        </td>
        <td>
            <SignalEventSubprocessNonInterruptingSvg className="implemented" />
        </td>
        <td>
            <SignalCatchEventSvg className="implemented" />
        </td>
        <td>
            <SignalBoundaryEventSvg className="implemented" />
        </td>
        <td>
            <SignalBoundaryEventNonInterruptingSvg className="implemented" />
        </td>
        <td>
            <SignalThrowEventSvg className="implemented" />
        </td>
        <td>
            <SignalEndEventSvg className="implemented" />
        </td>
    </tr>
    <tr>
        <td>
            Conditional
        </td>
        <td>
            <ConditionalStartEventSvg />
        </td>
        <td>
            <ConditionalEventSubprocessSvg />
        </td>
        <td>
            <ConditionalEventSubprocessNonInterruptingSvg />
        </td>
        <td>
            <ConditionalCatchEventSvg />
        </td>
        <td>
            <ConditionalBoundaryEventSvg />
        </td>
        <td>
            <ConditionalBoundaryEventNonInterruptingSvg />
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
            <EscalationEventSubprocessSvg className="implemented" />
        </td>
        <td>
            <EscalationEventSubprocessNonInterruptingSvg className="implemented" />
        </td>
        <td></td>
        <td>
            <EscalationBoundaryEventSvg className="implemented" />
        </td>
        <td>
            <EscalationBoundaryEventNonInterruptingSvg className="implemented" />
        </td>
        <td>
            <EscalationThrowEventSvg className="implemented" />
        </td>
        <td>
            <EscalationEndEventSvg className="implemented" />
        </td>
    </tr>
    <tr>
        <td>
            Compensation
        </td>
        <td></td>
        <td>
            <CompensationEventSubprocessSvg />
        </td>
        <td></td>
        <td></td>
        <td>
            <CompensationBoundaryEventSvg />
        </td>
        <td></td>
        <td>
            <CompensationThrowEventSvg />
        </td>
        <td>
            <CompensationEndEventSvg />
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
            <CancelBoundaryEventSvg />
        </td>
        <td></td>
        <td></td>
        <td>
            <CancelEndEventSvg />
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
            <TerminationEndEventSvg className="implemented" />
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
            <LinkCatchEventSvg className="implemented"/>
        </td>
        <td></td>
        <td></td>
        <td>
            <LinkThrowEventSvg className="implemented"/>
        </td>
        <td></td>
    </tr>
    <tr>
        <td>
            Multiple
        </td>
        <td>
            <MultipleStartEventSvg />
        </td>
        <td>
            <MultipleEventSubprocessSvg />
        </td>
        <td>
            <MultipleEventSubprocessNonInterruptingSvg />
        </td>
        <td>
            <MultipleCatchEventSvg />
        </td>
        <td>
            <MultipleBoundaryEventSvg />
        </td>
        <td>
            <MultipleBoundaryEventNonInterruptingSvg />
        </td>
        <td>
            <MultipleThrowEventSvg />
        </td>
        <td>
            <MultipleEndEventSvg />
        </td>
    </tr>
    <tr>
        <td>
            Multiple Parallel
        </td>
        <td>
            <MultipleParallelStartEventSvg />
        </td>
        <td>
            <MultipleParallelEventSubprocessSvg />
        </td>
        <td>
            <MultipleParallelEventSubprocessNonInterruptingSvg />
        </td>
        <td>
            <MultipleParallelCatchEventSvg />
        </td>
        <td>
            <MultipleParallelBoundaryEventSvg />
        </td>
        <td>
            <MultipleParallelBoundaryEventNonInterruptingSvg />
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
