# Development Environment Setup

## Prerequisites

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Install Lean 4
```bash
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh
source ~/.elan/env
```

### Verify Installation
```bash
rustc --version
cargo --version
lean --version
```

## Editor Setup

### VS Code (Recommended)
Install these extensions:
- rust-analyzer
- Lean 4 (leanprover.lean4)
- Better TOML
- GitLens

### Vim/Neovim
- Install rust.vim plugin
- Configure LSP with rust-analyzer
- Install lean.nvim for Lean support

## Building and Running

```bash
# Build entire workspace
cargo build

# Run specific binary
cargo run --bin caesar -- encrypt --shift 3 --text "Hello World"

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Lean Workflow

```bash
# Check Lean proofs
lean --make lean-proofs/

# Interactive theorem proving
lean --server lean-proofs/basic_number_theory.lean
```
