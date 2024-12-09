use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Calculus {
    result: i64,
    //numbers: Vec<i64>,
    //solutions: Vec<i64>,
    numbers: Vec<String>,
    solutions_string: Vec<String>,
    solutions: Vec<i64>,
}


impl Calculus {
    fn is_valid(&self) -> bool {
        return self.solutions.iter().any(|i| *i == self.result);
    }
}

//fn solutions(list: Vec<i64>) -> Vec<i64> {
fn solutions(list: Vec<String>, op: Vec<String>) -> Vec<String> {
    let mut mylist = list.clone();
    if list.len() == 2 {

        let mut result: Vec<String> = Vec::new();
        for i in &op {
            result.push(format!("{},{},{}", list[0], i, list[1]));
        }

        return result;
    } else {
        let el = mylist.remove(0);
        let mut result: Vec<i64> = Vec::new();
        let mut result: Vec<String> = Vec::new();
        for i in solutions(mylist, op.clone()) {
            //result.push(el * i);
            //result.push(el + i);
            for j in &op {
                result.push(format!("{},{},{}", el, j, i));
            }
        }
        return result;
    }
}

fn calculate(math: String) -> i64 {
    let mut table: Vec<&str> = math.split(",").collect();

    let mut first: String = table.remove(0).to_string();
    print!(".");
    while table.len() > 0 {
        let op = table.remove(0);
        let second = table.remove(0).to_string();
        match op {
            "+" => first = format!("{}", first.parse::<i64>().unwrap() + second.parse::<i64>().unwrap()),
            "*" => first = format!("{}", first.parse::<i64>().unwrap() * second.parse::<i64>().unwrap()),
            "||" => first = format!("{}{}",first,second),
            _ => break,
        }
    }
    return first.parse::<i64>().unwrap();

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    let mut calcul: Vec<Calculus> = Vec::new();
    // parse using filename
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let results: Vec<&str> = line.split(':').collect();
            let result = results[0].to_string().parse::<i64>().unwrap();
            let numbers: Vec<i64> = results[1]
                .split_whitespace()
                .rev()
                .map(|i| i.parse::<i64>().unwrap())
                .collect();
            let numbers: Vec<String> = results[1]
                .split_whitespace()
                .map(|i| i.to_string())
                .collect();

            let solutions = solutions(numbers.clone(), vec!["+".to_string(),"*".to_string(),"||".to_string()]);
            calcul.push(Calculus {
                result: result,
                numbers: numbers.clone(),
                solutions_string: solutions.clone(),
                solutions: solutions.iter().map(|i| calculate(i.to_string())).collect(),
            });
        }
    }
    //dbg!("Calculus: {}", calcul);
    //println!("{}", calculate("20,||,16,||,6,||,11".to_string()));
    for i in calcul {
        if i.is_valid() {
            sum += i.result
        }
    };

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
