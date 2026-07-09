// use crate::scheduler::Scheduler;

// mod state;
// mod process;
// mod algorithm;
// mod scheduler;
// mod tui;
// fn main() {
//     println!("Operating System Scheduling in Rust");
//     algorithm::round_robin::run(&mut scheduler::Scheduler::new(10), 40);
// }
use std::io;

use crossterm::event;

mod algorithm;
mod process;
mod scheduler;
mod state;
mod tui;

use tui::{
    app::App,
    terminal,
    ui,
};

fn main() -> io::Result<()> {
    // Initialize the terminal
    let mut terminal = terminal::init()?;

    // Create the application
    let app = App::new();

    // Draw the dashboard once
    terminal.draw(|frame| {
        ui::draw(frame, &app);
    })?;

    // Wait for any key press
    let _ = event::read()?;

    // Restore the terminal
    terminal::restore(&mut terminal)?;

    Ok(())
}