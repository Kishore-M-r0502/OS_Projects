# Introduction

This project explores the implementation of classical operating system scheduling algorithms in Rust.

The objective is not merely to simulate scheduling, but to understand how schedulers manage processes, transition process states, and make execution decisions. Every component of the simulator is built incrementally, allowing the reader to follow both the implementation and the underlying operating system concepts.

The project began as a Python scheduler simulator. While the Python implementation provided a concise way to prototype scheduling algorithms, this Rust version focuses on stronger type safety, explicit ownership, modular design, and comprehensive testing.

Throughout this book we will implement several scheduling algorithms, including:

* First Come First Served (FCFS)
* Shortest Job First (SJF)
* Priority Scheduling
* Round Robin

Rather than writing the simulator as a single program, the implementation is divided into small modules that each represent an important operating system concept. This keeps the code organized and makes each part easier to understand, test, and extend.

By the end of this project, the scheduler will support multiple algorithms, process state transitions, scheduling metrics, command-line configuration, logging, and automated tests. More importantly, the project demonstrates how Rust's ownership system and type safety can be applied to systems programming in a practical setting.
