use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            matrix.push(line.chars().collect());
        }
    }

    for (i, el) in matrix.iter().enumerate() {
        for (j, c) in el.iter().enumerate() {
            if *c == 'X' {
                // search for M in every direction
                let dir = [-1, 0, 1];
                for dir_x in dir {
                    for dir_y in dir {
                        if search(vec!['M', 'A', 'S'], matrix.clone(), dir_x, dir_y, i as i32, j as i32) {
                            sum += 1;
                        }
                    }
                }
            }
        }
    }
    for (i, el) in matrix.iter().enumerate() {
        for (j, c) in el.iter().enumerate() {
            if *c == 'A' {
                // search for X-MAS form
                if ((search(vec!['M'], matrix.clone(), -1, -1, i as i32, j as i32) &&
                   search(vec!['S'], matrix.clone(), 1, 1, i as i32, j as i32)) ||
                   (search(vec!['S'], matrix.clone(), -1, -1, i as i32, j as i32) &&
                    search(vec!['M'], matrix.clone(), 1, 1, i as i32, j as i32))) &&
                    ((search(vec!['M'], matrix.clone(), -1, 1, i as i32, j as i32) &&
                   search(vec!['S'], matrix.clone(), 1, -1, i as i32, j as i32)) ||
                   (search(vec!['S'], matrix.clone(), -1, 1, i as i32, j as i32) &&
                    search(vec!['M'], matrix.clone(), 1, -1, i as i32, j as i32))) {
                        sum2 += 1;
                }
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

fn search(
    s: Vec<char>,
    matrix: Vec<Vec<char>>,
    dir_x: i32,
    dir_y: i32,
    start_x: i32,
    start_y: i32,
) -> bool {
    let c = s[0];
    let x = start_x + dir_x;
    let y = start_y + dir_y;
    let len = matrix.len() as i32;
    let width = matrix[0].len() as i32;

    if x < 0 || y < 0 || x >= width || y >= len {
        return false;
    }
    if matrix[x as usize][y as usize] == c {
        if s.len() == 1 {
            return true;
        }
        let mut t = s.clone();
        t.remove(0);
        return search(t, matrix, dir_x, dir_y, x, y);
    }
    return false;
}
