use crate::scheduler::Scheduler;

pub fn run(scheduler: &mut Scheduler) {
    for i in 0..scheduler.len() {

        let current_time = scheduler.time();

        let burst = {
            let process = scheduler.processes_mut(i);

            process.start(current_time);

            process.burst_time()
        };
        scheduler.advance_time(burst);
        let current_time = scheduler.time();

{
    let process = scheduler.processes_mut(i);

    process.finish(current_time);
}

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;

    #[test]
    fn fcfs_completes_all_processes() {
        let mut scheduler = Scheduler::new(5);

        run(&mut scheduler);

        for i in 0..scheduler.len() {
            assert_eq!(scheduler.processes(i).state(), State::Done);
        }
    }
}