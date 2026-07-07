use crate::state::State;

pub struct Process {
    pid: u32,
    state: State,
}

impl Process {
    pub fn new(pid: u32) -> Self {
        Self {
            pid,
            state: State::Ready,
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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_process_is_ready() {
        let p = Process::new(1);

        assert_eq!(p.state, State::Ready);
    }
    #[test]
fn process_changes_state() {
    let mut process = Process::new(1);

    process.set_state(State::Running);

    assert_eq!(process.state(), State::Running);
}
#[test]
fn process_can_finish() {
    let mut process = Process::new(1);

    process.set_state(State::Running);
    process.set_state(State::Done);

    assert_eq!(process.state(), State::Done);
}
}