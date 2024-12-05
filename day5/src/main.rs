use std::collections::HashMap;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    let r_orders = Regex::new(r"^([0-9]+)\|([0-9]+)$").unwrap();
    let r_prints = Regex::new(r"^([0-9]+,)+[0-9]+$").unwrap();
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let result = r_orders.captures(&line);
            if result.is_some() {
                let result = result.unwrap();
                let key = result.get(1).unwrap().as_str();
                let value = result.get(2).unwrap().as_str();
                println!("key {}, value {}",key, value);
                continue;
                }
            let result = r_prints.captures(&line);
            if result.is_some() {
                println!("prints {}", result.unwrap().get(0).unwrap().as_str());

            }
        }
    }
}
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

