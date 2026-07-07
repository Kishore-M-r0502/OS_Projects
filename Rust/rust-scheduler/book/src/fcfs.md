# First Come First Served (FCFS)

First Come First Served (FCFS) is one of the simplest CPU scheduling algorithms. As its name suggests, processes are executed in the order in which they arrive. Once a process begins executing, it continues until it completes. The scheduler does not interrupt a running process.

Because a running process is never preempted, FCFS is classified as a **non-preemptive scheduling algorithm**.

## How FCFS Works

The scheduler visits each process in arrival order.

For every process, it performs the following steps:

1. Record the current simulation time as the process's start time.
2. Change the process state from **Ready** to **Running**.
3. Execute the process for its CPU burst time.
4. Advance the scheduler's simulation clock.
5. Record the completion time.
6. Change the process state to **Done**.

Conceptually, the algorithm is:

```text
for each process

    start process

    advance simulation time by burst time

    finish process
```

## Simulation Example

Consider three processes with the following burst times.

| Process | Burst Time |
| ------: | ---------: |
|      P1 |          4 |
|      P2 |          2 |
|      P3 |          3 |

Initially, the scheduler clock is zero.

```text
Time = 0

P1 Ready
P2 Ready
P3 Ready
```

The scheduler executes the processes one after another.

```text
0        4    6       9
|--------|----|-------|
   P1      P2    P3
```

The recorded timing information becomes:

| Process | Start | Completion |
| ------: | ----: | ---------: |
|      P1 |     0 |          4 |
|      P2 |     4 |          6 |
|      P3 |     6 |          9 |

The final simulation time is **9**, which is the sum of all burst times.

## FCFS in the Rust Simulator

The scheduler owns both the collection of processes and the simulation clock.

For each process, the FCFS algorithm:

* reads the current simulation time,
* starts the process,
* advances the scheduler clock by the process's burst time,
* finishes the process.

The process itself is responsible for updating its internal state and recording timestamps through methods such as `start()` and `finish()`. This keeps the scheduling algorithm focused on **which process executes next**, while the `Process` type maintains its own lifecycle.

## Advantages

* Extremely simple to implement.
* Easy to understand and verify.
* Fair with respect to arrival order.
* Minimal scheduling overhead.

## Disadvantages

* Short processes may wait behind long processes.
* Average waiting time can become large.
* Interactive workloads may experience poor responsiveness.
* Does not account for process priority or urgency.

## Time Complexity

If there are **n** processes, FCFS visits each process exactly once.

**Time Complexity:** `O(n)`

**Additional Space:** `O(1)`

## Summary

FCFS provides an excellent starting point for understanding CPU scheduling. Although simple, it introduces the fundamental ideas of process state transitions, simulation time, and process execution order. More advanced algorithms such as Round Robin, Priority Scheduling, and Shortest Job First build upon the same concepts while introducing different strategies for selecting the next process to execute.
