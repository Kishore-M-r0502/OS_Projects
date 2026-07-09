use crate::scheduler::Scheduler;

#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    None,
    Fcfs,
    RoundRobin,
}

pub struct App {
    pub scheduler: Scheduler,
    pub algorithm: Algorithm,
    pub running: bool,
    pub status: String,
}
impl App {
    pub fn new() -> Self {
        Self {
            scheduler: Scheduler::from_bursts(vec![5, 3, 1]),
            algorithm: Algorithm::None,
            running: true,
            status: String::from("Ready"),
        }
    }
}

impl App {
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}