use crate::scheduler::Scheduler;

mod state;
mod process;
mod algorithm;
mod scheduler;
fn main() {
    println!("Operating System Scheduling in Rust");
    algorithm::round_robin::run(&mut scheduler::Scheduler::new(10), 40);
}
