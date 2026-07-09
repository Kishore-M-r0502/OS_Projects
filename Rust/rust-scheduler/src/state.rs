#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ready,
    Running,
    Waiting,
    Done,
}
use std::fmt;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Ready => write!(f, "Ready"),
            State::Running => write!(f, "Running"),
            State::Waiting => write!(f, "Waiting"),
            State::Done => write!(f, "Done"),
        }
    }
}