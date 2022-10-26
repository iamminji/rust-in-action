use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Reading a file line by line via BufReader::lines()
fn main() {
    let f = File::open("README.md").unwrap();
    let reader = BufReader::new(f);

    // A subtle behavior change occurs here.
    // BufReader::lines() removes the trailing newline character from each line.
    for line_ in reader.lines() {
        // Unwraps the Result, but at the risk of crashing the program if an error occurs
        let line = line_.unwrap();
        println!("{}, ({} bytes long)", line, line.len());
    }
}
