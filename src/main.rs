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

    fn evaluate(&mut self, instruction: Vec<Gate>){
        for gate in instructions {
            match gate {
                Gate::H(q) => self.apply_h(),
                Gate::X(q) => self.apply_x(),
                Gate::CX(control, target) => self.apply_cx(control, target),
            }
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
    fn apply_h(&mut self, q: usize) {
    let s = 1.0 / 2.0_f64.sqrt();

    match q {
        0 => {
            // Pair 1: |00> and |10> (Indices 0 and 2)
            let a = self.state[0];
            let b = self.state[2];
            self.state[0] = (a + b) * s;
            self.state[2] = (a - b) * s;

            // Pair 2: |01> and |11> (Indices 1 and 3)
            let c = self.state[1];
            let d = self.state[3];
            self.state[1] = (c + d) * s;
            self.state[3] = (c - d) * s;
        },
        1 => {
            // Pair 1: |00> and |01> (Indices 0 and 1)
            let a = self.state[0];
            let b = self.state[1];
            self.state[0] = (a + b) * s;
            self.state[1] = (a - b) * s;

            // Pair 2: |10> and |11> (Indices 2 and 3)
            let c = self.state[2];
            let d = self.state[3];
            self.state[2] = (c + d) * s;
            self.state[3] = (c - d) * s;
        },
        _ => println!("Error: Qano only supports qubits 0 and 1!"),
    }
}

    fn apply_x(&mut self, q: usize) {
        if q == 0 {
            self.state.swap(0,2);
            self.state.swap(1,3);
        } else if q == 1 {
            self.state.swap(0,1);
            self.state.swap(2,3);
        }
    }

    fn apply_cx(&mut self, control: usize, target: usize) {
        if control == 0 && target == 1 {
            self.state.swap(2,3);
        } else if control == 1 && target == 0 {
            self.state.swap(1,3);
        }
    }
}