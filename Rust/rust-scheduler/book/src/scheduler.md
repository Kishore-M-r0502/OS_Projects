# The Scheduler

The scheduler is the central component of the simulator. While a process represents an individual unit of execution, the scheduler is responsible for managing the collection of processes and simulating the passage of CPU time.

In this project, the scheduler owns every process. New processes are created by the scheduler, assigned unique process identifiers, and stored in an internal collection.

```text
Scheduler
├── processes
└── time
```

## Owning the Process Collection

The scheduler stores processes in a `Vec<Process>`.

This design provides several advantages:

* The scheduler has complete control over process creation.
* Process identifiers can be assigned sequentially.
* Scheduling algorithms can iterate over the collection efficiently.
* The process list remains encapsulated behind the scheduler's interface.

Rather than exposing the underlying vector directly, the scheduler provides methods for accessing individual processes when required.

## Simulation Time

Unlike a real operating system, this simulator does not use the system clock.

Instead, the scheduler maintains its own simulation clock.

```text
Time = 0
```

At the beginning of every simulation, no CPU time has elapsed, so the clock starts at zero.

Whenever a scheduling algorithm executes a process, the scheduler advances the simulation clock by the amount of CPU time consumed. This allows every scheduling algorithm to share the same notion of time while remaining independent of the host operating system.

## Scheduler Interface

The scheduler exposes a small collection of methods that scheduling algorithms use to interact with the simulation.

Examples include:

* Creating a scheduler with a specified number of processes.
* Retrieving the number of managed processes.
* Accessing a process by index.
* Accessing a mutable process for state updates.
* Reading the current simulation time.
* Advancing the simulation clock.

These operations are intentionally generic. They are not tied to any specific scheduling algorithm, allowing FCFS, Round Robin, Priority Scheduling, and Shortest Job First to share the same underlying infrastructure.

## Separation of Responsibilities

A key design principle of this project is separating responsibilities between the scheduler and individual processes.

The scheduler is responsible for:

* Creating processes.
* Assigning process identifiers.
* Managing the simulation clock.
* Selecting which process should execute.

Each process is responsible for:

* Maintaining its own state.
* Recording execution timestamps.
* Providing methods that describe its lifecycle, such as starting and finishing execution.

This separation keeps the scheduling algorithms concise while making each component easier to understand, test, and extend.

The following chapters build on this foundation by implementing classical scheduling algorithms using the scheduler's interface.
