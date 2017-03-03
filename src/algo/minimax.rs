use std::cmp;
use algo::state::{State, Score};

pub fn minimax<S: State>(mut state: &mut S) -> Vec<S::Move> {
    let possible_moves = state.generate_moves();
    let mut best_moves: Vec<S::Move> = Vec::with_capacity(5);
    let mut best_score = S::Score::min_value();

    for mov in possible_moves.into_iter() {
        state.apply(&mov);
        let score = min::<S>(&mut state, 5);

        if score == best_score {
            best_moves.push(mov);
        } else if score > best_score {
            best_score = score;
            best_moves.clear();
            best_moves.push(mov);
        }
        state.undo(&mov);
    }

    best_moves
}

fn min<S: State>(mut state: &mut S, depth: usize) -> S::Score {
    if depth == 0 || state.is_terminal() {
        return state.score();
    }

    let possible_moves = state.generate_moves();
    if possible_moves.is_empty() {
        return state.score();
    }
    let mut worst_score = S::Score::max_value();

    for mov in possible_moves.iter() {
        state.apply(mov);
        let score = max::<S>(&mut state, depth - 1);
        state.undo(mov);

        worst_score = cmp::min(worst_score, score);
    }
    
    worst_score
}

fn max<S: State>(mut state: &mut S, depth: usize) -> S::Score {
    if depth == 0 || state.is_terminal() {
        return state.score();
    }

    let possible_moves = state.generate_moves();
    if possible_moves.is_empty() {
        return state.score();
    }
    let mut best_score = S::Score::min_value();

    for mov in possible_moves.iter() {
        state.apply(mov);
        let score = min::<S>(&mut state, depth - 1);
        state.undo(mov);

        best_score = cmp::max(best_score, score);
    }

    best_score
}
