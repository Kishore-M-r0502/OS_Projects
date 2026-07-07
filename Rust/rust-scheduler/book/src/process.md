# Chapter 2: Processes

A process is the basic unit of execution managed by the scheduler. In a real operating system, a process contains executable code, memory mappings, registers, open files, and many other resources. This simulator focuses only on the information required to study scheduling algorithms.

Each process is represented by a unique Process Identifier (PID), its current execution state, and several timing fields used to measure scheduling performance.

```text
Process
├── pid
├── state
├── arrival_time
├── burst_time
├── start_time
└── completion_time
```

## Process Identifier

Every process is assigned a unique Process Identifier (PID) by the scheduler when it is created.

Unlike the original Python implementation, which generated random process identifiers, this Rust implementation assigns sequential identifiers beginning at PID 1.

This approach has several advantages:

* Predictable program execution.
* Deterministic unit tests.
* Easier debugging.
* Consistent examples throughout this book.

The scheduler is responsible for assigning identifiers because it owns the collection of processes.

## Process State

Each process stores its own execution state.

Initially, every process is placed in the **Ready** state. During execution, scheduling algorithms update the process state as execution progresses.

The current simulator models four classical process states:

* Ready
* Running
* Waiting
* Done

Future scheduling algorithms will transition between these states as they simulate process execution.

## Burst Time

A burst time represents the amount of CPU time required for a process to complete.

Rather than generating execution times during scheduling, each process stores its burst time when it is created. Scheduling algorithms consume this information while advancing the simulation clock.

## Timing Information

Several timestamps are recorded during execution.

* **Arrival Time** records when the process enters the system.
* **Start Time** records when the scheduler first dispatches the process.
* **Completion Time** records when execution finishes.

Initially, the start and completion times are unknown and are therefore represented using Rust's `Option<u32>` type.

As scheduling algorithms execute, these values are populated by the scheduler.

These timestamps allow scheduling metrics such as waiting time, turnaround time, and response time to be computed without requiring additional bookkeeping.

## Process Behaviour

A process owns its own state transitions.

Instead of allowing external modules to modify internal fields directly, the process exposes methods such as `start()` and `finish()`. These methods update the process state and record the appropriate timestamps.

This design keeps scheduling algorithms simple while ensuring that the process remains responsible for maintaining its own internal consistency.
