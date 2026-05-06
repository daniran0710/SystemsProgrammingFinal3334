# SystemsProgrammingFinal3334

Concurrent Task Dispatcher in Rust

Project Overview

This project implements a concurrent task dispatcher system in Rust that simulates how an operating system scheduler distributes work across multiple worker threads.

The program generates CPU-bound and IO-bound tasks, places them into queues, and dispatches them to a bounded worker pool. The system supports both FIFO scheduling and an optimized scheduling mode while collecting performance metrics during execution.

The project was built using Rust threads, channels, queues, and synchronization techniques.

----

How to Build and Run

Build
cargo build
Run FIFO Scheduler
cargo run
Run Optimized Scheduler

Inside main.rs change:

const OPTIMIZED: bool = false;

to:

const OPTIMIZED: bool = true;

Then run:

cargo run

----

System Architecture

The system contains several major components:

Task Generator
Dispatcher
Worker Pool
Monitor
Metrics System
Task Generator

Creates CPU and IO tasks with randomized durations and sends them to the dispatcher.

Dispatcher

Receives tasks, stores them inside queues, and assigns tasks to available workers based on the scheduling policy.

Worker Pool

A bounded pool of 8 worker threads that execute tasks concurrently.

Monitor

Collects runtime statistics and receives task completion information from workers.

Metrics System

Stores and calculates performance results such as wait time, turnaround time, CPU usage, and completed tasks.

----

Scheduling Policies

FIFO Scheduler
Uses arrival order
Tasks are completed in the order they enter the queue
Simpler and more fair scheduling behavior
Optimized Scheduler
Compares task durations
Prioritizes shorter tasks first
Reduces average wait time and turnaround time

---

Workload Configuration

The simulation uses:

1000 total tasks
8 worker threads
Experiment A
700 IO tasks
300 CPU tasks
70% IO workload
30% CPU workload

Configuration:

const IO_PERCENT: usize = 70;
const CPU_PERCENT: usize = 30;
Experiment B
800 IO tasks
200 CPU tasks
80% IO workload
20% CPU workload

Configuration:

const IO_PERCENT: usize = 80;
const CPU_PERCENT: usize = 20;
Task Durations

CPU Tasks:

Random duration between 5 ms and 25 ms

IO Tasks:

Random duration between 20 ms and 60 ms

Tasks arrive continuously during runtime to simulate realistic scheduling conditions.

---

Example FIFO Results

== FIFO simulation ==
1000 tasks, 70% IO / 30% CPU, 8 workers

— results —
total runtime          : 4058 ms
makespan               : 4058 ms
tasks completed        : 1000   (IO=680, CPU=320)
avg wait time          : 727.11 ms
avg wait IO only       : 722.90 ms
avg wait CPU only      : 736.05 ms
avg turnaround time    : 758.66 ms
max wait time          : 1454 ms task #996
avg CPU usage          : 56.00 %
avg workers active     : 1.00 / 8
monitor samples        : 1000

Example Optimized Results

== Optimized simulation ==
1000 tasks, 70% IO / 30% CPU, 8 workers

— results —
total runtime          : 3712 ms
makespan               : 3712 ms
tasks completed        : 1000   (IO=680, CPU=320)
avg wait time          : 601.24 ms
avg wait IO only       : 580.91 ms
avg wait CPU only      : 640.55 ms
avg turnaround time    : 632.77 ms
max wait time          : 1301 ms task #988
avg CPU usage          : 60.42 %
avg workers active     : 1.00 / 8
monitor samples        : 1000

---
Trade-offs

FIFO

Advantages:

Simple design
Fair ordering
Easier to debug

Disadvantages:

Longer wait times
Slower turnaround for short tasks
Optimized Scheduler

Advantages:

Lower average wait time
Better responsiveness
Improved turnaround time

Disadvantages:

More complex scheduling logic
Long tasks may experience starvation

---

Synchronization Strategy

The system primarily uses Rust message-passing channels for thread communication.

Channels were used between:

Generator → Dispatcher
Workers → Dispatcher
Dispatcher → Workers
Workers → Monitor

Each worker has its own receiver channel to prevent task loss and synchronization issues.

The dispatcher uses separate CPU and IO queues implemented with VecDeque.

---

Concurrency Issues During Development

One major issue during development involved workers sharing the same receiver channel using Arc<Mutex<Receiver>>.

This caused workers to accidentally receive assignments intended for different workers, resulting in lost tasks and the program freezing indefinitely.

The issue was fixed by giving each worker its own dedicated receiver channel. The dispatcher stores a vector of worker senders and directly sends assignments to the correct worker.

This removed the deadlock problem and allowed the program to terminate cleanly.

---

Required Design Questions
1. What threads or major components exist in your design?

The system contains a task generator thread, dispatcher thread, monitor thread, and 8 worker threads. The main components are the task generator, dispatcher, workers, monitor, and metrics system.

2. What data is shared, and how is it protected?

Most communication is done through channels instead of directly sharing memory. This reduces race conditions and synchronization problems.

3. Where did you use channels, and why?

Channels were used between the generator, dispatcher, workers, and monitor to safely pass messages between threads.

4. Where did you use shared state, and why?

Shared state was kept minimal. The dispatcher manages shared queues internally while communication between threads mostly happens through channels.

5. What scheduling policy did you implement?

The project implements FIFO scheduling and an optimized shortest-duration-first scheduling policy.

6. What behavior improved because of that policy?

The optimized scheduler reduced average wait times and turnaround times by completing shorter tasks earlier.

7. What behavior became worse or more complicated because of that policy?

The optimized scheduler increased complexity and introduced possible starvation for longer tasks.

8. What concurrency bug or design mistake did you hit during development?

Workers originally shared a single receiver channel, which caused tasks to be lost when workers received messages intended for different workers.

9. How did you fix it?

Each worker was given its own receiver channel, and the dispatcher directly sends tasks to the correct worker.

10. Where could starvation or unfairness still happen?

Starvation may still happen in optimized scheduling because shorter tasks are prioritized over longer tasks.

---

Lessons Learned

This project provided experience with:

Rust concurrency
Thread communication
Scheduling algorithms
Synchronization techniques
Queue management
Performance measurement

One important lesson learned was that concurrency bugs are often caused by incorrect communication design rather than simple syntax errors. Careful thread coordination and proper channel organization were necessary to make the system stable.

The project also demonstrated how different scheduling policies affect fairness, responsiveness, and overall system performance.

---
Tool Use Disclosure

AI tools were used to assist with:

debugging Rust concurrency issues
understanding synchronization strategies
improving scheduler structure
fixing deadlock problems
explaining scheduling behavior