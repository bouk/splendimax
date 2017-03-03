use std::cmp;
use algo::state::{State, Score};

pub fn alphabeta<S: State>(mut state: &mut S) -> Vec<S::Move> {
    let possible_moves = state.generate_moves();
    let mut best_moves: Vec<S::Move> = Vec::with_capacity(5);
    let mut best_score = S::Score::min_value();

    for mov in possible_moves.into_iter() {
        state.apply(&mov);
        let score = min::<S>(&mut state, 5, best_score, S::Score::max_value());

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

fn min<S: State>(mut state: &mut S, depth: usize, alpha: S::Score, mut beta: S::Score) -> S::Score {
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
        let score = max::<S>(&mut state, depth - 1, alpha, beta);
        state.undo(mov);

        worst_score = cmp::min(worst_score, score);
        beta = cmp::min(beta, score);

        if beta <= alpha {
            break;
        }
    }
    
    worst_score
}

fn max<S: State>(mut state: &mut S, depth: usize, mut alpha: S::Score, beta: S::Score) -> S::Score {
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
        let score = min::<S>(&mut state, depth - 1, alpha, beta);
        state.undo(mov);

        best_score = cmp::max(best_score, score);
        alpha = cmp::max(alpha, score);

        if beta <= alpha {
            break;
        }
    }

    best_score
}
