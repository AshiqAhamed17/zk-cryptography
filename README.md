# zk-cryptography

Cryptography & Zero-Knowledge playground in **Rust** and **Solidity** — elliptic curves, finite fields, and zk primitives.

This repo is my hands-on space to practice and showcase cryptography fundamentals used in **blockchains** and **zero-knowledge proofs**. 
The goal is to build strong proof-of-work by implementing math + code from scratch and experimenting with ZK-friendly constructs.

---

## Contents
- **Elliptic Curves**
  - Point creation, addition, scalar multiplication
  - Rust & Solidity implementations
- **Finite Fields**
  - Modular arithmetic
  - Inverses, exponentiation
- **ZK Building Blocks**
  - Fiat–Shamir heuristic (simulation)
  - Commitment schemes
  - Hashing (Poseidon/Keccak experiments coming soon)

---

## Tech Stack
- **Rust** → low-level cryptography implementations (efficient + safe)
- **Solidity** → Ethereum smart contracts (on-chain primitives & experiments)
- **Foundry** → testing Solidity contracts
- **Cargo** → managing Rust projects

---

## Vision
This repo is meant to grow into:
- A **learning lab** for cryptography + ZK
- A **portfolio of proof-of-work** for security & ZK engineering roles
- A set of **reference implementations** (clean, well-documented, easy to understand)

---

## 🧑‍💻 How to Use
Clone the repo and run the Rust code:
```bash
git clone https://github.com/<your-username>/zk-cryptography.git
cd zk-cryptography
cargo run