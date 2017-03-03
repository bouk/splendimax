use cost::Tokens;

fn create_noble(black: u8, blue: u8, green: u8, red: u8, white: u8) -> Noble {
    Noble {
        cost: Tokens {
            black: black,
            blue: blue,
            green: green,
            red: red,
            white: white,
            joker: 0,
        },
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Noble {
    pub cost: Tokens,
}

impl Noble {
    pub fn all() -> Vec<Noble> {
        vec![
            create_noble(0, 0, 4, 4, 0),
            create_noble(3, 0, 0, 3, 3),
            create_noble(4, 0, 0, 0, 4),
            create_noble(0, 4, 4, 0, 0),
            create_noble(0, 3, 3, 3, 0),
            create_noble(0, 3, 3, 0, 3),
            create_noble(4, 0, 0, 4, 0),
            create_noble(3, 3, 0, 0, 3),
            create_noble(3, 0, 3, 3, 0),
        ]
    }
}
