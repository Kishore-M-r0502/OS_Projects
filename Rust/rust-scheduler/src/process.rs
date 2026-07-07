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

}