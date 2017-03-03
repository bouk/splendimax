#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    White,
    Joker,
}

impl Color {
    pub fn all() -> AllIter {
        AllIter {
            current: Some(Color::Black),
        }
    }

    pub fn all_except_joker() -> AllExceptJokerIter {
        AllExceptJokerIter {
            current: Some(Color::Black),
        }
    }

    pub fn code(&self) -> &'static str {
        match *self {
            Color::Black => "K",
            Color::Blue => "B",
            Color::Green => "G",
            Color::Red => "R",
            Color::White => "W",
            Color::Joker => "J",
        }
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct AllExceptJokerIter {
    current: Option<Color>,
}

impl Iterator for AllExceptJokerIter {
    type Item = Color;

    fn next(&mut self) -> Option<Color> {
        let ret = self.current;
        self.current = self.current.and_then(|c|
                                        match c {
                                            Color::Black => Some(Color::Blue),
                                            Color::Blue  => Some(Color::Green),
                                            Color::Green => Some(Color::Red),
                                            Color::Red   => Some(Color::White),
                                            Color::Joker | Color::White => None,
                                        }
        );
        ret
    }
}

#[derive(Clone)]
#[derive(Copy)]
pub struct AllIter {
    current: Option<Color>,
}

impl Iterator for AllIter {
    type Item = Color;

    fn next(&mut self) -> Option<Color> {
        let ret = self.current;
        self.current = self.current.and_then(|c|
                                        match c {
                                            Color::Black => Some(Color::Blue),
                                            Color::Blue  => Some(Color::Green),
                                            Color::Green => Some(Color::Red),
                                            Color::Red   => Some(Color::White),
                                            Color::White => Some(Color::Joker),
                                            Color::Joker => None,
                                        }
        );
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_all() {
        let mut iter = Color::all();
        assert_eq!(iter.next(), Some(Color::Black));
        assert_eq!(iter.next(), Some(Color::Blue));
        assert_eq!(iter.next(), Some(Color::Green));
        assert_eq!(iter.next(), Some(Color::Red));
        assert_eq!(iter.next(), Some(Color::White));
        assert_eq!(iter.next(), Some(Color::Joker));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_all_except_joker() {
        let mut iter = Color::all_except_joker();
        assert_eq!(iter.next(), Some(Color::Black));
        assert_eq!(iter.next(), Some(Color::Blue));
        assert_eq!(iter.next(), Some(Color::Green));
        assert_eq!(iter.next(), Some(Color::Red));
        assert_eq!(iter.next(), Some(Color::White));
        assert_eq!(iter.next(), None);
    }
}
