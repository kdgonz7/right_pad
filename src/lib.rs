use pad::PadRules;

mod pad {
    type PadResult = Result<String, PadErr>;

    pub struct PadRules {
        pub pad_char: char,
        pub truncate: bool,
        pub use_ellipsis: bool,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum PadErr {
        Overflow,
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
                return Err(PadErr::Overflow);
            }

            if spaces < 4 {
                return Err(PadErr::StringDoesntFit);
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
}

#[cfg(test)]
mod tests {
    use crate::pad::{right_pad, PadRules};

    #[test]
    fn test_right_pad() {
        assert_eq!(right_pad("hello", 6, &PadRules::default()).unwrap(), "hello ");
        assert_eq!(right_pad("hello", 7, &PadRules::default()).unwrap(), "hello  ");
        assert_eq!(right_pad("hel", 7, &PadRules::default()).unwrap(), "hel    ");
    }
}
