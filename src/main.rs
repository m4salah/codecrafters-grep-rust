use std::env;
use std::io;
use std::process;

#[derive(Debug)]
pub enum Pattern {
    SingleCharacter,
    SingleDigit,
    ExactWord(String),
}

impl Pattern {
    fn parse(s: &str) -> Vec<Pattern> {
        let mut pats = Vec::new();
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            if c == '\\' {
                println!("c: {}", c);
                if let Some(w) = it.next() {
                    println!("w: {}", w);
                    if w == 'w' {
                        pats.push(Pattern::SingleCharacter)
                    } else if w == 'd' {
                        pats.push(Pattern::SingleDigit)
                    } else if w == '\\' {
                        pats.push(Pattern::ExactWord("\\".to_string()));
                    } else {
                        todo!("Unhandled pattern");
                    }
                }
            } else {
                let mut exact_string = String::new();
                exact_string.push(c);
                while let Some(&ch) = it.peek() {
                    if ch == '\\' {
                        break;
                    } else {
                        exact_string.push(ch);
                        it.next();
                    }
                }
                pats.push(Pattern::ExactWord(exact_string));
            }
        }
        return pats;
    }

    fn match_pat(&self, s: &str) -> Option<usize> {
        match self {
            Pattern::SingleCharacter => s.chars().position(|c| c.is_alphanumeric() || c == '_'),
            Pattern::SingleDigit => s.chars().position(|c| c.is_ascii_digit()),
            Pattern::ExactWord(word) => s.find(word),
        }
    }

    fn consumed_chars(&self) -> usize {
        match self {
            Pattern::SingleCharacter | Pattern::SingleDigit => 1,
            Pattern::ExactWord(word) => word.chars().count(),
        }
    }

    fn match_all(pats: &[Self], s: &str, from_beginin: bool) -> bool {
        if pats.is_empty() {
            return true;
        }

        let first_pat = &pats[0];

        if let Some(char_idx) = first_pat.match_pat(s) {
            if (from_beginin && char_idx != 0) {
                return false;
            }
            // Calculate how many characters this pattern consumed

            let consumed_chars = first_pat.consumed_chars();
            // Find the byte index after consuming the pattern
            let after_match = s
                .char_indices()
                .nth(char_idx + consumed_chars)
                .map(|(idx, _)| idx)
                .unwrap_or(s.len());

            // Recursively match the rest
            Self::match_all(&pats[1..], &s[after_match..], from_beginin)
        } else {
            false
        }
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let pats = Pattern::parse(pattern);
    println!("{:#?}", pats);

    if pattern.starts_with('[') && pattern.ends_with(']') {
        let pat_len = pattern.len();
        let finding_chars = &pattern[1..pat_len - 1];
        if finding_chars.starts_with('^') {
            return input_line.chars().any(|c| !finding_chars.contains(c));
        } else {
            return input_line.chars().any(|c| finding_chars.contains(c));
        }
    } else if pattern.starts_with('^') {
        let start_with_pats = Pattern::parse(&pattern[1..]);
        return Pattern::match_all(&start_with_pats, input_line, true);
    } else {
        return Pattern::match_all(&pats, input_line, false);
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // TODO: Uncomment the code below to pass the first stage
    if match_pattern(&input_line, &pattern) {
        println!("Pattern matched");
        process::exit(0)
    } else {
        eprintln!("Pattern did NOT matched");
        process::exit(1)
    }
}
