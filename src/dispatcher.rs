use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use crate::task::{Task, TaskKind};

pub enum DispatchMessage {
    RequestTask(usize),
}

pub enum DispatcherToWorker {
    Assign(Task),
    Shutdown,
}

pub struct Dispatcher {
    cpu_queue: VecDeque<Task>,
    io_queue: VecDeque<Task>,
    waiting_workers: VecDeque<usize>,
    incoming_tasks: Receiver<Task>,
    worker_requests: Receiver<DispatchMessage>,
    worker_senders: Vec<Sender<DispatcherToWorker>>,
    optimized: bool,
}

impl Dispatcher {
    pub fn new(
        incoming_tasks: Receiver<Task>,
        worker_requests: Receiver<DispatchMessage>,
        worker_senders: Vec<Sender<DispatcherToWorker>>,
        optimized: bool,
    ) -> Self {
        Self {
            cpu_queue: VecDeque::new(),
            io_queue: VecDeque::new(),
            waiting_workers: VecDeque::new(),
            incoming_tasks,
            worker_requests,
            worker_senders,
            optimized,
        }
    }

    pub fn run(&mut self) {
        let mut generator_done = false;

        loop {
            while let Ok(task) = self.incoming_tasks.try_recv() {
                if task.id == usize::MAX {
                    generator_done = true;
                } else {
                    match task.kind {
                        TaskKind::CPU => self.cpu_queue.push_back(task),
                        TaskKind::IO => self.io_queue.push_back(task),
                    }
                }
            }

            while let Ok(DispatchMessage::RequestTask(worker_id)) = self.worker_requests.try_recv() {
                if worker_id < self.worker_senders.len() {
                    self.waiting_workers.push_back(worker_id);
                }
            }

            while !self.waiting_workers.is_empty() {
                if let Some(task) = self.select_task() {
                    let worker_id = self.waiting_workers.pop_front().unwrap();
                    let _ = self.worker_senders[worker_id].send(DispatcherToWorker::Assign(task));
                } else {
                    break;
                }
            }

            if generator_done && self.cpu_queue.is_empty() && self.io_queue.is_empty() {
                for sender in &self.worker_senders {
                    let _ = sender.send(DispatcherToWorker::Shutdown);
                }
                break;
            }

            std::thread::sleep(Duration::from_millis(1));
        }
    }

    fn select_task(&mut self) -> Option<Task> {
        let cpu = self.cpu_queue.front();
        let io = self.io_queue.front();

        match (cpu, io) {
            (Some(c), Some(i)) => {
                if self.optimized {
                    if c.duration <= i.duration {
                        self.cpu_queue.pop_front()
                    } else {
                        self.io_queue.pop_front()
                    }
                } else if c.arrival_time <= i.arrival_time {
                    self.cpu_queue.pop_front()
                } else {
                    self.io_queue.pop_front()
                }
            }
            (Some(_), None) => self.cpu_queue.pop_front(),
            (None, Some(_)) => self.io_queue.pop_front(),
            (None, None) => None,
        }
    }
}