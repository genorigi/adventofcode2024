use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Matrix {
    initial: Vec<Vec<char>>,
    left: Vec<Vec<char>>,
    right: Vec<Vec<char>>,
    up: Vec<Vec<char>>,
    down: Vec<Vec<char>>,
    blockers: Vec<Vec<char>>,
    path: Vec<Vec<char>>,
    backup: Vec<Vec<char>>,
}

impl Matrix {
    fn xlen(&self) -> i32 {
        self.initial.len() as i32
    }
    fn ylen(&self) -> i32 {
        self.initial[0].len() as i32
    }
    fn new() -> Self {
        Self {
            initial: Vec::new(),
            left: Vec::new(),
            up: Vec::new(),
            right: Vec::new(),
            down: Vec::new(),
            blockers: Vec::new(),
            path: Vec::new(),
            backup: Vec::new(),
        }
    }
    fn add_line(&mut self, line: Vec<char>) {
        self.initial.push(line.clone());
        self.left.push(line.clone());
        self.right.push(line.clone());
        self.up.push(line.clone());
        self.down.push(line.clone());
        self.blockers.push(line.clone());
        self.path.push(line.clone());
        self.backup.push(line);
    }

    fn reset(&mut self) {
        self.left = self.backup.clone();
        self.up = self.backup.clone();
        self.right = self.backup.clone();
        self.down = self.backup.clone();
        self.path = self.backup.clone();
        self.blockers = self.backup.clone();
        self.initial = self.backup.clone();
    }

    fn print(&self) {
        for i in 0..self.xlen() {
            for j in 0..self.ylen() {
                let mut c: char = '.';
                if self.left[i as usize][j as usize] == 'X' {
                    c = '<';
                };
                if self.right[i as usize][j as usize] == 'X' {
                    if c == '<' {
                        c = '-'
                    } else {
                        c = '>'
                    };
                };
                if self.up[i as usize][j as usize] == 'X' {
                    if c == '<' {
                        c = 'J'
                    } else if c == '-' {
                        c = 'W'
                    } else if c == '>' {
                        c = 'L'
                    } else {
                        c = '^'
                    }
                };
                if self.down[i as usize][j as usize] == 'X' {
                    if c == '<' {
                        c = '7'
                    } else if c == '-' {
                        c = 'T'
                    } else if c == '>' {
                        c = 'F'
                    } else if c == 'W' {
                        c = '*'
                    } else if c == 'J' {
                        c = '3'
                    } else if c == 'L' {
                        c = 'E'
                    } else {
                        c = 'v'
                    }
                };
                if self.blockers[i as usize][j as usize] == 'O' {
                    c = 'O';
                };
                if self.initial[i as usize][j as usize] == '#' {
                    c = '#'
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn mark_dir(&mut self, x: i32, y: i32, dir: char) {
        if self.is_inside(x, y) {
            match dir {
                '^' => {
                    if !self.is_dash(x, y) {
                        self.up[x as usize][y as usize] = 'X'
                    }
                }
                '>' => {
                    if !self.is_dash(x, y) {
                        self.right[x as usize][y as usize] = 'X'
                    }
                }
                '<' => {
                    if !self.is_dash(x, y) {
                        self.left[x as usize][y as usize] = 'X'
                    }
                }
                'v' => {
                    if !self.is_dash(x, y) {
                        self.down[x as usize][y as usize] = 'X'
                    }
                }
                'X' => {
                    if !self.is_dash(x, y) {
                        self.path[x as usize][y as usize] = 'X'
                    }
                }
                'O' => {
                    if !self.is_dash(x, y) {
                        self.blockers[x as usize][y as usize] = 'O'
                    }
                }
                _ => (),
            }
        }
    }

    // check if based on direction, we close a loop if we put a O after... meaning the place where
    // we are is going on the right at the same place ... we put the stone in the blockers table
    fn check_loop(&mut self, x: i32, y: i32, dir: char) {
        if self.is_inside(x, y) {
            match dir {
                '^' => {
                    if self.right[x as usize][y as usize] == 'X' {
                        self.mark_dir(x - 1, y, 'O')
                    }
                }
                '>' => {
                    if self.down[x as usize][y as usize] == 'X' {
                        self.mark_dir(x, y + 1, 'O')
                    }
                }
                '<' => {
                    if self.up[x as usize][y as usize] == 'X' {
                        self.mark_dir(x, y - 1, 'O')
                    }
                }
                'v' => {
                    if self.left[x as usize][y as usize] == 'X' {
                        self.mark_dir(x + 1, y, 'O')
                    }
                }
                _ => (),
            }
        }
    }

    fn is_looping(&mut self, x: i32, y: i32, dir: char) -> bool {
        if self.is_inside(x, y) {
            match dir {
                '^' => return self.up[x as usize][y as usize] == 'X',
                '>' => return self.right[x as usize][y as usize] == 'X',
                '<' => return self.left[x as usize][y as usize] == 'X',
                'v' => return self.down[x as usize][y as usize] == 'X',
                _ => return false,
            }
        }
        false
    }

    fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.xlen() && y < self.ylen()
    }

    fn is_dash(&self, x: i32, y: i32) -> bool {
        if !self.is_inside(x, y) {
            false
        } else {
            self.initial[x as usize][y as usize] == '#'
        }
    }

    fn mark_loop(&mut self, x: i32, y: i32, dir: char) {
        // we move backward
        let move_dir = turn(turn(dir));
        let (x, y) = next_place(x, y, move_dir);
        if self.is_inside(x, y) && !self.is_dash(x, y) {
            if self.obstacle_on_left(x, y, dir) {
                self.mark_loop(x, y, turn(turn(turn(dir))));
            }
            self.mark_dir(x, y, dir);
            self.mark_loop(x, y, dir);
        }
    }

    // return true if there is a # on the left of the guard
    fn obstacle_on_left(&self, x: i32, y: i32, dir: char) -> bool {
        match dir {
            '^' => return self.is_dash(x, y - 1),
            '>' => return self.is_dash(x - 1, y),
            '<' => return self.is_dash(x + 1, y),
            'v' => return self.is_dash(x, y + 1),
            _ => return false,
        }
    }
}

#[derive(Debug)]
struct Guard {
    x: i32,
    y: i32,
    dir: char,
}

impl Guard {
    fn next_guard(&self) -> Self {
        match self.dir {
            '^' => Guard {
                x: self.x - 1,
                y: self.y,
                dir: self.dir,
            },
            '>' => Guard {
                y: self.y + 1,
                x: self.x,
                dir: self.dir,
            },
            '<' => Guard {
                y: self.y - 1,
                x: self.x,
                dir: self.dir,
            },
            'v' => Guard {
                x: self.x + 1,
                y: self.y,
                dir: self.dir,
            },
            _ => Guard {
                x: self.x,
                y: self.y,
                dir: self.dir,
            },
        }
    }

    fn turn(&mut self) {
        self.dir = turn(self.dir);
    }
}

fn turn(dir: char) -> char {
    match dir {
        '^' => '>',
        '>' => 'v',
        '<' => '^',
        'v' => '<',
        _ => '.',
    }
}

fn next_place(x: i32, y: i32, dir: char) -> (i32, i32) {
    match dir {
        '^' => (x - 1, y),
        '>' => (x, y + 1),
        '<' => (x, y - 1),
        'v' => (x + 1, y),
        _ => (x, y),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut matrix: Matrix = Matrix::new();
    let mut guard: Guard = Guard {
        x: 0,
        y: 0,
        dir: '^',
    };
    let mut guard2: Guard = Guard {
        x: 0,
        y: 0,
        dir: '^',
    };
    //let mut sum2: i32 = 0;
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        let mut line_nbr = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            matrix.add_line(line.chars().collect());
            // find the guard
            let col_nbr = line.chars().position(|c| c == '^');
            if col_nbr.is_some() {
                // we found him, let's keep the value (so we don't go through the matrix another
                // time
                guard = Guard {
                    x: line_nbr,
                    y: col_nbr.unwrap() as i32,
                    dir: line.chars().nth(col_nbr.unwrap()).unwrap(),
                };
                guard2 = Guard {
                    x: line_nbr,
                    y: col_nbr.unwrap() as i32,
                    dir: line.chars().nth(col_nbr.unwrap()).unwrap(),
                };
            }
            line_nbr += 1;
        }
    }

    println!("guard {} is at [{},{}]", guard.dir, guard.x, guard.y);
    println!("matrix is {} by {}", matrix.xlen(), matrix.ylen());
    let mut inside: bool = true;

    // part1
    // we mark the first place
    matrix.mark_dir(guard.x, guard.y, 'X');
    while inside {
        let next_guard = guard.next_guard();

        inside = matrix.is_inside(next_guard.x, next_guard.y);
        if !inside {
            break;
        }
        if matrix.is_dash(next_guard.x, next_guard.y) {
            guard.turn();
            continue;
        } else {
            guard = next_guard;
            matrix.mark_dir(guard.x, guard.y, 'X');
        }
    }

    /*
        // part2, we reset the place of the guard
        guard = guard2;
        // we look for loops paths from the beginning position
        matrix.mark_loop(guard.x, guard.y, guard.dir);
        matrix.mark_dir(guard.x, guard.y, guard.dir);
        if matrix.obstacle_on_left(guard.x, guard.y, guard.dir) {
            matrix.mark_loop(guard.x, guard.y, turn(turn(turn(guard.dir))));
        }
        let mut inside: bool = true;
        while inside {
            //matrix.print();
            //pause();
            let next_guard = guard.next_guard();

            inside = matrix.is_inside(next_guard.x, next_guard.y);
            if !inside {
                break;
            }
            if matrix.is_dash(next_guard.x, next_guard.y) {
                guard.turn();
                matrix.mark_loop(guard.x, guard.y, guard.dir);
                continue;
            } else {
                guard = next_guard;
                matrix.check_loop(guard.x, guard.y, guard.dir);
                matrix.mark_dir(guard.x, guard.y, guard.dir);
                if matrix.obstacle_on_left(guard.x, guard.y, guard.dir) {
                    matrix.mark_loop(guard.x, guard.y, turn(turn(turn(guard.dir))));
                }
            }
        }
    */
    // try brute force
    let mut sum2: i32 = 0;

    for i in 0..matrix.xlen() {
        for j in 0..matrix.ylen() {
            // reset the matrix
            matrix.reset();
            let mut inside: bool = true;
            let mut guard = Guard {
                x: guard2.x,
                y: guard2.y,
                dir: guard2.dir,
            };

            // test if there is a loop
            if matrix.is_dash(i, j) {
                // there is a dash, skip
                continue;
            }
            // add a dash
            matrix.initial[i as usize][j as usize] = '#';
            // we insert a dash in the matrix
            while inside {
                //matrix.print();
                //pause();
                let next_guard = guard.next_guard();

                inside = matrix.is_inside(next_guard.x, next_guard.y);
                if !inside {
                    break;
                }
                if matrix.is_dash(next_guard.x, next_guard.y) {
                    guard.turn();
                    //pause();

                    if matrix.is_looping(guard.x, guard.y, guard.dir) {
                        sum2 += 1;
                        break;
                    }
                    matrix.mark_dir(guard.x, guard.y, guard.dir);
                    continue;
                } else {
                    guard = next_guard;
                    matrix.mark_dir(guard.x, guard.y, guard.dir);
                }
            }
        }
    }

    let sum = number_of_x(matrix.path, 'X');
    //let sum2 = number_of_x(matrix.blockers, 'O');

    println!("Sum is :{}", sum);
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
