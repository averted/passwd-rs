const INVALID: [Ascii; 8] = [
    Ascii::DoubleQuote,
    Ascii::SingleQuote,
    Ascii::Slash,
    Ascii::LessThan,
    Ascii::GreaterThan,
    Ascii::BackSlash,
    Ascii::Accent,
    Ascii::Bar,
];

const SPECIAL: [Ascii; 25] = [
    Ascii::ExclamationMark,
    Ascii::Number,
    Ascii::DollarSign,
    Ascii::Percent,
    Ascii::Ampersand,
    Ascii::LeftParenthesis,
    Ascii::RightParenthesis,
    Ascii::Asterisk,
    Ascii::Plus,
    Ascii::Comma,
    Ascii::Minus,
    Ascii::Period,
    Ascii::Slash,
    Ascii::Colon,
    Ascii::Semicolon,
    Ascii::Equal,
    Ascii::QuestionMark,
    Ascii::At,
    Ascii::LeftBracket,
    Ascii::RightBracket,
    Ascii::Caret,
    Ascii::Underscore,
    Ascii::LeftCurlyBracket,
    Ascii::RightCurlyBracket,
    Ascii::Tilde,
];

// TODO: Replace with marco
#[derive(Debug, PartialEq)]
pub enum Ascii {
    ExclamationMark = 33,
    DoubleQuote = 34,
    Number = 35,
    DollarSign = 36,
    Percent = 37,
    Ampersand = 38,
    SingleQuote = 39,
    LeftParenthesis = 40,
    RightParenthesis = 41,
    Asterisk = 42,
    Plus = 43,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Colon = 58,
    Semicolon = 59,
    LessThan = 60,
    Equal = 61,
    GreaterThan = 62,
    QuestionMark = 63,
    At = 64,
    LeftBracket = 91,
    BackSlash = 92,
    RightBracket = 93,
    Caret = 94,
    Underscore = 95,
    Accent = 96,
    LeftCurlyBracket = 123,
    Bar = 124,
    RightCurlyBracket = 125,
    Tilde = 126,
}

impl Ascii {
    pub fn from_u8(val: u8) -> Option<Ascii> {
        match val {
            33 => Some(Ascii::ExclamationMark),
            34 => Some(Ascii::DoubleQuote),
            35 => Some(Ascii::Number),
            36 => Some(Ascii::DollarSign),
            37 => Some(Ascii::Percent),
            38 => Some(Ascii::Ampersand),
            39 => Some(Ascii::SingleQuote),
            40 => Some(Ascii::LeftParenthesis),
            41 => Some(Ascii::RightParenthesis),
            42 => Some(Ascii::Asterisk),
            43 => Some(Ascii::Plus),
            44 => Some(Ascii::Comma),
            45 => Some(Ascii::Minus),
            46 => Some(Ascii::Period),
            47 => Some(Ascii::Slash),
            58 => Some(Ascii::Colon),
            59 => Some(Ascii::Semicolon),
            60 => Some(Ascii::LessThan),
            61 => Some(Ascii::Equal),
            62 => Some(Ascii::GreaterThan),
            63 => Some(Ascii::QuestionMark),
            64 => Some(Ascii::At),
            91 => Some(Ascii::LeftBracket),
            92 => Some(Ascii::BackSlash),
            93 => Some(Ascii::RightBracket),
            94 => Some(Ascii::Caret),
            95 => Some(Ascii::Underscore),
            96 => Some(Ascii::Accent),
            123 => Some(Ascii::LeftCurlyBracket),
            124 => Some(Ascii::Bar),
            125 => Some(Ascii::RightCurlyBracket),
            126 => Some(Ascii::Tilde),
            _ => None,
        }
    }

    pub fn cmp_invalid(value: u8) -> bool {
        if let Some(ascii_value) = Ascii::from_u8(value) {
            for val in INVALID {
                if ascii_value == val {
                    return true;
                }
            }
        }

        false
    }

    pub fn cmp_special(value: u8) -> bool {
        if let Some(ascii_value) = Ascii::from_u8(value) {
            for val in SPECIAL {
                if ascii_value == val {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_ascii_value_from_u8() {
        assert_eq!(Ascii::from_u8(11), None);
        assert_eq!(Ascii::from_u8(34), Some(Ascii::DoubleQuote));
        assert_eq!(Ascii::from_u8(126), Some(Ascii::Tilde));
    }

    #[test]
    fn compares_invalid_values_correctly() {
        let invalids: Vec<u8> = vec![34, 60, 62, 92, 96];

        for item in invalids {
            assert_eq!(Ascii::cmp_invalid(item), true);
        }

        let valids: Vec<u8> = vec![43, 44, 45, 46];

        for item in valids {
            assert_eq!(Ascii::cmp_invalid(item), false);
        }
    }

    #[test]
    fn compares_special_values_correctly() {
        let specials: Vec<u8> = vec![35, 36, 37, 38];

        for item in specials {
            assert_eq!(Ascii::cmp_special(item), true);
        }

        let non_specials: Vec<u8> = vec![65, 66, 67, 68, 69];

        for item in non_specials {
            assert_eq!(Ascii::cmp_special(item), false);
        }
    }
}
