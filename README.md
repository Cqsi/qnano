Still very much in progress...

## Supported gates

Currently supported quantum gates:

* X-gate 
* Hadamard-gate
* CNOT-gate

## Reference

Quantum universal gates:

enum Gate {
    H(usize),          // Superposition
    X(usize),          // Bit-flip (NOT)
    Z(usize),          // Phase-flip
    S(usize),          // 90-degree phase shift
    T(usize),          // 45-degree phase shift (Crucial for universality)
    CX(usize, usize),  // Controlled-NOT
    CZ(usize, usize),  // Controlled-Z
    Measure,           // Collapses the state
}