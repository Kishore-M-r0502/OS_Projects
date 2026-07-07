use rand::RngExt;

use crate::process::Process;

pub struct Scheduler {
    processes: Vec<Process>,
    time: u32,
}

impl Scheduler {
    pub fn new(count: usize) -> Self {
        let mut processes = Vec::new();

        let mut rng = rand::rng();

for pid in 1..=count {
    let burst = rng.random_range(1..=15);

    processes.push(Process::new(pid as u32, burst));
}

        Self {
            processes,
            time: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }
    pub fn processes(&self,index:usize) -> &Process{
        &self.processes[index]
    }
    pub fn processes_mut(&mut self,index:usize) -> &mut Process {
        &mut self.processes[index]
    }
    pub fn time(&self) -> u32 {
        self.time
    }
    pub fn advance_time(&mut self, amount: u32) {
        self.time += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn scheduler_creates_requested_number_of_processes() {
//         let scheduler = Scheduler::new(5);

//         assert_eq!(scheduler.len(), 5);
//     }
//     #[test]
// fn scheduler_assigns_sequential_pids() {
//     let scheduler = Scheduler::new(3);

//     assert_eq!(scheduler.processes()[0].pid(), 1);
//     assert_eq!(scheduler.processes()[1].pid(), 2);
//     assert_eq!(scheduler.processes()[2].pid(), 3);
// }
// #[test]
// fn process_has_positive_burst_time() {
//     let scheduler = Scheduler::new(5);

//     for process in scheduler.processes() {
//         assert!(process.burst_time() >= 1);
//     }
// }
}