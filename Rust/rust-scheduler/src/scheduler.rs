use crate::process::Process;

pub struct Scheduler {
    processes: Vec<Process>,
    time: u32,
}

impl Scheduler {
    pub fn new(count: usize) -> Self {
        let mut processes = Vec::new();

        for pid in 1..=count {
            processes.push(Process::new(pid as u32));
        }

        Self {
            processes,
            time: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.processes.len()
    }
    pub fn processes(&self) -> &[Process] {
        &self.processes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_creates_requested_number_of_processes() {
        let scheduler = Scheduler::new(5);

        assert_eq!(scheduler.len(), 5);
    }
    #[test]
fn scheduler_assigns_sequential_pids() {
    let scheduler = Scheduler::new(3);

    assert_eq!(scheduler.processes()[0].pid(), 1);
    assert_eq!(scheduler.processes()[1].pid(), 2);
    assert_eq!(scheduler.processes()[2].pid(), 3);
}
}