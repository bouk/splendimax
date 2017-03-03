use std::ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign};
use color::Color;
use std::cmp::min;
use iter::CopyingIterator;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Tokens {
    pub black: u8,
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub white: u8,
    pub joker: u8,
}

impl Tokens {
    pub fn empty() -> Tokens {
        Tokens {
            black: 0,
            blue: 0,
            green: 0,
            red: 0,
            white: 0,
            joker: 0,
        }
    }

    pub fn start(players: u8) -> Tokens {
        if players != 2 {
            panic!("only 2 players")
        }

        Tokens {
            black: 4,
            blue: 4,
            green: 4,
            red: 4,
            white: 4,
            joker: 5,
        }
    }

    pub fn one(color: Color) -> Tokens {
        let mut tokens = Tokens::empty();
        tokens[color] = 1;
        tokens
    }

    pub fn total(&self) -> u8 {
        self.black + self.blue + self.green + self.red + self.white + self.joker
    }

    pub fn can_buy(&self, other: &Tokens) -> bool {
        let mut jokers_left = self.joker;
        for color in Color::all_except_joker() {
            if self[color] < other[color] {
                let difference = other[color] - self[color];
                match jokers_left.checked_sub(difference) {
                    None => return false,
                    Some(new_val) => jokers_left = new_val,
                }
            }
        }
        true
    }

    // Generate an array that contains all the possible permutations for discarding tokens, up to 3
    // TODO(bouk): optimize if we don't need to know all permutations
    pub fn discard_permutations(&self) -> [Vec<Tokens>; 4] {
        let mut result = [vec![Tokens::empty()], Vec::with_capacity(5), Vec::with_capacity(15), Vec::with_capacity(20)];
        for (color1, iter2) in Color::all_except_joker().copying() {
            for count1 in 1..min(self[color1], 3) + 1 {
                for (color2, iter3) in iter2.copying() {
                    for count2 in 1..min(self[color2], 3 - count1) + 1 {
                        for color3 in iter3 {
                            for count3 in 1..min(self[color3], 3 - count1 - count2) + 1 {
                                let mut tokens = Tokens::empty();
                                tokens[color1] = count1;
                                tokens[color2] = count2;
                                tokens[color3] = count3;
                                result[(count1 + count2 + count3) as usize].push(tokens);
                            }
                        }
                        let mut tokens = Tokens::empty();
                        tokens[color1] = count1;
                        tokens[color2] = count2;
                        result[(count1 + count2) as usize].push(tokens);
                    }
                }
                let mut tokens = Tokens::empty();
                tokens[color1] = count1;
                result[count1 as usize].push(tokens);
            }
        }
        result
    }
}

impl Index<Color> for Tokens {
    type Output = u8;

    fn index<'a>(&'a self, color: Color) -> &'a u8 {
        match color {
            Color::Black => &self.black,
            Color::Blue  => &self.blue,
            Color::Green => &self.green,
            Color::Red   => &self.red,
            Color::White => &self.white,
            Color::Joker => &self.joker,
        }
    }
}

impl IndexMut<Color> for Tokens {
    fn index_mut<'a>(&'a mut self, color: Color) -> &'a mut u8 {
        match color {
            Color::Black => &mut self.black,
            Color::Blue  => &mut self.blue,
            Color::Green => &mut self.green,
            Color::Red   => &mut self.red,
            Color::White => &mut self.white,
            Color::Joker => &mut self.joker,
        }
    }
}

impl AddAssign for Tokens {
    fn add_assign(&mut self, other: Tokens) {
        self.black += other.black;
        self.blue  += other.blue;
        self.green += other.green;
        self.red   += other.red;
        self.white += other.white;
        self.joker += other.joker;
    }
}

impl SubAssign for Tokens {
    fn sub_assign(&mut self, other: Tokens) {
        self.black -= other.black;
        self.blue  -= other.blue;
        self.green -= other.green;
        self.red   -= other.red;
        self.white -= other.white;
        self.joker -= other.joker;
    }
}

impl Add for Tokens {
    type Output = Tokens;

    fn add(self, other: Tokens) -> Tokens {
        Tokens {
            black: self.black + other.black,
            blue:  self.blue  + other.blue,
            green: self.green + other.green,
            red:   self.red   + other.red,
            white: self.white + other.white,
            joker: self.joker + other.joker,
        }
    }
}

impl Sub for Tokens {
    type Output = Tokens;

    fn sub(self, other: Tokens) -> Tokens {
        Tokens {
            black: self.black - other.black,
            blue:  self.blue  - other.blue,
            green: self.green - other.green,
            red:   self.red   - other.red,
            white: self.white - other.white,
            joker: self.joker - other.joker,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_tokens_with_color() {
        let tokens = Tokens {
            black: 1,
            blue: 2,
            green: 3,
            red: 4,
            white: 5,
            joker: 0,
        };
        assert_eq!(tokens[Color::Black], 1);
        assert_eq!(tokens[Color::Blue], 2);
        assert_eq!(tokens[Color::Green], 3);
        assert_eq!(tokens[Color::Red], 4);
        assert_eq!(tokens[Color::White], 5);
    }

    #[test]
    fn mutate_tokens_with_color() {
        let mut tokens = Tokens {
            black: 1,
            blue: 2,
            green: 3,
            red: 4,
            white: 5,
            joker: 0,
        };

        tokens[Color::Black] += 1;
        tokens[Color::Blue] += 1;
        tokens[Color::Green] += 1;
        tokens[Color::Red] += 1;
        tokens[Color::White] = 7;

        assert_eq!(tokens[Color::Black], 2);
        assert_eq!(tokens[Color::Blue], 3);
        assert_eq!(tokens[Color::Green], 4);
        assert_eq!(tokens[Color::Red], 5);
        assert_eq!(tokens[Color::White], 7);
    }

    #[test]
    fn can_buy() {
        let tokens = Tokens {
            black: 1,
            blue: 0,
            green: 0,
            red: 0,
            white: 0,
            joker: 1,
        };

        assert!(tokens.can_buy(&Tokens {
            black: 1,
            blue: 0,
            green: 0,
            red: 0,
            white: 0,
            joker: 0,
        }));

        assert!(tokens.can_buy(&Tokens {
            black: 2,
            blue: 0,
            green: 0,
            red: 0,
            white: 0,
            joker: 0,
        }));

        assert!(tokens.can_buy(&Tokens {
            black: 1,
            blue: 1,
            green: 0,
            red: 0,
            white: 0,
            joker: 0,
        }));

        assert!(!tokens.can_buy(&Tokens {
            black: 1,
            blue: 1,
            green: 1,
            red: 0,
            white: 0,
            joker: 0,
        }));

        assert!(tokens.can_buy(&Tokens {
            black: 0,
            blue: 0,
            green: 0,
            red: 0,
            white: 0,
            joker: 0,
        }));
    }

    #[test]
    fn discard_permutations() {
        let tokens = Tokens {
            black: 0,
            blue: 0,
            green: 3,
            red: 0,
            white: 0,
            joker: 1,
        };

        assert_eq!(tokens.discard_permutations(), [vec![
            Tokens::empty(),
        ], vec![
            Tokens::one(Color::Green),
        ], vec![
            Tokens {
                black: 0,
                blue: 0,
                green: 2,
                red: 0,
                white: 0,
                joker: 0,
            },
        ], vec![
            Tokens {
                black: 0,
                blue: 0,
                green: 3,
                red: 0,
                white: 0,
                joker: 0,
            },
        ]]);
    }
}
