use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_from_file_commands(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let code: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    code
}

fn diagnostics(codes: Vec<String>) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let lines = &codes.len();
    let length = codes[0].len();
    let mut result = vec![0; length];
    for code in codes {
        for c in code.chars().enumerate() {
            if c.1 == '1' {
                result[c.0] += 1;
            }
        }
    }
    // println!("{:?}", result);
    for r in 0..result.len() {
        // println!("{} {}", result[r], lines/2);
        if result[r] > lines/2 {
            if r == result.len()-1 {
                gamma += 1;
            } else {
                gamma += 2 << (result.len() - r - 2);
            }
            println!("new gamma: {}", gamma);
        } else {
            if r == result.len()-1 {
                epsilon += 1;
            } else {
                epsilon += 2 << (result.len() - r - 2);
            }
            println!("new epsilon: {}", epsilon);
        }
    }
    println!("Gamma: {} Epislon: {}", gamma, epsilon);
    println!("Power Consumption: {}", gamma * epsilon);
}

fn main() {
    let commands = load_from_file_commands("src/data.txt");
    diagnostics(commands)
}
