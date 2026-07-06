---
title: Home
---

# Python OS Scheduler Simulator — v0.4

A CLI tool that simulates CPU scheduling algorithms using Python.
Built as a self-initiated learning project exploring OS internals.

!!! info "What this is"
    A simulator — not a real OS scheduler. It models the decision-making
    logic of four scheduling policies on randomly generated processes,
    exposed entirely through a flag-driven CLI.

## Algorithms

=== "FCFS"
    **First-Come-First-Serve** — Non-preemptive

    Processes execute in arrival order, start to finish, no interruption.
    The simplest policy — position in the queue determines everything.

=== "Priority"
    **Priority Scheduling** — Non-preemptive

    Processes are assigned a priority externally. The scheduler sorts
    by priority and executes in that order.

=== "Round Robin"
    **Round Robin** — Preemptive

    Each process runs for a fixed quantum. If it doesn't complete,
    it re-enters the queue. Repeats until all processes finish.

    !!! warning "Known limitation"
        If the quantum exceeds total scheduler time,
        some processes never complete.

=== "SJF"
    **Shortest Job First** — Non-preemptive

    The process with the shortest burst time runs first.
    Minimizes average waiting time.

    !!! tip
        In real systems, burst times are rarely known in advance —
        which is why SJF is mostly theoretical.

## Installation

!!! note "Requirements"
    Python 3.x and Git must be available on your machine.
```bash
git clone https://github.com/Kishore-M-r0502/OS_Projects.git
cd OS_Projects/scheduler
```

## Usage
```bash
python scheduler_main.py -h                                    # (1)!
python scheduler_main.py --algo fcfs -p 5                      # (2)!
python scheduler_main.py --algorithm priority -p 8 -v          # (3)!
python scheduler_main.py --algorithm rr -p 10 -q 5 -t 200 -v  # (4)!
python scheduler_main.py --algorithm rr -p 20 -q 10 -t 200 -v --autotrace # (5)!
```

1. Show the help CLI utility
2. Run FCFS with 5 processes
3. Run Priority Scheduling with verbose output
4. Run Round Robin — 10 processes, quantum 5, total time 200
5. Same as above with internal state transitions visible

## Flags

| Flag | Description |
|---|---|
| `-h` | Help CLI utility |
| `-a` / `--algorithm` | Algorithm to use (default: Round Robin) |
| `-p` | Process count |
| `-t` | Total scheduler running time (Round Robin) |
| `-q` | Per-process quantum (Round Robin) |
| `-v` | Verbose — human readable output |
| `-A` / `--autotrace` | Show internal state transitions |
| `-r` / `--route` | Redirect state transitions to a file |

!!! tip "Flag combinations"
    `-v` and `-A` expose different layers — `-v` is for humans,
    `-A` is for inspecting scheduler internals. Use both together
    for the most detailed output.

## Tests
```bash
python -m unittest
```

## Examples

=== "Help"
    ![Help](images/help.png)

=== "Autotrace"
    ![Autotrace](images/autotrace.png)

=== "Verbose"
    ![Verbose](images/verbose.png)