# QNano Language Documentation

QNano is a lightweight assembly-style language for defining 2-qubit quantum circuits.

## File Format

- Files use the `.qnano` extension
- One instruction per line
- Case-insensitive (`H 0` = `h 0`)
- Lines starting with `#` are comments
- Blank lines are ignored

## Qubits

QNano simulates 2 qubits indexed as **0** and **1**.

## Single-Qubit Gates

**Syntax:** `gate qubit`

| Gate | Description |
|------|-------------|
| `h` | Hadamard - creates superposition |
| `x` | Pauli-X - bit flip (NOT gate) |
| `z` | Pauli-Z - phase flip |
| `s` | S-gate - 90° phase shift |
| `t` | T-gate - 45° phase shift |

**Example:**
```
h 0    # Superposition on qubit 0
x 1    # Flip qubit 1
```

## Two-Qubit Gates

| Gate | Syntax | Description |
|------|--------|-------------|
| `cx` | `cx control target` | Controlled-NOT - flips target if control is \|1⟩ |
| `cz` | `cz qubit_a qubit_b` | Controlled-Z - phase flip on \|11⟩ state |

**Example:**
```
h 0
cx 0 1    # Create Bell state
```

## Running QNano

```bash
qnano your_file.qnano
```

**Output:** Final state vector showing complex amplitudes and probabilities for all four basis states (|00⟩, |01⟩, |10⟩, |11⟩).