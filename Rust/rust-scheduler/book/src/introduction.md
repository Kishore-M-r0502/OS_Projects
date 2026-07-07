
# Chapter 1: Introduction

Welcome to **Operating System Scheduling in Rust**.

This book documents the design and implementation of a scheduler simulator written in Rust. Along the way, we will explore the fundamental scheduling algorithms taught in operating systems courses while applying modern Rust programming practices.

The project originated as a Python scheduler simulator. While Python allowed rapid experimentation with scheduling algorithms, Rust provides an opportunity to build the simulator with stronger type safety, explicit ownership, and a modular architecture. Rather than performing a direct line-by-line translation, we will redesign the simulator using idiomatic Rust.

The simulator models a simplified operating system scheduler. Processes are created, assigned states, and executed according to a scheduling policy. As the project evolves, we will implement several classical scheduling algorithms, including:

* First Come First Served (FCFS)
* Shortest Job First (SJF)
* Priority Scheduling
* Round Robin (RR)

The simulator will also introduce concepts such as process state transitions, simulated execution time, scheduling metrics, logging, command-line configuration, and automated testing.

This book is written incrementally. Each chapter corresponds to a development milestone, allowing readers to understand both the implementation and the reasoning behind each design decision. Instead of presenting a finished codebase, we build the simulator one component at a time, explaining how each piece contributes to the overall architecture.

By the end of this book, you will have a working scheduler simulator, a deeper understanding of operating system scheduling, and practical experience applying Rust to systems programming problems.
