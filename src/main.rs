use std::env;
use std::fs;

fn main() {
    
    let filename = "code.qnano"
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file.");

    println!("With text:\n{contents}")
}

enum Gate {
    H(usize),
    X(usize),
    CX(usize, usize)
}

fn parse_program(contents: &str) -> Vec<Gate> {
    let mut instructions: Vec<Gate> = Vec::new();

    for line in contents.lines() {
        
    }

}