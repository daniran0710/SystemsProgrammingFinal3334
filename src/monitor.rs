use std::sync::mpsc::Receiver;

use crate::metrics::{Metrics, SummaryMetrics};

pub enum MonitorMessage {
    TaskCompleted {
        task_id: usize,
        arrival_time: u128,
        start_time: u128,
        finish_time: u128,
        is_cpu: bool,
    },
    Sample {
        cpu_usage: f64,
        workers_active: f64,
    },
    Shutdown,
}

pub struct Monitor {
    rx: Receiver<MonitorMessage>,
    metrics: Metrics,
    worker_count: usize,
}

impl Monitor {
    pub fn new(rx: Receiver<MonitorMessage>, worker_count: usize) -> Self {
        Self {
            rx,
            metrics: Metrics::new(),
            worker_count,
        }
    }

    pub fn run(&mut self) {
        let mut shutdown_count = 0;

        while let Ok(msg) = self.rx.recv() {
            match msg {
                MonitorMessage::TaskCompleted {
                    task_id,
                    arrival_time,
                    start_time,
                    finish_time,
                    is_cpu,
                } => {
                    let wait = start_time - arrival_time;
                    let turnaround = finish_time - arrival_time;

                    self.metrics
                        .record_task(task_id, wait, turnaround, is_cpu);
                }

                MonitorMessage::Sample {
                    cpu_usage,
                    workers_active,
                } => {
                    self.metrics
                        .record_monitor_sample(cpu_usage, workers_active);
                }

                MonitorMessage::Shutdown => {
                    shutdown_count += 1;

                    if shutdown_count == self.worker_count {
                        break;
                    }
                }
            }
        }
    }

    pub fn finalize(self) -> SummaryMetrics {
        self.metrics.finalize()
    }
}