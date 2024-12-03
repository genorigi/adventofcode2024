use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let numbers = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            let numbers2 = line
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            if is_valid(numbers, 0, 0, 0) {
                sum += 1;
            }
            if is_valid(numbers2, 0, 0, 1) {
                sum2 += 1;
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

// return true is the line is valid, false if the line is not valid
// valid: always increasing or always decreasing
// adjacent number differ between 1 and 3
fn is_valid(line: Vec<i32>, index: usize, direction: i32, tolerance: i32) -> bool {
    // at the end, no issue detected, we are happy
    if index + 1 == line.len() {
        return true;
    }
    let diff = line[index + 1] - line[index];
    if diff == 0 {
        if tolerance == 0 {
            return false;
        } else {
            let mut new_line = line.clone();
            new_line.remove(index);
            return is_valid(new_line, 0, 0, 0);
        }
    } else if diff.abs() > 3 {
        if tolerance == 0 {
            return false;
        } else {
            let mut new_line1 = line.clone();
            let mut new_line2 = line.clone();
            new_line1.remove(index);
            new_line2.remove(index + 1);
            return is_valid(new_line1, 0, 0, 0) || is_valid(new_line2, 0, 0, 0);
        }
    }
    let new_direction = diff / diff.abs();
    if direction == 0 {
        return is_valid(line, index + 1, new_direction, tolerance);
    } else if direction != new_direction {
        if tolerance == 0 {
            return false;
        } else {
            let mut new_line1 = line.clone();
            let mut new_line2 = line.clone();
            let mut new_line3 = line.clone();
            new_line1.remove(index);
            new_line2.remove(index + 1);
            new_line3.remove(index - 1);
            return is_valid(new_line1, 0, 0, 0) || is_valid(new_line2, 0, 0, 0) || is_valid(new_line3, 0, 0, 0);
        }
    }

    return is_valid(line, index + 1, new_direction, tolerance);
}
