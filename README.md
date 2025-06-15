# crypto-learning-lab
*A mathematician's journey into cryptography: theory, implementation, and formal verification*

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Lean](https://img.shields.io/badge/Lean-4-blue?style=for-the-badge)](https://lean-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## 🎯 Mission

This repository documents my 12-month journey from pure mathematics into applied cryptography, emphasizing:
- **Theory to Practice**: Every mathematical concept implemented in Rust
- **Formal Verification**: Critical implementations proven correct in Lean 4
- **Security Awareness**: Constant-time algorithms and side-channel resistance
- **Educational Value**: Clear explanations linking math, code, and proofs

## 📚 Learning Path

### Phase 1 (Months 1-3): Foundations
- [x] Rust fundamentals and ownership model
- [ ] Classical cryptography implementations
- [ ] CryptoPals challenges (Sets 1-2)
- [ ] Basic Lean proofs for number theory

### Phase 2 (Months 4-6): Modern Primitives
- [ ] AES implementation with timing attack resistance
- [ ] SHA-256/SHA-3 hash functions
- [ ] Elliptic curve cryptography
- [ ] Aeneas integration for formal verification

### Phase 3 (Months 7-12): Protocols & Impact
- [ ] TLS protocol implementation
- [ ] Post-quantum cryptography integration
- [ ] Performance optimization studies
- [ ] Community contributions and teaching

## 🗂️ Repository Structure

```
crypto-learning-lab/
├── README.md                   # This file
├── LEARNING_LOG.md            # Weekly progress and reflections
├── Cargo.toml                 # Rust workspace configuration
├── 
├── 01-foundations/            # Phase 1: Basic concepts
│   ├── classical-crypto/      # Caesar, Vigenère, substitution ciphers
│   ├── number-theory/         # Modular arithmetic, GCD, primality
│   ├── cryptopals/           # Solutions to CryptoPals challenges
│   └── lean-proofs/          # Basic mathematical proofs
│
├── 02-primitives/            # Phase 2: Modern cryptographic primitives
│   ├── symmetric/            # AES, ChaCha20, block cipher modes
│   ├── hashing/              # SHA family, BLAKE3, hash constructions
│   ├── asymmetric/           # RSA, ECC, key exchange protocols
│   └── formal-verification/  # Aeneas extractions and Lean proofs
│
├── 03-protocols/             # Phase 3: Complete cryptographic systems
│   ├── tls/                  # Transport Layer Security implementation
│   ├── post-quantum/         # PQC algorithm integrations
│   ├── performance/          # Benchmarking and optimization studies
│   └── educational/          # Teaching materials and visualizations
│
├── docs/                     # Documentation and guides
│   ├── setup.md             # Development environment setup
│   ├── resources.md         # Learning resources and references
│   └── blog-posts/          # Markdown versions of blog content
│
├── tools/                    # Helper scripts and utilities
│   ├── benchmarks/          # Performance measurement tools
│   ├── visualizations/      # Crypto concept visualizations
│   └── testing/             # Security testing utilities
│
└── examples/                 # Complete example applications
    ├── secure-chat/         # End-to-end encrypted messaging
    ├── password-manager/    # Cryptographic key derivation demo
    └── blockchain-demo/     # Simple blockchain with crypto primitives
```

## 🚀 Getting Started

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Lean 4
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh

# Install Aeneas (for formal verification)
git clone https://github.com/AeneasVerif/aeneas
cd aeneas && make install
```

### Quick Start
```bash
git clone https://github.com/[your-username]/crypto-learning-lab
cd crypto-learning-lab

# Run all tests
cargo test

# Run specific examples
cargo run --bin classical-crypto
cargo run --bin cryptopals-set1

# Check Lean proofs
lean --make lean-proofs/
```

## 📖 Blog Integration

Each major milestone corresponds to a blog post series:

1. **"Starting the Journey"** - Why cryptography? Why Rust + Lean?
2. **"From Caesar to AES"** - Evolution of cryptographic thinking
3. **"Proving Code Correct"** - Introduction to formal verification
4. **"Timing is Everything"** - Side-channel attacks and defenses
5. **"Elliptic Curves Explained"** - Math to implementation
6. **"Building Secure Protocols"** - From primitives to systems

Blog posts are drafted in `docs/blog-posts/` before publication.

## 🤝 Contributing

While this is primarily a personal learning repository, contributions are welcome:

- **Bug Reports**: If you find errors in implementations
- **Educational Improvements**: Better explanations or visualizations
- **Additional Examples**: More ways to demonstrate concepts
- **Lean Proofs**: Help with formal verification challenges

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📊 Progress Tracking

- **Commits**: Regular commits with descriptive messages
- **Issues**: Self-assigned tasks and learning goals
- **Milestones**: Major achievements and deadlines
- **Projects**: Kanban boards for each learning phase

## 📚 Resources

### Books
- "The Rust Programming Language" - Klabnik & Nichols
- "Understanding Cryptography" - Paar & Pelzl
- "Cryptography Engineering" - Ferguson, Schneier, Kohno
- "Theorem Proving in Lean 4" - Avigad et al.

### Online Resources
- [CryptoPals Challenges](https://cryptopals.com/)
- [RustCrypto GitHub](https://github.com/RustCrypto)
- [Lean Community](https://leanprover-community.github.io/)
- [Aeneas Documentation](https://github.com/AeneasVerif/aeneas)

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust community for excellent documentation and tooling
- The Lean community for patient help with formal proofs
- CryptoPals creators for hands-on learning challenges
- Open source cryptography researchers and implementers

---

**Latest Update**: [Current Date] - Repository structure created, beginning Phase 1 implementations.

*Follow my progress on [Blog Name] and connect with me on [Twitter/LinkedIn].*
