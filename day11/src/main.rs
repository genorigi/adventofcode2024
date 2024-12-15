use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct StoneLine {
    stoneline: Vec<Stone>,
    len: HashMap<Stone, HashMap<i64, i64>>,
}

impl StoneLine {
    fn new() -> Self {
        Self {
            stoneline: Vec::new(),
            len: HashMap::new(),
        }
    }

    fn len(&self, s: &Stone, iter: i64) -> Option<&i64> {
        if let Some(s_hash) = self.len.get(&s) {
            return s_hash.get(&iter);
        }
        return None;
    }
    fn insert_len(&mut self, s: Stone, iter: i64, value: i64) {
        if let Some(s_hash) = self.len.get_mut(&s) {
            s_hash.insert(iter, value);
        } else {
            let mut new_hash: HashMap<i64, i64> = HashMap::new();
            new_hash.insert(iter, value);
            self.len.insert(s, new_hash);
        }
    }

    fn find_len(&mut self, s: Stone, iter: i64) -> i64 {
        if iter == 0 {
            self.insert_len(s, iter, 1);
            return 1;
        }
        if let Some(len) = self.len(&s, iter) {
            //println!("found in hash");
            //dbg!("s:{} iter: {} hashmap:{}", &s, iter,&self.len);
            //pause();
            return *len;
        }
        //println!("not found in hash");
        
        let mut len: i64 = 0;
        for stone in s.next() {
            len += self.find_len(stone, iter - 1)
        }
        self.insert_len(s, iter, len);
        return len;
    }
}
#[derive(Debug, Clone, Hash)]
struct Stone {
    mark: i64,
}

impl PartialEq for Stone {
    fn eq(&self, other: &Self) -> bool {
        self.mark == other.mark
    }
}

impl Eq for Stone {}

impl Stone {
    fn print(&self) {
        println!("{}", self.mark);
    }

    fn next(&self) -> Vec<Stone> {
        if self.mark == 0 {
            let stone = Stone { mark: 1 };
            return vec![stone];
        }
        let mark_string = self.mark.to_string();
        let length = mark_string.len();
        if length % 2 == 0 {
            let stone1 = Stone {
                mark: mark_string[0..length / 2].parse::<i64>().unwrap(),
            };
            let stone2 = Stone {
                mark: mark_string[length / 2..length].parse::<i64>().unwrap(),
            };
            return vec![stone1, stone2];
        }
        let stone = Stone {
            mark: self.mark * 2024,
        };
        return vec![stone];
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut stone_line = StoneLine::new();
    let mut sum2: i64 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            stone_line.stoneline = line
                .split_whitespace()
                .map(|s| Stone {
                    mark: s.parse::<i64>().unwrap(),
                })
                .collect();
        }
    }

    let mut stone_line2 = stone_line.clone();

    //part1 - dumm solution
    for i in 0..25 {
        let mut new_stone_line: Vec<Stone> = Vec::new();
        for stone in stone_line.stoneline.clone() {
            new_stone_line.extend(stone.next());
        }
        stone_line.stoneline = new_stone_line;
    }

    for stone in stone_line2.stoneline {
        sum2 += stone_line.find_len(stone, 75)
    }
    //part 2 let's try to find another way

    println!("Sum is :{}", stone_line.stoneline.len());
    println!("Sum2 is :{}", sum2);
}

//number of X
fn number_of_x(matrix: Vec<Vec<char>>, x: char) -> i32 {
    let mut sum: i32 = 0;
    for line in matrix.iter() {
        for col in line.iter() {
            if *col == x {
                sum += 1
            }
        }
    }
    return sum;
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

fn pause() {
    dbg!("Pausing! Press enter to continue...");

    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}
