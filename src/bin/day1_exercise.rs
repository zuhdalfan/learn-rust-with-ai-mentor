use std::io::{self, Read, Write};
fn main() {
    // read 2 int from stdin (same line, space-separated)
    // print their sum, diff, product
    // if input parsing fails, print a friendly error message and exit

    let mut input = String::new();

    eprintln!("Enter two numbers (CTRL+D when done):");
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Failed to read input");
        return;
    }else {
        eprintln!("\nInput: {}", input);
    }

    eprintln!("Re-enter two numbers (ENTER when done):");
    let _=io::stdout().flush();

    input.clear();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Failed to read input");
        return;
    }else {
        eprintln!("\nInput: {}", input);
    }

    let mut parts = input.split_whitespace();
    let a = match parts.next().and_then(|s| s.parse::<i64>().ok()){
        Some(a) => a,
        None => return,
    };

    let mut parts = input.split_whitespace();
    let b = match parts.next().and_then(|s| s.parse::<i64>().ok()){
        Some(b) => b,
        None => return,
    };

    eprintln!("sum: {}", a + b);
    eprintln!("diff: {}", a - b);
    eprintln!("product: {}", a * b);

    return;
}