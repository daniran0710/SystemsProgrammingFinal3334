mod task;
mod dispatcher;
mod worker;
mod monitor;
mod metrics;

use std::sync::mpsc;
use std::thread;
use std::time::Instant;

use dispatcher::{DispatchMessage, Dispatcher};
use worker::Worker;
use monitor::Monitor;
use task::Task;

const WORKER_COUNT: usize = 8;
const TOTAL_TASKS: usize = 1000;
const IO_PERCENT: usize = 80;
const CPU_PERCENT: usize = 20;
const OPTIMIZED: bool = true;

fn main() {
    let (task_tx, task_rx) = mpsc::channel::<Task>();
    let (req_tx, req_rx) = mpsc::channel::<DispatchMessage>();
    let (monitor_tx, monitor_rx) = mpsc::channel();

    let start_time = Instant::now();

    let gen_thread = thread::spawn(move || {
        task::generate_tasks(TOTAL_TASKS, IO_PERCENT, task_tx);
    });

    let mut worker_senders = Vec::new();
    let mut worker_threads = Vec::new();

    for id in 0..WORKER_COUNT {
        let (assign_tx, assign_rx) = mpsc::channel();

        worker_senders.push(assign_tx);

        let req_tx_clone = req_tx.clone();
        let monitor_tx_clone = monitor_tx.clone();

        worker_threads.push(thread::spawn(move || {
            let mut worker = Worker::new(id, req_tx_clone, assign_rx, monitor_tx_clone);
            worker.run();
        }));
    }

    let mut dispatcher = Dispatcher::new(
        task_rx,
        req_rx,
        worker_senders,
        OPTIMIZED,
    );

    let dispatcher_thread = thread::spawn(move || {
        dispatcher.run();
    });

    let monitor_thread = thread::spawn(move || {
        let mut monitor = Monitor::new(monitor_rx, WORKER_COUNT);
        monitor.run();
        monitor.finalize()
    });

    gen_thread.join().unwrap();
    dispatcher_thread.join().unwrap();

    for t in worker_threads {
        t.join().unwrap();
    }

    let metrics = monitor_thread.join().unwrap();

    let total_runtime = start_time.elapsed().as_millis();

    if OPTIMIZED {
        metrics.print_optimized(
            TOTAL_TASKS,
            IO_PERCENT,
            CPU_PERCENT,
            WORKER_COUNT,
            total_runtime,
        );
    } else {
        metrics.print_fifo(
            TOTAL_TASKS,
            IO_PERCENT,
            CPU_PERCENT,
            WORKER_COUNT,
            total_runtime,
        );
    }
}