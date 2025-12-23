# :atom_symbol: QNano 

![Overview](/pictures/new_overview.png)

QNano is a lightweight two-qubit quantum circuit simulator and gate compiler written in Rust. It's designed for rapid prototyping and provides a straightforward look into state-vector manipulation.

## :zap: Key Features
* **Universal quantum gates:** It can perform any quantum computation possible on two qubits. See [supported gates](#supported-gates).

* **Four state vectors:** Tracks ∣00⟩, ∣01⟩, ∣10⟩, ∣11⟩.

* **Complex phases:** Uses [Complex64](https://docs.rs/num-complex/latest/num_complex/struct.Complex.html) to track amplitudes.

* **Entanglement Support:** Full CX gate implementation allows for the creation of entangled qubits.

* **Custom .qnano DSL:** A simple text-based assembly language for defining and loading quantum circuits.

## :file_folder: Installation

Install directly from GitHub using Cargo:

```
cargo install --git https://github.com/Cqsi/qnano
```

Once installed, you can run the simulator from any directory:
```
qnano bell_state.qnano
```
The example above uses the `bell_state.qnano` file that is [found in the project root](bell_state.qnano).

## :books: Example

Using the file `bell_state.qnano`, we can produce [Bell's State](https://en.wikipedia.org/wiki/Bell_state), which represent one of the simplest examples of **quantum entanglement**.

The code file reads:
```
h 0
cx 0 1
```

This code translates to the following:

1. We first apply the Hadamard Gate on qubit 0, which puts the qubit in a superposition state. 

2. After this we apply CNOT (Controlled-NOT), which uses qubit 0 as a control and qubit 1 as a target. This is the same as saying "If qubit 0 is 1, flip qubit 1."

This results in the famous Bell State, where the qubits are now entangled. The final state has the following amplitudes:

∣00⟩: 0.707
∣11⟩: 0.707

Let's say we would measure qubit 0 and find that it has value 0, then qubit 1 must be 0 as well. Same goes the other way around when measuring the qubit to be 1. These qubits are no longer two independent bits; they are *entangled*. Even if you took these two qubits to opposite sides of the galaxy, the moment you look at one, the other instantly adopts the same value

## :key: Supported gates

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

## :crystal_ball: Future improvements

Currently a work in progress.

* Ability to change initial state
* Implement Measure-gate and history
* Quantum circuit visualization using Ratatui (see [archive_files](/archive_files/))
* Writing qnano programs in the console
* Error messages

## :pencil2: Blog post
Blog post is coming soon.