use cost::Tokens;
use color::Color;

fn create_card(color: Color, points: u8, black: u8, blue: u8, green: u8, red: u8, white: u8) -> Card {
    Card {
        color: color,
        cost: Tokens {
            black: black,
            blue: blue,
            green: green,
            red: red,
            white: white,
            joker: 0,
        },
        points: points,
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Card {
    pub color: Color,
    pub cost: Tokens,
    pub points: u8,
}

impl Card {
    pub fn deck1() -> Vec<Card> {
        vec![
            create_card(Color::Black, 0, 0, 1, 1, 1, 1),
            create_card(Color::Black, 0, 0, 2, 1, 1, 1),
            create_card(Color::Black, 0, 0, 2, 0, 1, 2),
            create_card(Color::Black, 0, 1, 0, 1, 3, 0),
            create_card(Color::Black, 0, 0, 0, 2, 1, 0),
            create_card(Color::Black, 0, 0, 0, 2, 0, 2),
            create_card(Color::Black, 0, 0, 0, 3, 0, 0),
            create_card(Color::Black, 1, 0, 4, 0, 0, 0),
            create_card(Color::Blue, 0, 1, 0, 1, 1, 1),
            create_card(Color::Blue, 0, 1, 0, 1, 2, 1),
            create_card(Color::Blue, 0, 0, 0, 2, 2, 1),
            create_card(Color::Blue, 0, 0, 1, 3, 1, 0),
            create_card(Color::Blue, 0, 2, 0, 0, 0, 1),
            create_card(Color::Blue, 0, 2, 0, 2, 0, 0),
            create_card(Color::Blue, 0, 3, 0, 0, 0, 0),
            create_card(Color::Blue, 1, 0, 0, 0, 4, 0),
            create_card(Color::White, 0, 1, 1, 1, 1, 0),
            create_card(Color::White, 0, 1, 1, 2, 1, 0),
            create_card(Color::White, 0, 1, 2, 2, 0, 0),
            create_card(Color::White, 0, 1, 1, 0, 0, 3),
            create_card(Color::White, 0, 1, 0, 0, 2, 0),
            create_card(Color::White, 0, 2, 2, 0, 0, 0),
            create_card(Color::White, 0, 0, 3, 0, 0, 0),
            create_card(Color::White, 1, 0, 0, 4, 0, 0),
            create_card(Color::Green, 0, 1, 1, 0, 1, 1),
            create_card(Color::Green, 0, 2, 1, 0, 1, 1),
            create_card(Color::Green, 0, 2, 1, 0, 2, 0),
            create_card(Color::Green, 0, 0, 3, 1, 0, 1),
            create_card(Color::Green, 0, 0, 1, 0, 0, 2),
            create_card(Color::Green, 0, 0, 2, 0, 2, 0),
            create_card(Color::Green, 0, 0, 0, 0, 3, 0),
            create_card(Color::Green, 1, 4, 0, 0, 0, 0),
            create_card(Color::Red, 0, 1, 1, 1, 0, 1),
            create_card(Color::Red, 0, 1, 1, 1, 0, 2),
            create_card(Color::Red, 0, 2, 0, 1, 0, 2),
            create_card(Color::Red, 0, 3, 0, 0, 1, 1),
            create_card(Color::Red, 0, 0, 2, 1, 0, 0),
            create_card(Color::Red, 0, 0, 0, 0, 2, 2),
            create_card(Color::Red, 0, 0, 0, 0, 0, 3),
            create_card(Color::Red, 1, 0, 0, 0, 0, 4),
        ]
    }

    pub fn deck2() -> Vec<Card> {
        vec![
            create_card(Color::Black, 1, 0, 2, 2, 0, 3),
            create_card(Color::Black, 1, 2, 0, 3, 0, 3),
            create_card(Color::Black, 2, 0, 1, 4, 2, 0),
            create_card(Color::Black, 2, 0, 0, 5, 3, 0),
            create_card(Color::Black, 2, 0, 0, 0, 0, 5),
            create_card(Color::Black, 3, 6, 0, 0, 0, 0),
            create_card(Color::Blue, 1, 0, 2, 2, 3, 0),
            create_card(Color::Blue, 1, 3, 2, 3, 0, 0),
            create_card(Color::Blue, 2, 0, 3, 0, 0, 5),
            create_card(Color::Blue, 2, 4, 0, 0, 1, 2),
            create_card(Color::Blue, 2, 0, 5, 0, 0, 0),
            create_card(Color::Blue, 3, 0, 6, 0, 0, 0),
            create_card(Color::White, 1, 2, 0, 3, 2, 0),
            create_card(Color::White, 1, 0, 3, 0, 3, 2),
            create_card(Color::White, 2, 2, 0, 1, 4, 0),
            create_card(Color::White, 2, 3, 0, 0, 5, 0),
            create_card(Color::White, 2, 0, 0, 0, 5, 0),
            create_card(Color::White, 3, 0, 0, 0, 0, 6),
            create_card(Color::Green, 1, 0, 0, 2, 3, 3),
            create_card(Color::Green, 1, 2, 3, 0, 0, 2),
            create_card(Color::Green, 2, 1, 2, 0, 0, 4),
            create_card(Color::Green, 2, 0, 5, 3, 0, 0),
            create_card(Color::Green, 2, 0, 0, 5, 0, 0),
            create_card(Color::Green, 3, 0, 0, 6, 0, 0),
            create_card(Color::Red, 1, 3, 0, 0, 2, 2),
            create_card(Color::Red, 1, 3, 3, 0, 2, 0),
            create_card(Color::Red, 2, 0, 4, 2, 0, 1),
            create_card(Color::Red, 2, 5, 0, 0, 0, 3),
            create_card(Color::Red, 2, 5, 0, 0, 0, 0),
            create_card(Color::Red, 3, 0, 0, 0, 6, 0),
        ]
    }

    pub fn deck3() -> Vec<Card> {
        vec![
            create_card(Color::Black, 3, 0, 3, 5, 3, 3),
            create_card(Color::Black, 4, 0, 0, 0, 7, 0),
            create_card(Color::Black, 4, 3, 0, 3, 6, 0),
            create_card(Color::Black, 5, 3, 0, 0, 7, 0),
            create_card(Color::Blue, 3, 5, 0, 3, 3, 3),
            create_card(Color::Blue, 4, 0, 0, 0, 0, 7),
            create_card(Color::Blue, 4, 3, 3, 0, 0, 6),
            create_card(Color::Blue, 5, 0, 3, 0, 0, 7),
            create_card(Color::White, 3, 3, 3, 3, 5, 0),
            create_card(Color::White, 4, 7, 0, 0, 0, 0),
            create_card(Color::White, 4, 6, 0, 0, 3, 3),
            create_card(Color::White, 5, 7, 0, 0, 0, 3),
            create_card(Color::Green, 3, 3, 3, 0, 3, 5),
            create_card(Color::Green, 4, 0, 7, 0, 0, 0),
            create_card(Color::Green, 4, 0, 6, 3, 0, 3),
            create_card(Color::Green, 5, 0, 7, 3, 0, 0),
            create_card(Color::Red, 3, 3, 5, 3, 0, 3),
            create_card(Color::Red, 4, 0, 0, 7, 0, 0),
            create_card(Color::Red, 4, 0, 3, 6, 3, 0),
            create_card(Color::Red, 5, 0, 0, 7, 3, 0),
        ]
    }
}

