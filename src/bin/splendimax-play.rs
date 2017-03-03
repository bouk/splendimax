extern crate splendimax;
extern crate rand;

use std::io;
use splendimax::algo::state::State as AlgoState;
use splendimax::algo::state::Score;
use splendimax::algo::alphabeta;
use splendimax::state::{Move, State, Deck};
use splendimax::card::Card;
use splendimax::cost::Tokens;
use splendimax::color::Color;
use rand::{thread_rng, Rng};

fn main() {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    let mut state = State::new(2);
    let mut rng = thread_rng();
    state.print(&mut stdout);
    loop {
        if state.is_terminal() {
            state.print(&mut stdout);
            break
        }

        if state.players_turn {
            let moves = alphabeta(&mut state);
            if let Some(mov) = rng.choose(&moves) {
                println!("{:?}", mov);
                state.apply(&mov);
            } else {
                state.print(&mut stdout);
                panic!("No moves");
            }
        } else {
            state.print(&mut stdout);
            'outer: loop {
                let mut buf = String::new();
                println!("Please specify action. (t)ake rkw, (b)uy 1 3 (row column), (r)eserve 1 3 (row column) b(u)y reserved 1 (index), (p)ass");
                let mov;
                match stdin.read_line(&mut buf) {
                    Ok(_) => {
                        let mut iter = buf.chars();
                        match iter.next() {
                            Some('t') => {
                                let mut tokens = Tokens::empty();
                                for c in iter {
                                    match c {
                                        'k' => tokens.black += 1,
                                        'b' => tokens.blue += 1,
                                        'g' => tokens.green += 1,
                                        'r' => tokens.red += 1,
                                        'w' => tokens.white += 1,
                                        _ => continue,
                                    }
                                }
                                mov = Move::Take { tokens: tokens, drop: Tokens::empty() };
                            },
                            Some('b') => {
                                let cards;
                                let deck;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            cards = &state.cards1;
                                            deck = Deck::One;
                                            break;
                                        },
                                        Some('2') => {
                                            cards = &state.cards2;
                                            deck = Deck::Two;
                                            break;
                                        },
                                        Some('3') => {
                                            cards = &state.cards3;
                                            deck = Deck::Three;
                                            break;
                                        },
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        },
                                    }
                                }

                                let card;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            card = cards.get(0);
                                            break;
                                        },
                                        Some('2') => {
                                            card = cards.get(1);
                                            break;
                                        },
                                        Some('3') => {
                                            card = cards.get(2);
                                            break;
                                        },
                                        Some('4') => {
                                            card = cards.get(3);
                                            break;
                                        },
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        },
                                    }
                                }

                                if let Some(card) = card {
                                    if let Some(cost) = state.adversary.cost_for(card) {
                                        mov = Move::Buy { card: *card, deck: deck, cost: cost };
                                    } else {
                                        println!("can't afford");
                                        break 'outer;
                                    }
                                } else {
                                    println!("invalid card");
                                    break 'outer;
                                }
                            },
                            Some('r') => {
                                let cards;
                                let deck;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            cards = &state.cards1;
                                            deck = Deck::One;
                                            break;
                                        },
                                        Some('2') => {
                                            cards = &state.cards2;
                                            deck = Deck::Two;
                                            break;
                                        },
                                        Some('3') => {
                                            cards = &state.cards3;
                                            deck = Deck::Three;
                                            break;
                                        },
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        },
                                    }
                                }

                                let card;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            card = cards.get(0);
                                            break;
                                        },
                                        Some('2') => {
                                            card = cards.get(1);
                                            break;
                                        },
                                        Some('3') => {
                                            card = cards.get(2);
                                            break;
                                        },
                                        Some('4') => {
                                            card = cards.get(3);
                                            break;
                                        },
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        },
                                    }
                                }

                                if let Some(card) = card {
                                    mov = Move::Reserve { card: *card, deck: deck, drop: Tokens::empty(), joker: state.bank.joker > 0 };
                                } else {
                                    println!("invalid card");
                                    break 'outer;
                                }
                            },
                            Some('u') => {
                                let card;
                                loop {
                                    let c = iter.next();
                                    match c {
                                        Some('1') => {
                                            card = state.adversary.reserved.get(0);
                                            break;
                                        },
                                        Some('2') => {
                                            card = state.adversary.reserved.get(1);
                                            break;
                                        },
                                        Some('3') => {
                                            card = state.adversary.reserved.get(2);
                                            break;
                                        },
                                        Some(_) => continue,
                                        None => {
                                            println!("invalid command");
                                            break 'outer;
                                        },
                                    }
                                }
                                if let Some(card) = card {
                                    if let Some(cost) = state.adversary.cost_for(card) {
                                        mov = Move::BuyReserved { card: *card, cost: cost };
                                    } else {
                                        println!("can't afford");
                                        break 'outer;
                                    }
                                } else {
                                    println!("invalid card");
                                    break 'outer;
                                }
                            },
                            Some('p') => {
                                mov = Move::Pass;
                            },
                            Some(_) | None => continue
                        }
                    },
                    Err(_) => {
                        panic!("couldn't read input");
                    },
                }

                let moves = state.generate_moves();
                if moves.iter().any(|m| m == &mov) {
                    println!("{:?}", &mov);
                    state.apply(&mov);
                    break;
                } else {
                    println!("Invalid move");
                }
            }
        }

        if state.cards1.len() < 4 {
            if let Some(card) = state.deck1.pop() {
                state.cards1.push(card);
            }
        }
        if state.cards2.len() < 4 {
            if let Some(card) = state.deck2.pop() {
                state.cards2.push(card);
            }
        }
        if state.cards3.len() < 4 {
            if let Some(card) = state.deck3.pop() {
                state.cards3.push(card);
            }
        }
        println!("");
    }
}
