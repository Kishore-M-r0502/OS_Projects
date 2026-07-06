use crate::state::State;

#[derive(Debug)]
pub struct Process {
    pub pid: u32,
    pub state: State,
}
use rand::{Rng, RngExt};

impl Process {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        Self {
            pid: rng.random_range(1..=1000),
            state: State::Ready,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_process_is_ready() {
        let p = Process::new();

        assert_eq!(p.state, State::Ready);
    }
}