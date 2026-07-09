use rand::RngExt;
use crate::algorithm::fcfs::run;
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
    pub fn from_bursts(list:Vec<u32>)->Self{
        let mut bursts=Vec::new();
            for (pid,burst) in list.into_iter().enumerate(){
                let x =Process::new(pid as u32, burst);
                bursts.push(x);
            }
            Scheduler { processes: bursts, time: 0 }
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
    pub fn average_waiting_time(&self) -> f64 {
        let mut sum = 0u32;
        let mut count = 0u32;
        for process in &self.processes {
            if let Some(waiting) = process.waiting_time() {
                sum += waiting;
                count += 1;
}

}
if count == 0 {
    return 0.0;
}
sum as f64 / count as f64
}
pub fn average_turnaround_time(&self) -> f64{
    let mut sum = 0u32;
        let mut count = 0u32;
        for process in &self.processes {
            if let Some(turnaround) = process.turnaround_time() {
                sum += turnaround;
                count += 1;
}

}
if count == 0 {
    return 0.0;
}
sum as f64 / count as f64
}
pub fn average_response_time(&self) -> f64{
    let mut sum = 0u32;
        let mut count = 0u32;
        for process in &self.processes {
            if let Some(response) = process.response_time() {
                sum += response;
                count += 1;
}

}
if count == 0 {
    return 0.0;
}
sum as f64 / count as f64
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
#[test]
fn average_waiting_time_is_zero_for_new_scheduler() {
    let scheduler = Scheduler::new(5);

    assert_eq!(scheduler.average_waiting_time(), 0.0);
}
#[test]
fn average_waiting_time_is_computed_after_fcfs() {
    let mut scheduler = Scheduler::new(5);

    run(&mut scheduler);

    assert!(scheduler.average_waiting_time() >= 0.0);
}
#[test]
fn average_turnaround_time_is_zero_for_new_scheduler() {
    let scheduler = Scheduler::new(5);

    assert_eq!(scheduler.average_turnaround_time(), 0.0);
}
#[test]
fn average_turnaround_time_is_computed_after_fcfs() {
    let mut scheduler = Scheduler::new(5);

    run(&mut scheduler);

    assert!(scheduler.average_turnaround_time() >= 0.0);
}
#[test]
fn average_response_time_is_zero_for_new_scheduler() {
    let scheduler = Scheduler::new(5);

    assert_eq!(scheduler.average_response_time(), 0.0);
}
#[test]
fn average_response_time_is_computed_after_fcfs() {
    let mut scheduler = Scheduler::new(5);

    run(&mut scheduler);

    assert!(scheduler.average_response_time() >= 0.0);
}
#[test]
fn scheduler_from_bursts_creates_requested_number_of_processes() {
    let scheduler = Scheduler::from_bursts(vec![5, 3, 2]);

    assert_eq!(scheduler.len(), 3);
}
#[test]
fn scheduler_from_bursts_preserves_burst_times() {
    let scheduler = Scheduler::from_bursts(vec![5, 3, 2]);

    assert_eq!(scheduler.processes(0).burst_time(), 5);
    assert_eq!(scheduler.processes(1).burst_time(), 3);
    assert_eq!(scheduler.processes(2).burst_time(), 2);
}
#[test]
fn scheduler_from_bursts_initializes_remaining_time() {
    let scheduler = Scheduler::from_bursts(vec![5, 3, 2]);

    assert_eq!(scheduler.processes(0).remaining_time(), 5);
    assert_eq!(scheduler.processes(1).remaining_time(), 3);
    assert_eq!(scheduler.processes(2).remaining_time(), 2);
}
#[test]
fn scheduler_from_bursts_assigns_sequential_pids() {
    let scheduler = Scheduler::from_bursts(vec![5, 3, 2]);

    assert_eq!(scheduler.processes(0).pid(), 0);
    assert_eq!(scheduler.processes(1).pid(), 1);
    assert_eq!(scheduler.processes(2).pid(), 2);
}
#[test]
fn scheduler_from_bursts_starts_at_time_zero() {
    let scheduler = Scheduler::from_bursts(vec![5, 3, 2]);

    assert_eq!(scheduler.time(), 0);
}
}