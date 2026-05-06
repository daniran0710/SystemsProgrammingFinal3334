use rand::Rng;
use std::sync::mpsc::Sender;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub enum TaskKind {
    CPU,
    IO,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: usize,
    pub arrival_time: u128,
    pub duration: u128,
    pub kind: TaskKind,
}

impl Task {
    pub fn new(id: usize, kind: TaskKind, duration: u128) -> Self {
        Self {
            id,
            arrival_time: now_ms(),
            duration,
            kind,
        }
    }
}

pub fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn generate_tasks(total: usize, io_percent: usize, tx: Sender<Task>) {
    let mut rng = rand::thread_rng();

    for id in 0..total {
        let is_io = rng.gen_range(0..100) < io_percent;

        let kind = if is_io {
            TaskKind::IO
        } else {
            TaskKind::CPU
        };

        let duration = match kind {
            TaskKind::CPU => rng.gen_range(5..25),
            TaskKind::IO => rng.gen_range(20..60),
        };

        let task = Task::new(id, kind, duration);
        tx.send(task).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(
            rng.gen_range(1..5),
        ));
    }

    let _ = tx.send(Task {
        id: usize::MAX,
        arrival_time: now_ms(),
        duration: 0,
        kind: TaskKind::CPU,
    });
}