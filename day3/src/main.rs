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
    let do_string = Regex::new(r"do\(\)").unwrap();
    let dont_string = Regex::new(r"don't\(\)").unwrap();
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            sum += mulfunc(line);
        }
    }
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            // println!("base line {}", line.to_string());
            // find first dont
            let mut next_dont = dont_string.find(&line);

            // calculate string between 0 and dont
            // println!("calculate line {}", line[..next_dont.unwrap().start()].to_string());
            if !next_dont.is_some() {
                sum2 += mulfunc(line.to_string());
                continue;
            }
            sum2 += mulfunc(line[..next_dont.unwrap().start()].to_string());

            while next_dont.unwrap().end() < line.len() {
                // find next do
                let next_do = do_string.find_at(&line, next_dont.unwrap().end());
                if !next_do.is_some() {
                    //    println!("we are done");
                    break;
                }
                // find next dont
                next_dont = dont_string.find_at(&line, next_do.unwrap().end());
                if !next_dont.is_some() {
                    //    println!("end of line, calculating {}", line[next_do.unwrap().end()..].to_string());
                    sum2 += mulfunc(line[next_do.unwrap().end()..].to_string());
                    break;
                }
                //println!("processing {}", line[next_do.unwrap().end()..next_dont.unwrap().start()].to_string());
                sum2 +=
                    mulfunc(line[next_do.unwrap().end()..next_dont.unwrap().start()].to_string());

                // calculate string between do and dont
            }
        }
    }
    println!("Sum is :{}", sum);
    println!("Sum2 is :{}", sum2);
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

// calculate mul in a string
//
fn mulfunc(line: String) -> i32 {
    let mul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut sum: i32 = 0;
    let mul_list = mul.captures_iter(&line);
    for i in mul_list {
        let num1 = i
            .get(1)
            .map(|i| i.as_str())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let num2 = i
            .get(2)
            .map(|i| i.as_str())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        sum += num1 * num2;
    }
    return sum;
}
