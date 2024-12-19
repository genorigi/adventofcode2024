use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Score {
    X: f64,
    Y: f64,
}

#[derive(Debug, Clone)]
struct Button {
    X: f64,
    Y: f64,
}

impl Button {
    fn push(&self, num: f64) -> Score {
        return Score {
            X: self.X * num,
            Y: self.Y * num,
        };
    }
}

#[derive(Debug, Clone)]
struct Game {
    A: Button,
    B: Button,
    Prize: Score,
}

impl Game {
    fn solution(&self) -> f64{
        return (( ( (self.A.X / self.A.Y) * self.Prize.Y) - self.Prize.X ) / ( ( ( self.A.X / self.A.Y ) * self.B.Y ) - self.B.X)).round();

    }

    fn test(&self, solution: f64) -> bool{
        let a: f64 = ((self.Prize.Y - self.B.Y * solution) / self.A.Y);
        if a > 100.0 || solution > 100.0 {
            return false;
        } 
        let x = (a * self.A.X) + (solution * self.B.X);
        let y = (a * self.A.Y) + (solution * self.B.Y);
        return self.Prize.X == x && self.Prize.Y == y;
    }

    fn test2(&self, solution: f64) -> bool{
        let a: f64 = ((self.Prize.Y - self.B.Y * solution) / self.A.Y);
        if a != a.round() {
            return false;
        }
        let x = (a * self.A.X) + (solution * self.B.X);
        let y = (a * self.A.Y) + (solution * self.B.Y);
        return self.Prize.X == x && self.Prize.Y == y;
    }

    fn tokens(&self, solution: f64) -> f64 {
        let a: f64 = ((self.Prize.Y - self.B.Y * solution) / self.A.Y);
        return (a * 3.0) + solution;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut sum: f64 = 0.0;
    let mut sum2: f64 = 0.0;
    let r_button_a = Regex::new(r"^Button A: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
    let r_button_b = Regex::new(r"^Button B: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
    let r_score = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)$").unwrap();
    let mut games: Vec<Game> = Vec::new();
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        //remove lines till the end
        let mut lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
        while lines.len() > 0 {
            let mut line = lines.remove(0);
            let button_a = Button {
                X: r_button_a
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap(),
                Y: r_button_a
                .captures(&line)
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap(),
            };
            let mut line = lines.remove(0);
            let button_b = Button {
                X: r_button_b
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap(),
                Y: r_button_b
                .captures(&line)
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap(),
            };
            let mut line = lines.remove(0);
            let score = Score {
                X: r_score
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap(),
                Y: r_score
                .captures(&line)
                .unwrap()
                .get(2)
                .unwrap()
                .as_str()
                .parse::<f64>()
                .unwrap(),
            };
            let mut line = lines.remove(0);
            games.push(Game {A: button_a, B: button_b, Prize: score});


        }
    }

    let games2 = games.clone();
    for game in games {
        //dbg!("{}", &game);
        let game_sol = game.solution();
        if game.test(game_sol) {
            println!("{} is a solution", game_sol);
            sum += game.tokens(game_sol);
         }
    }

    for gameuh in games2 {
        let mut game = gameuh.clone();
        game.Prize.X += 10000000000000.0;
        game.Prize.Y += 10000000000000.0;
        //dbg!("{}", &game);
        let game_sol = game.solution();

        if game.test2(game_sol) {
            println!("{} is a solution", game_sol);
            sum2 += game.tokens(game_sol);
         }
    }

    println!("Sum is :{}", sum);
    println!("Sum2 is :{}", sum2);
}

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
