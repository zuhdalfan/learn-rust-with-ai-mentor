use std::io::{self, Read, Write}; // bring in stdin/stdout helpers and Read/Write traits

fn main() { // program entry point
    // Goal: read two integers from stdin and print sum/diff/product. // simple high-level goal
    // If parsing fails, print a friendly error and exit. // error handling expectation

    let mut input = String::new(); // reusable buffer to hold input text

    eprintln!("Enter two numbers (CTRL+D when done):"); // prompt for first read (EOF-terminated)
    if io::stdin().read_to_string(&mut input).is_err() { // read all stdin into input
        eprintln!("Failed to read input"); // error message if reading fails
        return; // stop program early
    } else { // success branch
        eprintln!("\nInput: {}", input); // echo back what was read
    } // end of read_to_string handling

    eprintln!("Re-enter two numbers (ENTER when done):"); // prompt for a single line
    let _ = io::stdout().flush(); // flush the prompt so it shows before waiting

    input.clear(); // empty the buffer so we can reuse it
    if io::stdin().read_line(&mut input).is_err() { // read one line into input
        eprintln!("Failed to read input"); // error message if reading fails
        return; // stop program early
    } else { // success branch
        eprintln!("\nInput: {}", input); // echo back what was read
    } // end of read_line handling

    let mut parts = input.split_whitespace(); // split the line into space-separated tokens
    let a = match parts.next().and_then(|s| s.parse::<i64>().ok()) { // parse the first token as i64
        Some(a) => a, // parsing succeeded; use the value
        None => return, // parsing failed; exit early
    }; // end match for a

    let mut parts = input.split_whitespace(); // split again (this restarts the iterator)
    let b = match parts.next().and_then(|s| s.parse::<i64>().ok()) { // parses the first token again (logic bug)
        Some(b) => b, // parsing succeeded; use the value
        None => return, // parsing failed; exit early
    }; // end match for b

    eprintln!("sum: {}", a + b); // print the sum
    eprintln!("diff: {}", a - b); // print the difference
    eprintln!("product: {}", a * b); // print the product

    return; // explicit return (not needed in Rust)
} // end main
