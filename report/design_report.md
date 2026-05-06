# Task Dispatcher in Rust - Project Report

## Introduction

This project was designed to simulate a concurrent task scheduling system using Rust threads, channels, queues, and synchronization techniques. The system creates CPU-bound and IO-bound tasks and distributes them across a pool of worker threads. The goal of the project was to compare different scheduling behaviors while also learning how concurrency, communication, and synchronization work in Rust.

The project uses a dispatcher-based architecture where tasks are generated, queued, assigned to workers, processed, monitored, and then summarized through collected metrics. The program was implemented using Rust’s threading library and message passing channels.

---

# Architecture Description

The system contains five main components:

1. Task Generator
2. Dispatcher
3. Worker Threads
4. Monitor
5. Metrics System

The task generator creates tasks and sends them to the dispatcher. The dispatcher separates tasks into CPU and IO queues and assigns work to available workers. Worker threads process tasks and report their completion to the monitor. The monitor records statistics and stores them inside the metrics system. After all tasks finish, the final results are printed.

The system uses multiple threads running at the same time:

* 1 generator thread
* 1 dispatcher thread
* 8 worker threads
* 1 monitor thread

This design allows the program to simulate real concurrent scheduling behavior.

---

# Data Structures Used

Several important data structures were used in the project.

### Task Structure

Each task contains:

* task ID
* arrival time
* task duration
* task type (CPU or IO)

The task structure allows the dispatcher and workers to track scheduling behavior and performance.

### VecDeque Queues

The dispatcher uses two `VecDeque` queues:

* CPU queue
* IO queue

These queues allow tasks to be pushed and popped efficiently from the front of the queue.

### Metrics Vectors

The metrics system stores:

* wait times
* turnaround times
* CPU usage samples
* active worker samples

These vectors are later used to calculate averages and performance statistics.

---

# Synchronization Strategy

The project uses Rust channels for synchronization and communication between threads.

The following channels were used:

* Generator → Dispatcher
* Workers → Dispatcher
* Dispatcher → Workers
* Workers → Monitor

Channels were used because they are safer than manually sharing memory between many threads. Rust channels help avoid race conditions and make communication more organized.

The worker threads each have their own receiver channel. This was important because sharing a single receiver caused tasks to be lost during development. Giving each worker its own receiver fixed the issue and removed the deadlock problem.

Shared state was kept minimal in this project. Most communication happened through channels instead of directly sharing memory. This reduced synchronization complexity and improved safety.

---

# Scheduling Policy

Two scheduling modes were implemented:

1. FIFO Scheduling
2. Optimized Scheduling

FIFO scheduling selects the oldest arriving task first. This follows a first-come first-served approach.

The optimized scheduler compares task durations and chooses the shorter task first. This behaves similarly to a shortest-job-first strategy.

The optimized mode was enabled using a boolean flag in `main.rs`.

---

# Metrics Collected

The program collected several performance metrics:

* total runtime
* makespan
* total completed tasks
* CPU tasks completed
* IO tasks completed
* average wait time
* average turnaround time
* maximum wait time
* average CPU usage
* average active workers

These metrics were collected by the monitor and metrics systems while the simulation was running.

---

# Experiment Results

The FIFO scheduling results showed that tasks completed correctly, but average wait times became large when many IO tasks were waiting in the queue.

Example FIFO results:

* total runtime: about 4058 ms
* average wait time: about 727 ms
* average turnaround time: about 758 ms

The optimized scheduler improved average wait times and turnaround times because shorter tasks were completed sooner. CPU usage also became slightly more efficient.

However, the optimized scheduler introduced some unfairness because longer tasks could remain in the queue for a long time while shorter tasks continued getting selected first.

---

# Development Problems and Concurrency Bugs

One major concurrency issue happened during development. Initially, all workers shared the same receiver channel using `Arc<Mutex<Receiver>>`.

This caused a problem where one worker could accidentally receive another worker’s task assignment. The worker would ignore the task because the worker ID did not match, causing tasks to disappear permanently.

This created deadlock-like behavior where the program would never finish running.

The issue was fixed by giving every worker its own private receiver channel. The dispatcher stored a vector of worker senders and directly sent assignments to the correct worker. This removed task loss and allowed the system to shut down correctly.

Another challenge was making sure workers shut down cleanly after all tasks completed. A shutdown message was added so workers could exit safely instead of waiting forever.

---

# Starvation and Fairness

Starvation can still happen in the optimized scheduling mode. Since shorter tasks are preferred, very long tasks may wait much longer than shorter tasks.

FIFO scheduling is more fair because tasks are completed in arrival order, but it is usually slower overall.

This demonstrates an important trade-off between fairness and performance in scheduling systems.

---

# Lessons Learned

This project helped demonstrate how concurrency works in Rust using threads, channels, and synchronization strategies. One important lesson learned was that communication between threads must be carefully designed to avoid deadlocks and lost messages. I was running into many issues with having a deadlock and struggled to find the fix to the issues.

The project also showed how scheduling policies directly affect system performance. FIFO scheduling was simpler and fairer, while the optimized scheduler improved performance but increased the risk of starvation.

Another lesson learned was the importance of debugging concurrent systems carefully. Small synchronization mistakes can cause programs to freeze or behave unpredictably.

Overall, the project provided experience with concurrent programming, scheduling algorithms, synchronization, and performance analysis using Rust.
