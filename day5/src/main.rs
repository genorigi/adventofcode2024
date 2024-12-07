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
    let mut orders: Vec<String> = Vec::new();
    // parse using filename - part1
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let result = r_orders.captures(&line);
            if result.is_some() {
                let result = result.unwrap();
                orders.push(result.get(0).unwrap().as_str().to_string());
                continue;
            }
            // if we are there we are done with the constraints so orders is full
            // now we tackle the lines
            let result = r_prints.captures(&line);
            if result.is_some() {
                let prints: Vec<_> = result
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(String::from)
                    .collect();
                // build all pairs
                if is_ok(prints.clone(), orders.clone()) {
                    sum += prints[prints.len() / 2].parse::<i32>().unwrap()
                }
            }
        }
    }
    // part2
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let result = r_orders.captures(&line);
            if result.is_some() {
                let result = result.unwrap();
                orders.push(result.get(0).unwrap().as_str().to_string());
                continue;
            }
            // if we are there we are done with the constraints so orders is full
            // now we tackle the lines
            let result = r_prints.captures(&line);
            if result.is_some() {
                let mut prints: Vec<_> = result
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(String::from)
                    .collect();
                // we take now the nok prints and rework them
                if !is_ok(prints.clone(), orders.clone()) {
                    // we build the order from scratch
                    let mut prints_splice: Vec<String> = vec![prints[0].clone()];
                    prints.remove(0);
                    for page in prints.iter() {
                        prints_splice = order(page.to_string(), prints_splice.clone(), orders.clone());
                    }
                    
                    sum2 += prints_splice[prints_splice.len() / 2].parse::<i32>().unwrap()
                }
            }
        }
    }
    println!("sum is {}", sum);
    println!("sum2 is {}", sum2);
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

fn is_ok(prints: Vec<String>, orders: Vec<String>) -> bool {
    let mut ok: bool = true;
    for (num, page) in prints.iter().enumerate() {
        for otherpage in prints[num + 1..].iter() {
            let test_order = page.to_string() + "|" + otherpage;
            ok = ok && orders.iter().any(|i| *i == test_order);
        }
        if num > 0 {
            for otherpage in prints[..num].iter() {
                let test_order = page.to_string() + "|" + otherpage;
                ok = ok && !orders.iter().any(|i| *i == test_order);
            }
        }
    }
    return ok;
}

fn order(item: String, prints: Vec<String>, orders: Vec<String>) -> Vec<String> {
    //insert an item into a prints
    //dbg!("calling order with item: {}, prints {} ", &item, &prints);
    for i in 0..prints.len() + 1 {
        let mut new_prints = prints.clone();
        new_prints.insert(i, item.clone());
        // dbg!("prints after insert {}", &new_prints);
        if is_ok(new_prints.clone(), orders.clone()) {
            // dbg!("prints is ok {}", &new_prints);
            return new_prints;
        }
    }
    println!("raté ");
    return vec!["".to_string()];
}
