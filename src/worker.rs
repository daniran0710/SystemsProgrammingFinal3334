use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use crate::dispatcher::{DispatchMessage, DispatcherToWorker};
use crate::monitor::MonitorMessage;
use crate::task::{now_ms, Task, TaskKind};

pub struct Worker {
    id: usize,
    request_tx: Sender<DispatchMessage>,
    assign_rx: Receiver<DispatcherToWorker>,
    monitor_tx: Sender<MonitorMessage>,
}

impl Worker {
    pub fn new(
        id: usize,
        request_tx: Sender<DispatchMessage>,
        assign_rx: Receiver<DispatcherToWorker>,
        monitor_tx: Sender<MonitorMessage>,
    ) -> Self {
        Self {
            id,
            request_tx,
            assign_rx,
            monitor_tx,
        }
    }

    pub fn run(&mut self) {
        loop {
            let _ = self.request_tx.send(DispatchMessage::RequestTask(self.id));

            match self.assign_rx.recv() {
                Ok(DispatcherToWorker::Assign(task)) => {
                    self.execute_task(task);
                }
                Ok(DispatcherToWorker::Shutdown) => {
                    let _ = self.monitor_tx.send(MonitorMessage::Shutdown);
                    break;
                }
                Err(_) => break,
            }
        }
    }

    fn execute_task(&mut self, task: Task) {
        let start_time = now_ms();

        let cpu_usage = match task.kind {
            TaskKind::CPU => 90.0,
            TaskKind::IO => 40.0,
        };

        let _ = self.monitor_tx.send(MonitorMessage::Sample {
            cpu_usage,
            workers_active: 1.0,
        });

        std::thread::sleep(Duration::from_millis(task.duration as u64));

        let finish_time = now_ms();

        let _ = self.monitor_tx.send(MonitorMessage::TaskCompleted {
            task_id: task.id,
            arrival_time: task.arrival_time,
            start_time,
            finish_time,
            is_cpu: matches!(task.kind, TaskKind::CPU),
        });
    }
}