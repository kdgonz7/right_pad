pub mod pad {
    type PadResult = Result<String, PadErr>;

    pub struct PadRules {
        pub pad_char: char,
        pub truncate: bool,
        pub use_ellipsis: bool,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum PadErr {
        RightOverflow,
        LeftOverflow,
        StringDoesntFit,
    }

    impl Default for PadRules {
        fn default() -> Self {
            Self {
                pad_char: ' ',
                truncate: true,
                use_ellipsis: false,
            }
        }
    }

    pub fn right_pad<'a>(text: &'a str, spaces: usize, rules: &'a PadRules) -> PadResult {
        let mut s = String::from(text);

        if text.len() > spaces {
            if !rules.truncate {
                return Err(PadErr::RightOverflow)
            }

            if rules.use_ellipsis && spaces < 4 {
                return Err(PadErr::StringDoesntFit)
            }

            s.clear();

            if rules.use_ellipsis {
                s.push_str(&text[0..spaces - 3]);
                s.push_str("...");
            } else {
                s.push_str(&text[0..spaces-1]);
            }
        }

        let spaces_needed_to_fit = spaces - s.len();

        /* note: if the text length is larger than the pad size, truncate it. */

        if spaces_needed_to_fit > 0 {
            for _ in 0..spaces_needed_to_fit {
                s.push(rules.pad_char);
            }
        }

        Ok(s)
    }

    pub fn left_pad<'a>(text: &'a str, spaces: usize, rules: &'a PadRules) -> PadResult {
        let mut s = String::new();
        
        if text.len() > spaces {
            return Err(PadErr::LeftOverflow);
        }
        
        let spaces_needed_to_fit = spaces - text.len();

        if spaces_needed_to_fit > 0 {
            for _ in 0..spaces_needed_to_fit {
                s.push(rules.pad_char);
            }
        }

        s.push_str(text);

        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::pad::{left_pad, right_pad, PadRules};

    #[test]
    fn test_right_pad() {
        assert_eq!(right_pad("hello", 6, &PadRules::default()).unwrap(), "hello ");
        assert_eq!(right_pad("hello", 7, &PadRules::default()).unwrap(), "hello  ");
        assert_eq!(right_pad("hel", 7, &PadRules::default()).unwrap(), "hel    ");
    }

    #[test]
    fn test_left_pad() {
        assert_eq!(left_pad("hello", 7, &PadRules::default()).unwrap(), "  hello");
        assert_eq!(left_pad("helloe", 7, &PadRules::default()).unwrap(), " helloe");
        assert_eq!(left_pad("helloea", 7, &PadRules::default()).unwrap(), "helloea");
    }
}
