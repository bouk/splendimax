use card::Card;
use color::Color;
use cost::Tokens;
use std::cmp::min;
use noble::Noble;
use algo;
use rand::{thread_rng, Rng};
use std::io;
use iter::CopyingIterator;

pub const MINIMUM_TO_TAKE_2_TOKENS: u8 = 4;
pub const NOBLE_SCORE: u8 = 3;
pub const MAXIMUM_RESERVED: usize = 3;
pub const MAXIMUM_COINS: u8 = 10;
pub const SCORE_TO_WIN: u8 = 15;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum Deck {
    One,
    Two,
    Three,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum Move {
    Take { tokens: Tokens, drop: Tokens },
    Reserve { card: Card, deck: Deck, drop: Tokens, joker: bool },
    Buy { card: Card, deck: Deck, cost: Tokens },
    BuyReserved { card: Card, cost: Tokens },
    Pass,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct State {
    pub deck1: Vec<Card>,
    pub cards1: Vec<Card>,

    pub deck2: Vec<Card>,
    pub cards2: Vec<Card>,

    pub deck3: Vec<Card>,
    pub cards3: Vec<Card>,

    pub bank: Tokens,
    pub player: Player,
    pub adversary: Player,

    pub nobles: Vec<Noble>,

    // true if player's turn, false if adversary's turn
    pub players_turn: bool,
}

pub type Score = i64;

impl State {
    pub fn new(players: u8) -> State {
        if players != 2 {
            panic!("only 2 players")
        }
        let mut rng = thread_rng();

        let mut deck1 = Card::deck1();
        rng.shuffle(&mut deck1);
        let new_deck1_len = deck1.len() - 4;
        let cards1 = deck1.drain(new_deck1_len..).collect();

        let mut deck2 = Card::deck2();
        rng.shuffle(&mut deck2);
        let new_deck2_len = deck2.len() - 4;
        let cards2 = deck2.drain(new_deck2_len..).collect();

        let mut deck3 = Card::deck3();
        rng.shuffle(&mut deck3);
        let new_deck3_len = deck3.len() - 4;
        let cards3 = deck3.drain(new_deck3_len..).collect();

        let mut nobles = Noble::all();
        rng.shuffle(&mut nobles);
        nobles.truncate(3);

        State {
            deck1: deck1,
            cards1: cards1,
            deck2: deck2,
            cards2: cards2,
            deck3: deck3,
            cards3: cards3,

            bank: Tokens::start(players),
            nobles: nobles,
            player: Player::new(),
            adversary: Player::new(),
            players_turn: true,
        }
    }

    pub fn print(&self, out: &mut io::Write) -> io::Result<()> {
        try!(write!(out, "Player: {}\n", self.adversary.score()));
        fn print_cards(out: &mut io::Write, cards: &Vec<Card>) -> io::Result<()> {
            if !cards.is_empty() {
                for _ in cards.iter() {
                    try!(write!(out, "┏━━━━━━━┓ "));
                }
                try!(write!(out, "\n"));
                for card in cards.iter() {
                    try!(write!(out, "┃{}     {}┃ ", card.color.code(), card.points));
                }
                try!(write!(out, "\n"));
                for _ in cards.iter() {
                    try!(write!(out, "┃       ┃ "));
                }
                try!(write!(out, "\n"));
                for card in cards.iter() {
                    try!(write!(out, "┃"));
                    let mut count = 0;
                    for color in Color::all_except_joker() {
                        if card.cost[color] > 0 {
                            try!(write!(out, "{}", color.code()));
                            count += 1;
                            if count < 4 {
                                try!(write!(out, " "));
                            }
                        }
                    }
                    if count < 4 {
                        try!(write!(out, " "));
                        if count < 3 {
                            try!(write!(out, "  "));
                            if count < 2 {
                                try!(write!(out, "  "));
                            }
                        }
                    }
                    try!(write!(out, "┃ "));
                }
                try!(write!(out, "\n"));
                for card in cards.iter() {
                    try!(write!(out, "┃"));
                    let mut count = 0;
                    for color in Color::all_except_joker() {
                        if card.cost[color] > 0 {
                            try!(write!(out, "{}", card.cost[color]));
                            count += 1;
                            if count < 4 {
                                try!(write!(out, " "));
                            }
                        }
                    }
                    if count < 4 {
                        try!(write!(out, " "));
                        if count < 3 {
                            try!(write!(out, "  "));
                            if count < 2 {
                                try!(write!(out, "  "));
                            }
                        }
                    }
                    try!(write!(out, "┃ "));
                }
                try!(write!(out, "\n"));
                for _ in cards.iter() {
                    try!(write!(out, "┗━━━━━━━┛ "));
                }
                try!(write!(out, "\n"));
            }
            Ok(())
        }
        fn print_player(out: &mut io::Write, player: &Player) -> io::Result<()> {
            if !player.reserved.is_empty() {
                try!(write!(out, "Reserved\n"));
                try!(print_cards(out, &player.reserved));
            }
            for color in Color::all() {
                try!(write!(out, "{}: {}", color.code(), player.tokens[color]));
                let count = player.cards.iter().filter(|ref card| card.color == color).count();
                if count > 0 {
                    try!(write!(out, " + {}", count));
                }
                try!(write!(out, "\n"));
            }
            Ok(())
        }
        try!(print_player(out, &self.adversary));
        try!(write!(out, "\n"));

        try!(write!(out, "Adversary: {}\n", self.player.score()));
        try!(print_player(out, &self.player));

        try!(write!(out, "\nBank\n"));
        for color in Color::all() {
            try!(write!(out, "{} ", color.code()));
        }
        try!(write!(out, "\n"));
        for color in Color::all() {
            try!(write!(out, "{} ", self.bank[color]));
        }
        try!(write!(out, "\n\n"));

        if !self.nobles.is_empty() {
            try!(write!(out, "\nNobles\n"));
            for _ in self.nobles.iter() {
                try!(write!(out, "┏━━━━━┓ "));
            }
            try!(write!(out, "\n"));
            for noble in self.nobles.iter() {
                try!(write!(out, "┃"));
                let mut count = 0;
                for color in Color::all_except_joker() {
                    if noble.cost[color] > 0 {
                        try!(write!(out, "{}", color.code()));
                        count += 1;
                        if count < 3 {
                            try!(write!(out, " "));
                        }
                    }
                }
                if count < 3 {
                    try!(write!(out, " "));
                }
                try!(write!(out, "┃ "));
            }
            try!(write!(out, "\n"));
            for noble in self.nobles.iter() {
                try!(write!(out, "┃"));
                let mut count = 0;
                for color in Color::all_except_joker() {
                    if noble.cost[color] > 0 {
                        try!(write!(out, "{}", noble.cost[color]));
                        count += 1;
                        if count < 3 {
                            try!(write!(out, " "));
                        }
                    }
                }
                if count < 3 {
                    try!(write!(out, " "));
                }
                try!(write!(out, "┃ "));
            }
            try!(write!(out, "\n"));
            for _ in self.nobles.iter() {
                try!(write!(out, "┗━━━━━┛ "));
            }
            try!(write!(out, "\n"));
        }
        try!(write!(out, "\n"));
        try!(print_cards(out, &self.cards3));
        try!(print_cards(out, &self.cards2));
        try!(print_cards(out, &self.cards1));
        Ok(())
    }
}

impl algo::State for State {
    type Score = Score;
    type Move = Move;

    fn score(&self) -> Score {
        let card_multiplier = Tokens::empty(); //self.nobles.iter().fold(Tokens::empty(), |acc, ref noble| acc + noble.cost);
        let player_score = self.player.score();
        let adversary_score = self.adversary.score();
        let mut score = (player_score as Score - adversary_score as Score) * 2000;

        if player_score >= SCORE_TO_WIN {
            if player_score < adversary_score {
                score -= 10000;
            } else {
                score += 10000;
            }
        } else if adversary_score >= SCORE_TO_WIN {
            score -= 10000;
        }

        score += self.player.card_score(&card_multiplier);
        score -= self.adversary.card_score(&card_multiplier);

        score += self.player.token_score();
        score -= self.adversary.token_score();

        score -= self.player.reserved.len() as Score;
        score += self.adversary.reserved.len() as Score;

        score
    }

    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let player: &Player = if self.players_turn {
            &self.player
        } else {
            &self.adversary
        };

        // Do most benificial moves first to get benefits of α β pruning

        for card in self.cards1.iter() {
            // TODO(bouk): check for noble
            if let Some(cost) = player.cost_for(card) {
                moves.push(Move::Buy {
                    card: *card,
                    deck: Deck::One,
                    cost: cost,
                });
            }
        }

        for card in self.cards2.iter() {
            // TODO(bouk): check for noble
            if let Some(cost) = player.cost_for(card) {
                moves.push(Move::Buy {
                    card: *card,
                    deck: Deck::Two,
                    cost: cost,
                });
            }
        }

        for card in self.cards3.iter() {
            // TODO(bouk): check for noble
            if let Some(cost) = player.cost_for(card) {
                moves.push(Move::Buy {
                    card: *card,
                    deck: Deck::Three,
                    cost: cost,
                });
            }
        }

        for card in player.reserved.iter() {
            // TODO(bouk): check for noble
            if let Some(cost) = player.cost_for(card) {
                moves.push(Move::BuyReserved {
                    card: *card,
                    cost: cost,
                });
            }
        }

        // The index is how many tokens need to be discarded
        let discard_options = player.tokens.discard_permutations();
        let total = player.tokens.total();

        for (color1, iter2) in Color::all_except_joker().copying() {
            if self.bank[color1] < 1 {
                continue
            }

            let mut any = false;
            if self.bank[color1] >= MINIMUM_TO_TAKE_2_TOKENS {
                any = true;

                let mut tokens = Tokens::empty();
                tokens[color1] = 2;

                let discard = (total + 2).saturating_sub(MAXIMUM_COINS);
                for drop in discard_options[discard as usize].iter() {
                    moves.push(Move::Take { tokens: tokens, drop: *drop });
                }
            }

            for (color2, iter3) in iter2.copying() {
                if self.bank[color2] < 1 {
                    continue
                }

                any = true;

                let mut any2 = false;
                for color3 in iter3 {
                    if self.bank[color3] < 1 {
                        continue
                    }
                    any2 = true;

                    let mut tokens = Tokens::empty();
                    tokens[color1] = 1;
                    tokens[color2] = 1;
                    tokens[color3] = 1;

                    let discard = (total + 3).saturating_sub(MAXIMUM_COINS);
                    for drop in discard_options[discard as usize].iter() {
                        moves.push(Move::Take { tokens: tokens, drop: *drop });
                    }
                }

                if !any2 {
                    let mut tokens = Tokens::empty();
                    tokens[color1] = 1;
                    tokens[color2] = 1;

                    let discard = (total + 2).saturating_sub(MAXIMUM_COINS);
                    for drop in discard_options[discard as usize].iter() {
                        moves.push(Move::Take { tokens: tokens, drop: *drop });
                    }
                }
            }

            if !any {
                let tokens = Tokens::one(color1);
                let discard = (total + 1).saturating_sub(MAXIMUM_COINS);
                for drop in discard_options[discard as usize].iter() {
                    moves.push(Move::Take { tokens: tokens, drop: *drop });
                }
            }
        }

        if player.reserved.len() < MAXIMUM_RESERVED {
            // Can I get a joker?
            let joker = self.bank.joker > 0;

            // Need to discard 1 coin if we're at the limit
            let drop_possibilities: &Vec<Tokens> = if joker && total == MAXIMUM_COINS {
                &discard_options[1]
            } else {
                &discard_options[0]
            };

            for card in self.cards1.iter() {
                for drop in drop_possibilities.iter() {
                    moves.push(Move::Reserve { 
                        card: *card,
                        deck: Deck::One,
                        joker: joker,
                        drop: *drop,
                    });
                }
            }

            for card in self.cards2.iter() {
                for drop in drop_possibilities.iter() {
                    moves.push(Move::Reserve { 
                        card: *card,
                        deck: Deck::Two,
                        joker: joker,
                        drop: *drop,
                    });
                }
            }

            for card in self.cards3.iter() {
                for drop in drop_possibilities.iter() {
                    moves.push(Move::Reserve { 
                        card: *card,
                        deck: Deck::Three,
                        joker: joker,
                        drop: *drop,
                    });
                }
            }
        }

        if moves.len() == 0 {
            moves.push(Move::Pass);
        }

        moves
    }

    fn apply(&mut self, mov: &Move) {
        match *mov {
            Move::Take { tokens, drop } => {
                let player = if self.players_turn {
                    &mut self.player
                } else {
                    &mut self.adversary
                };
                player.tokens += tokens;
                player.tokens -= drop;
                self.bank += drop;
                self.bank -= tokens;
            },
            Move::Reserve { ref card, deck, joker, drop } => {
                {
                    let player = if self.players_turn {
                        &mut self.player
                    } else {
                        &mut self.adversary
                    };
                    player.reserved.push(*card);
                    if joker {
                        player.tokens.joker += 1;
                        self.bank.joker -= 1;
                    }
                    player.tokens -= drop;
                }

                {
                    let cards = match deck {
                        Deck::One => &mut self.cards1,
                        Deck::Two => &mut self.cards2,
                        Deck::Three => &mut self.cards3,
                    };
                    if let Some(index) = cards.iter().position(|e| e == card) {
                        cards.remove(index);
                    } else {
                        panic!("Failed to reserve {:?}", card);
                    }
                }
            },
            Move::Buy { ref card, deck, cost } => {
                {
                    let player = if self.players_turn {
                        &mut self.player
                    } else {
                        &mut self.adversary
                    };
                    player.tokens -= cost;
                    player.cards.push(*card);
                    self.bank += cost;
                }

                {
                    let cards = match deck {
                        Deck::One => &mut self.cards1,
                        Deck::Two => &mut self.cards2,
                        Deck::Three => &mut self.cards3,
                    };
                    if let Some(index) = cards.iter().position(|e| e == card) {
                        cards.remove(index);
                    } else {
                        panic!("Failed to buy {:?}", card);
                    }
                }
            },
            Move::BuyReserved { ref card, cost } => {
                let player = if self.players_turn {
                    &mut self.player
                } else {
                    &mut self.adversary
                };

                player.tokens -= cost;
                player.cards.push(*card);
                self.bank += cost;

                let index = player.reserved.iter().position(|e| e == card).unwrap();
                player.reserved.remove(index);
            },
            Move::Pass => {},
        }
        self.players_turn = !self.players_turn;
    }

    fn undo(&mut self, mov: &Move) {
        self.players_turn = !self.players_turn;
        match *mov {
            Move::Take { tokens, drop } => {
                let player = if self.players_turn {
                    &mut self.player
                } else {
                    &mut self.adversary
                };
                self.bank += tokens;
                self.bank -= drop;
                player.tokens += drop;
                player.tokens -= tokens;
            },
            Move::Reserve { ref card, deck, joker, drop } => {
                {
                    let player = if self.players_turn {
                        &mut self.player
                    } else {
                        &mut self.adversary
                    };
                    let index = player.reserved.iter().position(|e| e == card).unwrap();
                    player.reserved.remove(index);
                    player.tokens += drop;
                    if joker {
                        player.tokens.joker -= 1;
                        self.bank.joker += 1;
                    }
                }

                {
                    let cards = match deck {
                        Deck::One => &mut self.cards1,
                        Deck::Two => &mut self.cards2,
                        Deck::Three => &mut self.cards3,
                    };
                    cards.push(*card);
                }
            },
            Move::Buy { ref card, deck, cost } => {
                {
                    let player = if self.players_turn {
                        &mut self.player
                    } else {
                        &mut self.adversary
                    };
                    let index = player.cards.iter().position(|e| e == card).unwrap();
                    player.cards.remove(index);
                    player.tokens += cost;
                    self.bank -= cost;
                }

                {
                    let cards = match deck {
                        Deck::One => &mut self.cards1,
                        Deck::Two => &mut self.cards2,
                        Deck::Three => &mut self.cards3,
                    };
                    cards.push(*card);
                }
            },
            Move::BuyReserved { ref card, cost } => {
                let player = if self.players_turn {
                    &mut self.player
                } else {
                    &mut self.adversary
                };

                player.tokens += cost;
                let index = player.cards.iter().position(|e| e == card).unwrap();
                player.cards.remove(index);
                self.bank -= cost;

                player.reserved.push(*card);
            },
            Move::Pass => {},
        }
    }

    fn is_terminal(&self) -> bool {
        self.player.score() >= SCORE_TO_WIN || self.adversary.score() >= SCORE_TO_WIN
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Player {
    pub tokens: Tokens,
    pub cards: Vec<Card>,
    pub reserved: Vec<Card>,
    pub nobles: Vec<Noble>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            tokens: Tokens::empty(),
            cards: Vec::with_capacity(16),
            reserved: Vec::with_capacity(MAXIMUM_RESERVED),
            nobles: Vec::new(),
        }
    }

    pub fn tokens_from_cards(&self) -> Tokens {
        let mut tokens = Tokens::empty();
        for card in self.cards.iter() {
            tokens[card.color] += 1
        }
        tokens
    }

    pub fn card_score(&self, card_multiplier: &Tokens) -> Score {
        let mut points = 0;
        // Give a bonus to multiple of the same card
        let mut multiple_card_bonus = Tokens {
            black: 1,
            blue: 1,
            green: 1,
            red: 1,
            white: 1,
            joker: 0,
        };

        for card in self.cards.iter() {
            points += (card_multiplier[card.color] as Score) * 20 + (multiple_card_bonus[card.color] as Score) * 30 + 50;
            if multiple_card_bonus[card.color] < 5 {
                multiple_card_bonus[card.color] += 1;
            }
        }

        points
    }

    pub fn token_score(&self) -> Score {
        // Give jokers 50% more value than other tokens
        ((min(MAXIMUM_COINS, self.tokens.total()) * 2 + self.tokens.joker) as Score) * 4
    }

    pub fn score(&self) -> u8 {
        self.cards.iter().fold(0, |acc, &card| acc + card.points) +
            self.nobles.len() as u8 * 3
    }

    pub fn can_buy(&self, card: &Card) -> bool {
        self.cost_for(card).is_some()
    }

    // Assumes you can pay for it.
    pub fn cost_for(&self, card: &Card) -> Option<Tokens> {
        let tokens_from_cards = self.tokens_from_cards();
        let total_tokens = self.tokens + tokens_from_cards;
        let mut cost = Tokens::empty();

        for color in Color::all_except_joker() {
            if total_tokens[color] < card.cost[color] {
                let difference = card.cost[color] - total_tokens[color];
                cost.joker += difference;
                if cost.joker > self.tokens.joker {
                    return None;
                }
                cost[color] = card.cost[color] - tokens_from_cards[color] - difference;
            } else {
                cost[color] = card.cost[color].saturating_sub(tokens_from_cards[color]);
            }
        }

        Some(cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algo::State;

    #[test]
    fn generate_possible_moves() {
        let state = super::State::new(2);
        let moves = state.generate_moves();
        println!("{:?}", moves);
        assert!(false);
    }

    #[test]
    fn can_buy() {
        let player = Player {
            tokens: Tokens {
                black: 0,
                blue: 0,
                green: 1,
                red: 0,
                white: 0,
                joker: 0,
            },
            cards: vec![
                Card {
                    color: Color::Blue,
                    cost: Tokens::empty(),
                    points: 0,
                }
            ],
            reserved: Vec::new(),
            nobles: Vec::new(),
        };
        assert!(player.can_buy(&Card {
            color: Color::Black,
            cost: Tokens {
                black: 0,
                blue: 1,
                green: 1,
                red: 0,
                white: 0,
                joker: 0,
            },
            points: 0,
        }));
        assert!(player.can_buy(&Card {
            color: Color::Black,
            cost: Tokens {
                black: 0,
                blue: 1,
                green: 0,
                red: 0,
                white: 0,
                joker: 0,
            },
            points: 0,
        }));
        assert!(!player.can_buy(&Card {
            color: Color::Black,
            cost: Tokens {
                black: 0,
                blue: 1,
                green: 0,
                red: 1,
                white: 0,
                joker: 0,
            },
            points: 0,
        }));
    }

    #[test]
    fn cost_for() {
        let player = Player {
            tokens: Tokens {
                black: 0,
                blue: 0,
                green: 1,
                red: 0,
                white: 0,
                joker: 1,
            },
            cards: vec![
                Card {
                    color: Color::Blue,
                    cost: Tokens::empty(),
                    points: 0,
                }
            ],
            reserved: Vec::new(),
            nobles: Vec::new(),
        };
        assert_eq!(player.cost_for(&Card {
            color: Color::Black,
            cost: Tokens {
                black: 0,
                blue: 0,
                green: 1,
                red: 0,
                white: 0,
                joker: 0,
            },
            points: 0,
        }).unwrap(), Tokens {
            black: 0,
            blue: 0,
            green: 1,
            red: 0,
            white: 0,
            joker: 0,
        });
        assert_eq!(player.cost_for(&Card {
            color: Color::Black,
            cost: Tokens {
                black: 0,
                blue: 1,
                green: 1,
                red: 0,
                white: 0,
                joker: 0,
            },
            points: 0,
        }).unwrap(), Tokens {
            black: 0,
            blue: 0,
            green: 1,
            red: 0,
            white: 0,
            joker: 0,
        });
        assert_eq!(player.cost_for(&Card {
            color: Color::Black,
            cost: Tokens {
                black: 0,
                blue: 2,
                green: 1,
                red: 0,
                white: 0,
                joker: 0,
            },
            points: 0,
        }).unwrap(), Tokens {
            black: 0,
            blue: 0,
            green: 1,
            red: 0,
            white: 0,
            joker: 1,
        });
        assert_eq!(player.cost_for(&Card {
            color: Color::Black,
            cost: Tokens {
                black: 0,
                blue: 1,
                green: 1,
                red: 1,
                white: 0,
                joker: 0,
            },
            points: 0,
        }).unwrap(), Tokens {
            black: 0,
            blue: 0,
            green: 1,
            red: 0,
            white: 0,
            joker: 1,
        });
    }
}
