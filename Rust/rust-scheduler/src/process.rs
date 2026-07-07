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
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_process_is_ready() {
        let p = Process::new(1);

        assert_eq!(p.state, State::Ready);
    }
}