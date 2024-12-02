use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let mut cols = line.split_whitespace();
            col1.push(cols.next().unwrap().parse::<i32>().unwrap());
            col2.push(cols.next().unwrap().parse::<i32>().unwrap());
        }
    }
    col1.sort();
    col2.sort();

    if col1.len() != col2.len() {
        println!("sizes are different");
        process::exit(1);
    } else {
        println!("size are the same, proceeding");
    }
    for (&mut a, &mut b) in col1.iter_mut().zip(col2.iter_mut()) {
        //println!("abs({} - {}) = {}", a , b , (a - b).abs());
        sum += (a - b).abs();
    }
    println!("Sum is :{}", sum);

    let mut locations = HashMap::new();
    let mut sum: i32 = 0;

    for loc in col1.iter() {
        locations.entry(loc).or_insert(0);
    }
    for loc in col2.iter() {
        if locations.contains_key(&loc) {
            let count = locations.get(&loc).unwrap();
            locations.insert(loc, count + 1);
        }
    }
    for loc in col1.iter() {
        //println!("sum {} += {} * {}", sum, loc, locations.get(loc).unwrap());
        sum += loc * locations.get(loc).unwrap();
    }
    println!("distance is :{}", sum);
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
