use crate::state::State;

pub struct Process {
    pid: u32,
    state: State,
    burst_time: u32,
    arrival_time: u32,
    start_time: Option<u32>,
    completion_time: Option<u32>,
}

impl Process {
    pub fn new(pid: u32, burst_time: u32) -> Self {
    Self {
        pid,
        state: State::Ready,
        burst_time,
        arrival_time:0,
        start_time:None,
        completion_time:None,
    }
}
    pub fn pid(&self) -> u32 {
        self.pid
    }
    pub fn state(&self) -> State {
        self.state
    }
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    pub fn burst_time(&self) -> u32 {
    self.burst_time
}

pub fn start(&mut self, current_time: u32) {
    self.state = State::Running;

    if self.start_time.is_none() {
        self.start_time = Some(current_time);
    }
}

    pub fn finish(&mut self, current_time: u32) {
        self.state = State::Done;
        self.completion_time = Some(current_time);
    }
    pub fn turnaround_time(&self) -> Option<u32>{
        self.completion_time
        .map(|completion| completion - self.arrival_time)
    }
    pub fn waiting_time(&self) -> Option<u32>{
        self.turnaround_time().map(|turnaround|turnaround-self.burst_time())
    }
    pub fn response_time(&self) -> Option<u32> {
    self.start_time
        .map(|start| start - self.arrival_time)
}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_process_is_ready() {
        let p = Process::new(1,1);

        assert_eq!(p.state, State::Ready);
    }
    #[test]
fn process_changes_state() {
    let mut process = Process::new(1,1);

    process.set_state(State::Running);

    assert_eq!(process.state(), State::Running);
}
#[test]
fn process_can_finish() {
    let mut process = Process::new(1,1);

    process.set_state(State::Running);
    process.set_state(State::Done);

    assert_eq!(process.state(), State::Done);
}
#[test]
fn process_starts() {
    let mut p = Process::new(1, 5);

    p.start(0);

    assert_eq!(p.state(), State::Running);
    assert_eq!(p.start_time, Some(0));
}

#[test]
fn process_finishes() {
    let mut p = Process::new(1, 5);

    p.start(0);
    p.finish(5);

    assert_eq!(p.state(), State::Done);
    assert_eq!(p.completion_time, Some(5));
}
#[test]
fn turnaround_time_is_computed() {
    let mut p = Process::new(1, 5);

    p.start(3);
    p.finish(8);

    assert_eq!(p.turnaround_time(), Some(8));
}
#[test]
fn waiting_time_is_computed() {
    let mut p = Process::new(1, 5);

    p.start(3);
    p.finish(8);

    assert_eq!(p.waiting_time(), Some(3));
}
#[test]
fn response_time_is_computed() {
    let mut p = Process::new(1, 5);

    p.start(3);

    assert_eq!(p.response_time(), Some(3));
}
#[test]
fn turnaround_time_is_none_before_completion() {
    let p = Process::new(1, 5);

    assert_eq!(p.turnaround_time(), None);
}
#[test]
fn waiting_time_is_none_before_completion() {
    let p = Process::new(1, 5);

    assert_eq!(p.waiting_time(), None);
}
#[test]
fn response_time_is_none_before_start() {
    let p = Process::new(1, 5);

    assert_eq!(p.response_time(), None);
}
#[test]
fn process_records_first_start_time_only() {
    let mut p = Process::new(1, 5);

    p.start(3);
    p.start(8);

    assert_eq!(p.start_time, Some(3));
}
}