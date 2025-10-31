use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else if pattern.contains("\\d") {
        return input_line.chars().any(|c| c.is_numeric());
    } else if pattern.contains("\\w") {
        return input_line.chars().any(|c| c.is_alphanumeric() || c == '_');
    } else if pattern.starts_with('[') && pattern.ends_with(']') {
        let pat_len = pattern.len();
        let finding_chars = &pattern[1..pat_len - 1];
        return input_line.chars().any(|c| finding_chars.contains(c));
    } else {
        panic!("Unhandled pattern: {}", pattern)
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
