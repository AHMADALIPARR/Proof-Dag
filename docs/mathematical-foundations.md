# Mathematical Foundations

## Artin Braid Group $B_n$
The braid group on $n$ strands $B_n$ is presented by generators $\sigma_1, \dots, \sigma_{n-1}$ subject to:
1. $\sigma_i \sigma_j = \sigma_j \sigma_i$ for $|i - j| \ge 2$
2. $\sigma_i \sigma_{i+1} \sigma_i = \sigma_{i+1} \sigma_i \sigma_{i+1}$ (Yang-Baxter equation)

## Exact Grover Search on 2 Qubits
For state vector $|\psi\rangle = \frac{1}{2}(|00\rangle + |01\rangle + |10\rangle + |11\rangle)$, with oracle $O|11\rangle = -|11\rangle$, diffusion $D = 2|s\rangle\langle s| - I$ yields:
$$D O |s\rangle = |11\rangle$$
with exact probability 1.
