use std::fmt;

pub trait Score: Copy + Ord + fmt::Display {
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn neg(&self) -> Self;
}

impl Score for i64 {
    fn min_value() -> Self {
        Self::min_value()
    }

    fn max_value() -> Self {
        Self::max_value()
    }

    fn neg(&self) -> Self {
        -self
    }
}

pub trait State {
    type Score: Score;
    type Move: Copy + fmt::Debug;

    fn score(&self) -> Self::Score;
    fn generate_moves(&self) -> Vec<Self::Move>;
    fn is_terminal(&self) -> bool;
    fn apply(&mut self, &Self::Move);
    fn undo(&mut self, &Self::Move);
}
