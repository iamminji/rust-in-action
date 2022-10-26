use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Reading a file manuall line by line
fn main() {
    // Creates a File object that requires a path argument
    // and error handling if the file does not exist.
    // This program crashs if a README.md is not present.
    let f = File::open("README.md").unwrap();
    let mut reader = BufReader::new(f);

    // Reuses a single String object over the lifetime of the program.
    let mut line = String::new();

    loop {
        // Because reading from disk can fail,
        // we need to explicitly handle this.
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);
        // Shrinks the String back to length 0,
        // preventing lines from persisting
        // into the following ones.
        line.truncate(0);
    }
}
