use std::collections::VecDeque;
use crate::scheduler::Scheduler;

pub fn run(scheduler: &mut Scheduler, quantum: u32) {
    let mut queue = VecDeque::new();

    // Initialize the ready queue.
    for i in 0..scheduler.len() {
        queue.push_back(i);
    }

    // Run until no process remains.
    while let Some(index) = queue.pop_front() {

        // ---- Execute one time slice ----
        let elapsed = {
            let current_time = scheduler.time();

let process = scheduler.processes_mut(index);

process.start(current_time);
            let remaining = process.remaining_time();
            let elapsed = remaining.min(quantum);

            process.run_for(quantum);

            elapsed
        };

        // Advance simulation time.
        scheduler.advance_time(elapsed);

        // ---- Decide what to do next ----
        let finished = {
            let process = scheduler.processes_mut(index);

            if process.is_finished() {
                let current_time = scheduler.time();

let process = scheduler.processes_mut(index);

process.finish(current_time);
                true
            } else {
                false
            }
        };

        if !finished {
            queue.push_back(index);
        }
    }
}

#[test]
fn round_robin_completes_all_processes() {
    use crate::state::State;
    let mut scheduler = Scheduler::new(5);

    run(&mut scheduler, 2);

    for i in 0..scheduler.len() {
        assert_eq!(scheduler.processes_mut(i).state(), State::Done);
    }
}
    #[test]
fn process_requires_multiple_quanta() {
    use crate::process::Process;
    let mut p = Process::new(1, 7);

    p.run_for(3);
    assert_eq!(p.remaining_time(), 4);

    p.run_for(3);
    assert_eq!(p.remaining_time(), 1);

    p.run_for(3);
    assert_eq!(p.remaining_time(), 0);

    assert!(p.is_finished());
}
#[test]
fn round_robin_leaves_no_process_unfinished() {
    let mut scheduler = Scheduler::new(10);

    run(&mut scheduler, 2);

    for i in 0..scheduler.len() {
        assert!(scheduler.processes(i).is_finished());
    }
    assert!(scheduler.time() > 0);
    for i in 0..scheduler.len() {
    let p = scheduler.processes(i);

    assert!(p.completion_time().is_some());
}
}