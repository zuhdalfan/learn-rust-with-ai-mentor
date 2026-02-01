use std::collections::HashMap; // bring in HashMap for counting words
use std::io::{self, Read}; // bring in stdin and Read trait

fn main() { // program entry point
    let mut input = String::new(); // buffer to hold all stdin text
    if io::stdin().read_to_string(&mut input).is_err() { // read stdin into the buffer
        println!("Failed to read input"); // report read error
        return; // stop program early
    } // end read handling

    let words: Vec<String> = input // start with the full input string
        .split(|c: char| !c.is_alphanumeric()) // split on any non-letter/digit
        .filter(|w| !w.is_empty()) // drop empty chunks
        .map(|w| w.to_ascii_lowercase()) // normalize to lowercase
        .collect(); // collect into a Vec<String>

    if words.is_empty() { // check for empty input after cleaning
        println!("No words found"); // report that nothing was found
        return; // stop program early
    } // end empty check

    let mut counts: HashMap<String, usize> = HashMap::new(); // map word -> count
    for word in words { // iterate over each word (moves Strings out of the Vec)
        let entry = counts.entry(word).or_insert(0); // get count entry or insert 0
        *entry += 1; // increment the count
    } // end counting loop

    let total_words: usize = counts.values().sum(); // sum all counts
    let unique_words: usize = counts.len(); // number of unique keys
    println!("Total words: {}, Unique words: {}", total_words, unique_words); // print stats

    let mut top: Vec<(&String, &usize)> = counts.iter().collect(); // collect references for sorting
    top.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0))); // sort by count desc, then word asc

    for (word, count) in top.iter().take(3) { // take top 3 words
        println!("{}: {}", word, count); // print each word and count
    } // end printing loop
} // end main
