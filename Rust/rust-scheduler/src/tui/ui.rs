use ratatui::{
    layout::{Constraint, Layout,Rect},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame
};

use crate::tui::app::App;

// fn draw_header(...)

// fn draw_scheduler(...)

fn draw_processes(frame: &mut Frame, area: Rect, app: &App) {
    let rows = app.scheduler.iter_processes().map(|process| {
        Row::new(vec![
            Cell::from(process.pid().to_string()),
            Cell::from(process.state().to_string()),
            Cell::from(process.burst_time().to_string()),
            Cell::from(process.remaining_time().to_string()),
            Cell::from(display_option(process.waiting_time())),
            Cell::from(display_option(process.turnaround_time())),
            Cell::from(display_option(process.response_time())),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),   // PID
            Constraint::Length(12),  // State
            Constraint::Length(8),   // Burst
            Constraint::Length(12),  // Remaining
            Constraint::Length(10),  // Waiting
            Constraint::Length(12),  // Turnaround
            Constraint::Length(10),  // Response
        ],
    )
    .header(Row::new([
        "PID",
        "State",
        "Burst",
        "Remaining",
        "Waiting",
        "Turnaround",
        "Response",
    ]))
    .block(
        Block::default()
            .title("Processes")
            .borders(Borders::ALL),
    );

    frame.render_widget(table, area);
}
 fn draw_metrics(){
    !unimplemented!()
 }

// fn draw_footer(...)
fn display_option<T: std::fmt::Display>(value: Option<T>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "-".to_string(),
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = Layout::vertical([
        Constraint::Length(3), // Header
        Constraint::Length(5), // Scheduler
        Constraint::Min(8),    // Processes
        Constraint::Length(5), // Metrics
        Constraint::Length(3), // Footer
    ])
    .split(frame.area());

    // draw_header(frame, areas[0], app);
    // draw_scheduler(frame, areas[1], app);
    draw_processes(frame, areas[2], app);
    // draw_metrics(frame, areas[3], app);
    // draw_footer(frame, areas[4], app);
}
