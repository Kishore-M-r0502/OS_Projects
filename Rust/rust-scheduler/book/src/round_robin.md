# Round Robin Scheduling

Round Robin (RR) is a preemptive CPU scheduling algorithm that gives each process a fixed amount of CPU time, called the **time quantum**. If a process does not finish within its assigned quantum, it is preempted and placed at the end of the ready queue.

This approach ensures that every runnable process receives CPU time regularly, making Round Robin well suited for interactive and time-sharing operating systems.

## Characteristics

* Preemptive scheduling algorithm
* Uses a FIFO ready queue
* Each process receives an equal time quantum
* Processes may execute multiple times before completing
* Provides good responsiveness for interactive workloads

## The Time Quantum

The time quantum determines the maximum amount of CPU time a process may use before being preempted.

If the remaining burst time is less than the quantum, the process simply finishes.

For example:

```text
Quantum = 3

Remaining Time = 7

Execution:

7 → 4 → 1 → 0
```

A process requiring less CPU time than the quantum completes immediately.

```text
Quantum = 5

Remaining Time = 2

Execution:

2 → 0
```

## Ready Queue

This implementation stores runnable processes in a ready queue implemented using Rust's `VecDeque`.

Initially, every process is inserted into the queue.

```text
Front

P0
P1
P2
P3

Back
```

The scheduler repeatedly performs the following steps:

1. Remove the process at the front of the queue.
2. Execute it for one time quantum.
3. If it finishes, remove it permanently.
4. Otherwise, place it at the back of the queue.

Example:

```text
Initial Queue

Front

P0
P1
P2

Back
```

After executing `P0` for one quantum:

```text
Front

P1
P2
P0

Back
```

## Implementation

The scheduler repeatedly executes the following algorithm:

```text
Initialize ready queue

while queue is not empty

    remove front process

    execute one quantum

    advance simulation time

    if process finished

        record completion

    else

        place process at back of queue
```

The scheduler advances the simulation clock by the actual amount of CPU time consumed. If a process requires less time than the configured quantum, only the remaining execution time is added to the clock.

## Process Execution

Each process stores two execution values:

* **Burst Time** – the original CPU burst requested by the process.
* **Remaining Time** – the amount of execution still required.

The burst time never changes during execution.

The remaining time decreases after every time slice until it reaches zero.

When the remaining time becomes zero, the process finishes and records its completion time.

## Response Time

Round Robin may execute the same process many times.

To compute response time correctly, the process records its start time only during its first dispatch.

Subsequent executions do not modify the recorded start time.

This allows response time to remain correct even though the scheduler repeatedly preempts and resumes processes.

## Example

Consider three processes:

| Process | Burst Time |
| ------- | ---------: |
| P0      |          5 |
| P1      |          3 |
| P2      |          1 |

Time Quantum:

```text
2
```

Execution order:

```text
0-2   P0
2-4   P1
4-6   P2
6-8   P0
8-10   P1
10-12   P0
```

All processes eventually complete while receiving CPU time fairly.

## Advantages

* Fair allocation of CPU time
* Good response time for interactive applications
* Prevents starvation
* Simple scheduling policy

## Disadvantages

* Performance depends heavily on the chosen time quantum
* Frequent context switching may reduce CPU efficiency
* Average turnaround time may be worse than algorithms such as Shortest Job First

## Complexity

For *n* processes:

* Queue operations (`push_back` and `pop_front`) are **O(1)**.
* The total running time depends on the number of time slices executed.

Using `VecDeque` allows both queue operations to execute efficiently, making it a natural choice for implementing Round Robin.

## Summary

Round Robin extends the scheduler from a non-preemptive system to a preemptive one.

Unlike FCFS, processes may execute multiple times before completion. The ready queue, time quantum, and remaining execution time together provide fair CPU sharing while maintaining simple scheduling logic.
