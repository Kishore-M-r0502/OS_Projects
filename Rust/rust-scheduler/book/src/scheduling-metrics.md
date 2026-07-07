# Scheduling Metrics

Implementing a scheduling algorithm is only part of the problem. Operating systems also need a way to evaluate how well a scheduling algorithm performs.

Scheduling metrics provide quantitative measurements that allow different algorithms to be compared fairly.

In this simulator, each process records the information required to compute these metrics. The scheduler then aggregates the results across all completed processes.

## Waiting Time

Waiting time measures how long a process spends waiting in the ready queue before receiving CPU time.

The formula is:

```text
Waiting Time = Turnaround Time − Burst Time
```

A lower waiting time generally indicates that processes spend less time waiting for execution.

### Example

| Arrival | Burst | Completion |
| ------: | ----: | ---------: |
|       0 |     5 |          8 |

```text
Turnaround = 8
Waiting    = 8 − 5 = 3
```

## Turnaround Time

Turnaround time measures the total amount of time a process spends in the system.

The formula is:

```text
Turnaround Time = Completion Time − Arrival Time
```

This includes both waiting time and execution time.

### Example

| Arrival | Completion |
| ------: | ---------: |
|       0 |         12 |

```text
Turnaround = 12 − 0 = 12
```

## Response Time

Response time measures how quickly a process receives CPU time for the first time.

The formula is:

```text
Response Time = Start Time − Arrival Time
```

Unlike turnaround time, response time only considers the first execution of a process.

### Example

| Arrival | Start |
| ------: | ----: |
|       0 |     4 |

```text
Response = 4 − 0 = 4
```

## Process-Level Metrics

Each `Process` computes its own metrics.

This design keeps the mathematical formulas close to the data they depend on.

The process provides methods such as:

* `waiting_time()`
* `turnaround_time()`
* `response_time()`

These methods return `Option<u32>` because some metrics cannot be computed until the required scheduling events have occurred.

For example, a process that has not yet finished does not have a turnaround time.

## Scheduler-Level Metrics

The scheduler computes averages across all completed processes.

Examples include:

* `average_waiting_time()`
* `average_turnaround_time()`
* `average_response_time()`

The scheduler ignores processes whose metrics are unavailable. If no completed processes exist, the average returned is `0.0`.

This separation of responsibilities keeps the implementation simple:

* A `Process` computes its own metrics.
* The `Scheduler` computes aggregate statistics.

## Why These Metrics Matter

Different scheduling algorithms have different performance characteristics.

For example:

* FCFS is simple but can produce long waiting times.
* Round Robin often improves response time for interactive workloads.
* Shortest Job First usually minimizes average waiting time.
* Priority Scheduling favors important processes but may delay lower-priority ones.

By measuring the same metrics for every algorithm, the simulator can compare scheduling policies objectively.

## Summary

Scheduling metrics transform the simulator from a program that executes scheduling algorithms into a tool that evaluates them.

As additional algorithms are implemented, these metrics will provide a consistent basis for comparing their behavior and performance.
