---
title: About the Simulator
description: Design thinking and architecture behind the Python OS Scheduler Simulator
author: Kishore M R
program: B.Tech Computer Science Engineering, Semester 4
type: Self-initiated learning project
---

# About the Python OS Scheduler Simulator

---

## What This Project Is

This is a simulator — not a real scheduler. It does not interact
with the OS kernel or manage actual processes. What it does is
model the decision-making logic of four CPU scheduling policies
on randomly generated processes, and expose that logic through
a flag-driven CLI.

!!! info "Why build it?"
    The goal was to understand scheduling from the inside —
    by building it, not just reading about it.

---

## A Framework for Thinking About Schedulers

Most resources describe scheduling algorithms by mechanism.
This project approaches them by classification.

Every scheduling algorithm can be understood through three views:

| View | Question | Example |
|---|---|---|
| **Positional** | Who arrives first? | FCFS |
| **Access** | Who moves first? | Priority |
| **Instance** | Who stays longer? | Round Robin |

!!! note "PAI Framework"
    This Positional/Access/Instance classification is not standard
    terminology. It emerged from thinking about what actually
    differentiates one algorithm from another at a conceptual level,
    beyond just implementation details.

---

## Policy vs Algorithm

These two terms are often used interchangeably. This project
treats them as distinct:

!!! abstract "Policy"
    Defines the rule of fairness or efficiency.
    Answers: *what outcome are we optimizing for?*
    If it cannot generate a baseline efficiency from its own
    requirements, it is not a policy — it is a situational response.

!!! abstract "Algorithm"
    Implements a policy.
    Answers: *how do we achieve that outcome mechanically?*

---

## The Four Policies

=== "FCFS"
    The simplest policy. Processes execute in arrival order,
    start to finish, with no interruption.

    **PAI view:** Positional — position in the queue determines everything.

=== "Priority"
    Processes are assigned a priority externally. The scheduler
    sorts by priority and executes in that order.

    **PAI view:** Access — who gets access is determined before execution begins.

=== "Round Robin"
    Each process runs for a fixed quantum. If it doesn't complete,
    it re-enters the queue. Repeats until all processes finish.

    **PAI view:** Instance — how long each process holds the CPU per
    instance is the defining constraint.

    !!! warning "Known limitation"
        If the quantum exceeds total scheduler time,
        some processes never complete.

=== "SJF"
    The process with the shortest burst time runs first.
    Minimizes average waiting time but requires knowing burst
    times in advance.

    !!! tip
        In real systems, burst times are rarely known in advance —
        which is why SJF is mostly theoretical.

---

## Architecture

The project has three components:

=== "Process"
    A task with an identifier and burst time.

=== "Scheduler"
    Generates execution order based on the selected policy.

=== "Interface"
    Controls what the user sees via flags.

!!! info "Flag design"
    `-v` and `-A` expose different layers of the simulation —
    human-readable output vs raw state transitions.
    `-r` redirects state transitions to a file,
    useful for inspecting longer runs.

---

## Conclusion

!!! quote
    A simulator is a controlled environment for understanding
    something that is otherwise hidden inside an OS kernel.
    The value here is not the code — it is what the code makes visible.