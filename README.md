[![Rust](https://github.com/timKraeuter/rust_bpmn_analyzer/actions/workflows/rust.yml/badge.svg)](https://github.com/timKraeuter/RustBPMNAnalyzer/actions/workflows/rust.yml)

[Benchmarks using Bencher🐰](https://bencher.dev/console/projects/rust-bpmn-analyzer/perf)

# Rust BPMN Analyzer

This is a simple BPMN analyzer written in Rust. It is able to parse BPMN files and analyze them for
certain properties. The analyzer is able to detect deadlocks, livelocks, and other properties of
BPMN models.

# BPMN coverage

The BPMN elements in green are supported by the analyzer. We follow the structure of the [Camunda 8 documentation](https://docs.camunda.io/docs/components/modeler/bpmn/bpmn-coverage/).

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


![](./documentation/assets/bpmn-symbols/none-start-event.svg)
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
            <a href="./documentation/assets/none-start-event">None</a>
        </td>
        <td>
            <a href="../none-events/">
                <NoneStartEventSvg className="implemented" />
            </a>
        </td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td>
            <a href="../none-events/">
                <NoneThrowEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../none-events/">
                <NoneEndEventSvg className="implemented" />
            </a>
        </td>
    </tr>
    <tr>
        <td>
            <a href="../message-events/">Message</a>
        </td>
        <td>
            <a href="../message-events/">
                <MessageStartEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../message-events/">
                <MessageEventSubprocessSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../message-events/">
                <MessageEventSubprocessNonInterruptingSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../message-events/">
                <MessageCatchEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../message-events/">
                <MessageBoundaryEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../message-events/">
                <MessageBoundaryEventNonInterruptingSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../message-events/">
                <MessageThrowEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../message-events/">
                <MessageEndEventSvg className="implemented" />
            </a>
        </td>
    </tr>
    <tr>
        <td>
            <a href="../timer-events/">Timer</a>
        </td>
        <td>
            <a href="../timer-events/">
                <TimerStartEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../timer-events/">
                <TimerEventSubprocessSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../timer-events/">
                <TimerEventSubprocessNonInterruptingSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../timer-events/">
                <TimerCatchEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../timer-events/">
                <TimerBoundaryEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../timer-events/">
                <TimerBoundaryEventNonInterruptingSvg className="implemented" />
            </a>
        </td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <td>
            <a href="../error-events/">Error</a>
        </td>
        <td></td>
        <td>
            <a href="../error-events/">
                <ErrorEventSubprocessSvg className="implemented" />
            </a>
        </td>
        <td></td>
        <td></td>
        <td>
            <a href="../error-events/">
                <ErrorBoundaryEventSvg className="implemented" />
            </a>
        </td>
        <td></td>
        <td></td>
        <td>
            <a href="../error-events/">
                <ErrorEndEventSvg className="implemented" />
            </a>
        </td>
    </tr>
    <tr>
        <td>
            <a href="../signal-events/">Signal</a>
        </td>
        <td>
            <a href="../signal-events/">
                <SignalStartEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../signal-events/">
                <SignalEventSubprocessSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../signal-events/">
                <SignalEventSubprocessNonInterruptingSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../signal-events/">
                <SignalCatchEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../signal-events/">
                <SignalBoundaryEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../signal-events/">
                <SignalBoundaryEventNonInterruptingSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../signal-events/">
                <SignalThrowEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../signal-events/">
                <SignalEndEventSvg className="implemented" />
            </a>
        </td>
    </tr>
    <tr>
        <td>
            Conditional
        </td>
        <td>
            <a href="#">
                <ConditionalStartEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <ConditionalEventSubprocessSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <ConditionalEventSubprocessNonInterruptingSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <ConditionalCatchEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <ConditionalBoundaryEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <ConditionalBoundaryEventNonInterruptingSvg />
            </a>
        </td>
        <td></td>
        <td></td>
    </tr>
    <tr>
        <td>
            <a href="../escalation-events/">Escalation</a>
        </td>
        <td></td>
        <td>
            <a href="../escalation-events/">
                <EscalationEventSubprocessSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../escalation-events">
                <EscalationEventSubprocessNonInterruptingSvg className="implemented" />
            </a>
        </td>
        <td></td>
        <td>
            <a href="../escalation-events">
                <EscalationBoundaryEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../escalation-events">
                <EscalationBoundaryEventNonInterruptingSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../escalation-events">
                <EscalationThrowEventSvg className="implemented" />
            </a>
        </td>
        <td>
            <a href="../escalation-events">
                <EscalationEndEventSvg className="implemented" />
            </a>
        </td>
    </tr>
    <tr>
        <td>
            Compensation
        </td>
        <td></td>
        <td>
            <a href="#">
                <CompensationEventSubprocessSvg />
            </a>
        </td>
        <td></td>
        <td></td>
        <td>
            <a href="#">
                <CompensationBoundaryEventSvg />
            </a>
        </td>
        <td></td>
        <td>
            <a href="#">
                <CompensationThrowEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <CompensationEndEventSvg />
            </a>
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
            <a href="#">
                <CancelBoundaryEventSvg />
            </a>
        </td>
        <td></td>
        <td></td>
        <td>
            <a href="#">
                <CancelEndEventSvg />
            </a>
        </td>
    </tr>
    <tr>
        <td>
            <a href="../terminate-events/">Terminate</a>
        </td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
        <td>
            <a href="../terminate-events/">
                <TerminationEndEventSvg className="implemented" />
            </a>
        </td>
    </tr>
    <tr>
        <td>
            <a href="../link-events">Link</a>
        </td>
        <td></td>
        <td></td>
        <td></td>
        <td>
            <a href="../link-events/">
                <LinkCatchEventSvg className="implemented"/>
            </a>
        </td>
        <td></td>
        <td></td>
        <td>
            <a href="../link-events">
                <LinkThrowEventSvg className="implemented"/>
            </a>
        </td>
        <td></td>
    </tr>
    <tr>
        <td>
            Multiple
        </td>
        <td>
            <a href="#">
                <MultipleStartEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleEventSubprocessSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleEventSubprocessNonInterruptingSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleCatchEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleBoundaryEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleBoundaryEventNonInterruptingSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleThrowEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleEndEventSvg />
            </a>
        </td>
    </tr>
    <tr>
        <td>
            Multiple Parallel
        </td>
        <td>
            <a href="#">
                <MultipleParallelStartEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleParallelEventSubprocessSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleParallelEventSubprocessNonInterruptingSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleParallelCatchEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleParallelBoundaryEventSvg />
            </a>
        </td>
        <td>
            <a href="#">
                <MultipleParallelBoundaryEventNonInterruptingSvg />
            </a>
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
