# QNano Language Documentation

This document describes the syntax and usage of the `.qnano` language, a lightweight assembly-style language for defining 2-qubit quantum circuits.

---

## 1. File Structure
QNano files are plain text files with a `.qnano` extension.
* Each instruction must be on a **new line**.
* Instructions are **case-insensitive** (e.g., `H 0` and `h 0` are identical).
* Blank lines and lines starting with `#` (comments) are ignored.

---

## 2. Qubit Addressing
QNano simulates a 2-qubit system. Qubits are zero-indexed:
* **Qubit 0**: The first qubit.
* **Qubit 1**: The second qubit.

Attempting to address a qubit index other than `0` or `1` will result in an error during execution.

---

## 3. Single-Qubit Gates
These gates act on one specific qubit. 
**Syntax:** `[gate_name] [qubit_index]`

| Command | Gate Name | Description |
| :--- | :--- | :--- |
| `h` | Hadamard | Places the qubit into a superposition of 0 and 1. |
| `x` | Pauli-X | Performs a bit-flip (Quantum NOT gate). |
| `z` | Pauli-Z | Performs a phase-flip (flips the sign of the $|1\rangle$ state). |
| `s` | S-Gate | Applies a 90-degree phase shift ($i$). |
| `t` | T-Gate | Applies a 45-degree phase shift ($e^{i\pi/4}$). |

**Example:**
```text
h 0    # Put Qubit 0 in superposition
x 1    # Flip Qubit 1 to the |1> state