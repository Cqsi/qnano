use std::env;
use std::fs;

fn main() {
    
    let filename = "code.qnano"
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    println!("With text:\n{contents}\n");

    let mut instructions = parse_program(&contents);
    
}

enum Gate {
    H(usize),
    X(usize),
    CX(usize, usize)
}

fn parse_program(contents: &str) -> Vec<Gate> {
    let mut instructions: Vec<Gate> = Vec::new();

    for line in contents.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() { 
            continue; 
        }

        match tokens[0] {
            "h" => instructions.push(Gate::H(tokens[1].parse().unwrap())),
            "x" => instructions.push(Gate::X(tokens[1].parse().unwrap())),
            "cx" => instructions.push(Gate::CX(tokens[1].parse().unwrap(), tokens[2].parse().unwrap())),
            _ => print!("Unknown gate: {}", tokens[0]),
        }
    }

    return instructions;
}