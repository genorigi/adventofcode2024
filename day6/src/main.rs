use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut sum: i32 = 0;
    //let mut sum2: i32 = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut guard_line_nbr:i32 = 0;
    let mut guard_col_nbr:i32 = 0;
    let mut guard_position = 'o';
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        let mut line_nbr = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            matrix.push(line.chars().collect());
            let col_nbr = line.chars().position(|c| c=='^');
            if col_nbr.is_some() {
                guard_col_nbr = col_nbr.unwrap() as i32;
                guard_line_nbr = line_nbr;
                guard_position = line.chars().nth(col_nbr.unwrap()).unwrap();
            }
            line_nbr+=1;
        }
    }
    
    // size of the room
    let line_max = matrix.len() as i32;
    let col_max = matrix[0].len() as i32;
    println!("guard {} is at [{},{}]", guard_position, guard_line_nbr, guard_col_nbr);
    println!("matrix is {} by {}", line_max, col_max);
    let mut inside: bool = true;

    while inside {
        let (next_line, next_col) = guard_next(guard_position, guard_line_nbr, guard_col_nbr);
        
        inside = !is_out(next_line, next_col, line_max, col_max);
        if !inside {
            break
        }
        if matrix[next_line as usize][next_col as usize] == '#' {
            guard_position = guard_turn(guard_position);
            continue
        } else {
            guard_line_nbr = next_line;
            guard_col_nbr = next_col;
            matrix[guard_line_nbr as usize][guard_col_nbr as usize] = 'X';
        }

    } 

    print_matrix(matrix.clone());
    sum = number_of_X(matrix);

    println!("Sum is :{}", sum);
    //println!("Sum2 is :{}", sum2);

}

// find if the guard is out
fn is_out(line_nbr: i32, col_nbr: i32, line_max: i32, col_max: i32) -> bool {

    return line_nbr < 0 || col_nbr < 0 || line_nbr >= line_max || col_nbr >= col_max;

}

fn print_matrix(matrix: Vec<Vec<char>>) {
    for line in matrix.iter() {
        for col in line.iter() {
            print!("{}", col);
        }
        println!();
    }
}

//number of X
fn number_of_X(matrix: Vec<Vec<char>>) -> i32 {
    let mut sum: i32 = 0;
    for line in matrix.iter() {
        for col in line.iter() {
            if *col == 'X' {
                sum +=1
            }
        }
    }
    return sum
}

fn guard_turn(guard_position: char) -> char {

    match guard_position {
           '^' => return '>',
           '>' => return 'v',
           '<' => return '^',
           'v' => return '<',
           _ => return 'o',
        }
}
// find guard next pos
fn guard_next(guard_position: char, line_nbr: i32, col_nbr: i32) -> (i32, i32) {
    match guard_position {
           '^' => return (line_nbr -1, col_nbr),
           '>' => return (line_nbr, col_nbr + 1),
           '<' => return (line_nbr, col_nbr - 1),
           'v' => return (line_nbr +1, col_nbr),
           _ => return (line_nbr, col_nbr),
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

