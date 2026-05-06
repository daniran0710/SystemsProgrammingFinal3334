#[derive(Default)]
pub struct Metrics {
    pub total_tasks: usize,
    pub cpu_completed: usize,
    pub io_completed: usize,

    pub wait_times: Vec<u128>,
    pub wait_times_cpu: Vec<u128>,
    pub wait_times_io: Vec<u128>,

    pub turnaround_times: Vec<u128>,

    pub max_wait: u128,
    pub max_wait_task_id: usize,

    pub cpu_usage_samples: Vec<f64>,
    pub workers_active_samples: Vec<f64>,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_task(
        &mut self,
        task_id: usize,
        wait: u128,
        turnaround: u128,
        is_cpu: bool,
    ) {
        self.total_tasks += 1;

        if is_cpu {
            self.cpu_completed += 1;
            self.wait_times_cpu.push(wait);
        } else {
            self.io_completed += 1;
            self.wait_times_io.push(wait);
        }

        self.wait_times.push(wait);
        self.turnaround_times.push(turnaround);

        if wait > self.max_wait {
            self.max_wait = wait;
            self.max_wait_task_id = task_id;
        }
    }

    pub fn record_monitor_sample(&mut self, cpu_usage: f64, workers_active: f64) {
        self.cpu_usage_samples.push(cpu_usage);
        self.workers_active_samples.push(workers_active);
    }

    pub fn finalize(self) -> SummaryMetrics {
        SummaryMetrics {
            total_tasks: self.total_tasks,
            cpu_completed: self.cpu_completed,
            io_completed: self.io_completed,

            avg_wait: avg_u128(&self.wait_times),
            avg_wait_cpu: avg_u128(&self.wait_times_cpu),
            avg_wait_io: avg_u128(&self.wait_times_io),
            avg_turnaround: avg_u128(&self.turnaround_times),

            max_wait: self.max_wait,
            max_wait_task_id: self.max_wait_task_id,

            avg_cpu_usage: avg(&self.cpu_usage_samples),
            avg_workers_active: avg(&self.workers_active_samples),
            monitor_samples: self.cpu_usage_samples.len(),
        }
    }
}

fn avg(v: &[f64]) -> f64 {
    if v.is_empty() {
        0.0
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

fn avg_u128(v: &[u128]) -> f64 {
    if v.is_empty() {
        0.0
    } else {
        v.iter().map(|x| *x as f64).sum::<f64>() / v.len() as f64
    }
}

pub struct SummaryMetrics {
    pub total_tasks: usize,
    pub cpu_completed: usize,
    pub io_completed: usize,

    pub avg_wait: f64,
    pub avg_wait_cpu: f64,
    pub avg_wait_io: f64,
    pub avg_turnaround: f64,

    pub max_wait: u128,
    pub max_wait_task_id: usize,

    pub avg_cpu_usage: f64,
    pub avg_workers_active: f64,
    pub monitor_samples: usize,
}

impl SummaryMetrics {
    pub fn print_fifo(
        &self,
        total_tasks: usize,
        io_pct: usize,
        cpu_pct: usize,
        workers: usize,
        total_runtime: u128,
    ) {
        println!("== FIFO simulation ==");
        println!("{total_tasks} tasks, {io_pct}% IO / {cpu_pct}% CPU, {workers} workers\n");

        println!("— results —");
        println!("total runtime          : {} ms", total_runtime);
        println!("makespan               : {} ms", total_runtime);
        println!(
            "tasks completed        : {}   (IO={}, CPU={})",
            self.total_tasks, self.io_completed, self.cpu_completed
        );
        println!("avg wait time          : {:.2} ms", self.avg_wait);
        println!("avg wait IO only       : {:.2} ms", self.avg_wait_io);
        println!("avg wait CPU only      : {:.2} ms", self.avg_wait_cpu);
        println!("avg turnaround time    : {:.2} ms", self.avg_turnaround);
        println!(
            "max wait time          : {} ms task #{}",
            self.max_wait, self.max_wait_task_id
        );
        println!("avg CPU usage          : {:.2} %", self.avg_cpu_usage);
        println!(
            "avg workers active     : {:.2} / {}",
            self.avg_workers_active, workers
        );
        println!("monitor samples        : {}", self.monitor_samples);
    }

    pub fn print_optimized(
        &self,
        total_tasks: usize,
        io_pct: usize,
        cpu_pct: usize,
        workers: usize,
        total_runtime: u128,
    ) {
        println!("== Optimized simulation ==");
        println!("{total_tasks} tasks, {io_pct}% IO / {cpu_pct}% CPU, {workers} workers\n");

        println!("— results —");
        println!("total runtime          : {} ms", total_runtime);
        println!("makespan               : {} ms", total_runtime);
        println!(
            "tasks completed        : {}   (IO={}, CPU={})",
            self.total_tasks, self.io_completed, self.cpu_completed
        );
        println!("avg wait time          : {:.2} ms", self.avg_wait);
        println!("avg wait IO only       : {:.2} ms", self.avg_wait_io);
        println!("avg wait CPU only      : {:.2} ms", self.avg_wait_cpu);
        println!("avg turnaround time    : {:.2} ms", self.avg_turnaround);
        println!(
            "max wait time          : {} ms task #{}",
            self.max_wait, self.max_wait_task_id
        );
        println!("avg CPU usage          : {:.2} %", self.avg_cpu_usage);
        println!(
            "avg workers active     : {:.2} / {}",
            self.avg_workers_active, workers
        );
        println!("monitor samples        : {}", self.monitor_samples);
    }
}