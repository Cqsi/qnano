//use std::env;
use std::fs;
use num_complex::Complex64;
use rand::prelude::*;

fn main() {
    
    let filename = "code.qnano";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");
    let instructions = parse_program(&contents);

    println!("With text:\n{contents}\n");

    //let mut instructions = parse_program(&contents);
    let mut circuit = QuantumCircuit::new();
    circuit.show_state();
    println!("Applying gates...");
    circuit.evaluate(instructions);
    circuit.show_state();

    // q.show_state();
    // println!("Applying Hadamard Gate: ");
    // q.apply_h();
    // q.show_state();
    // println!("Applying CNOT Gate: ");
    // q.apply_cx();
    // q.show_state();
}

enum Gate {
    H(usize),
    X(usize),
    Z(usize),
    S(usize),
    T(usize),
    CX(usize, usize),
    CZ(usize),
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
            "z" => instructions.push(Gate::Z(tokens[1].parse().unwrap())),
            "s" => instructions.push(Gate::S(tokens[1].parse().unwrap())),
            "t" => instructions.push(Gate::T(tokens[1].parse().unwrap())),
            "cx" => instructions.push(Gate::CX(tokens[1].parse().unwrap(), tokens[2].parse().unwrap())),
            "cz" => instructions.push(Gate::CZ(tokens[1].parse().unwrap())),
            _ => print!("Unknown gate: {}", tokens[0]),
        }
    }

    return instructions;
}

struct QuantumCircuit {
    state: [Complex64; 4], // c1|00> c2|01> c3|10> c4|11>
}

impl QuantumCircuit {

    fn new() -> Self {
        return Self {
            state: [
                Complex64::new(1.0, 0.0), 
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
        }
    }

    fn evaluate(&mut self, instructions: Vec<Gate>){
        for gate in instructions {
            match gate {
                Gate::H(q) => self.apply_h(q),
                Gate::X(q) => self.apply_x(q),
                Gate::Z(q) => self.apply_z(q),
                Gate::S(q) => self.apply_s(q),
                Gate::T(q) => self.apply_t(q),
                Gate::CX(control, target) => self.apply_cx(control, target),
                Gate::CZ(_q) => self.apply_cz(),
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
            _ => println!("Error: qnano only supports qubits 0 and 1!"),
        }
    }

    fn apply_x(&mut self, q: usize) {
        match q {
            0 => {
                self.state.swap(0,2);
                self.state.swap(1,3);
            },
            1 => {
                self.state.swap(0,1);
                self.state.swap(2,3);
            },
            _ => println!("Error: qnano only supports qubits 0 and 1!"),
        }
    }

    fn apply_z(&mut self, q: usize) {
        match q {
            0 => {
                self.state[2] *= -1.0;
                self.state[3] *= -1.0;
            },
            1 => {
                self.state[1] *= -1.0;
                self.state[3] *= -1.0;
            },
            _ => println!("Error: qnano only supports qubits 0 and 1!"),
        }
    }

    fn apply_s(&mut self, q: usize) {

        let i = Complex64::i();

        match q {
            0 => {
                self.state[2] *= i;
                self.state[3] *= i;
            },
            1 => {
                self.state[1] *= i;
                self.state[3] *= i;
            },
            _ => println!("Error: qnano only supports qubits 0 and 1!"),
        }
    }

    fn apply_t(&mut self, q: usize) {

        let phase = Complex64::from_polar(1.0, std::f64::consts::PI/4.0);

        match q {
            0 => {
                self.state[2] *= phase;
                self.state[3] *= phase;
            },
            1 => {
                self.state[1] *= phase;
                self.state[3] *= phase;
            },
            _ => println!("Error: qnano only supports qubits 0 and 1!"),
        }
    }

    fn apply_cx(&mut self, control: usize, target: usize) {
        if control == 0 && target == 1{
            self.state.swap(2,3);
        } else if control == 1 && target == 0{
            self.state.swap(1,3);
        }
    }

    fn apply_cz(&mut self) {
        self.state[3] *= -1.0
    }

    // fn measure(&mut self) -> {
    //     let mut rng = thread_rng();
    //     let r: f64 = rng.gen();

    //     let mut cumulative 
    // }
}