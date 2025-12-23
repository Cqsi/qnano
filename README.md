# QNano 

QNano is a lightweight two-qubit quantum circuit simulator and gate compiler written in Rust. 

It's designed for rapid prototyping and provides a straightforward look into state-vector manipulation.

## Installation

## Key Features
* **Universal quantum gates:** It can perform any quantum computation possible on two qubits. See [supported gates](#supported-gates).

* **Four state vectors:** Tracks ∣00⟩, ∣01⟩, ∣10⟩, ∣11⟩.

* **Complex phases:** Uses [Complex64](https://docs.rs/num-complex/latest/num_complex/struct.Complex.html) to track amplitudes.

* **Entanglement Support:** Full CX gate implementation allows for the creation of entangled qubits.

* **Custom .qnano DSL:** A simple text-based assembly language for defining and loading quantum circuits.

## Examples 

## Supported gates

Currently supported quantum gates:

* H (Hadamard)

* X (Pauli-X / NOT)

* Z (Pauli-Z / Phase-flip)

* S (Phase / 90° rotation)

* T (T-gate / 45° rotation)

* CX (Controlled-NOT / CNOT)

* CZ (Controlled-Z)

This set of quantum gates is universal, meaning that it can in theory perform any set of quantum computation on two qubits.

See [this visual](pictures/quantum_gates.jpeg) for an overview of support quantum gates.

## Future improvements

Currently a work in progress.

* Ability to change initial state
* Implement Measure-gate and history
* Quantum circuit visualization using Ratatui (see ![archive_files](https://github.com/Cqsi/qnano/tree/master/archive_files))
* Writing qnano programs in the console
* Error messages