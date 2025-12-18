//use std::env;
use std::f32::consts::FRAC_1_SQRT_2;
use std::fs;

fn main() {
    
    let filename = "code.qnano";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    println!("With text:\n{contents}\n");

    //let mut instructions = parse_program(&contents);
    let mut q = QuantumCircuit {
        state: [1.0, 0.0, 0.0, 0.0],
    };

    q.show_state();
    println!("Applying Hadamard Gate: ");
    q.apply_h();
    q.show_state();
    println!("Applying CNOT Gate: ");
    q.apply_cx();
    q.show_state();
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

struct QuantumCircuit {
    state: [f64; 4], // c1|00> c2|01> c3|10> c4|11>
}

impl QuantumCircuit {

    fn new() -> Self {
        return Self {
            state: [1.0, 0.0, 0.0, 0.0],
        }
    }

    // We use an immutable reference since printing the state shouldn't change the state
    fn show_state(&self) {
        let labels = ["|00>", "|01>", "|10>", "|11>"];

        for i in 0..4 {
            println!("{}: {}", labels[i], &self.state[i])
        }
    }

    // HADAMARD GATE
    // We use a mutable reference since the Hadamard Gate mutates the complex numbers in the array.
    fn apply_h(&mut self) {
        let s: f64 = FRAC_1_SQRT_2.into();

        let a0 = self.state[0];
        let b0 = self.state[2];

        // |00> = (|00> + |10>) * 1/sqrt(2)
        self.state[0] = (a0 + b0) * s;

        // |10> = (|00> - |10>) * 1/sqrt(2)
        self.state[2] = (a0 - b0) * s;

        let a1 = self.state[1];
        let b1 = self.state[3];

        self.state[1] = (a1 + b1) * s;
        self.state[3] = (a1 - b1) * s;

    }


    fn apply_cx(&mut self) {
        self.state.swap(2,3);
    }
}