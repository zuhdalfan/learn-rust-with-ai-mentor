# Rust Training: Progressive Mastery with an AI Mentor

A structured, hands-on Rust learning repository designed for progressive skill development through daily exercises. Learn by building real projects with guided feedback from an AI mentor.

## 🎯 What You'll Learn

This repository takes you from Rust fundamentals to production-ready engineering across 11 phases:
- **Phase 1 (Current):** Core Rust - ownership, borrowing, lifetimes, traits, error handling
- **Phases 2-11:** Async programming, web services, databases, full-stack, WASM, DevOps, AI integration

See the complete learning path in [`ROADMAP.md`](./ROADMAP.md).

## 🚀 Quick Start

### Prerequisites
- Rust toolchain installed ([rustup.rs](https://rustup.rs))
- Basic command-line familiarity
- A code editor (VS Code, IntelliJ IDEA, or vim)

### Build and Run

```bash
# Build the entire project
cargo build

# Run default binary
cargo run

# Run a specific daily exercise
cargo run --bin day1_exercise
cargo run --bin day2_exercise
cargo run --bin day3_exercise

# Example: Day 3 word-frequency CLI with input
echo "rust is awesome rust is powerful" | cargo run --bin day3_exercise

# Format your code
cargo fmt

# Run linter for code quality
cargo clippy

# Run tests
cargo test
```

## 📚 Repository Structure

```
rust_training/
├── src/
│   ├── main.rs              # Primary binary (minimal starter)
│   └── bin/                 # Daily exercises (standalone binaries)
│       ├── day1_exercise.rs # Basic syntax and ownership
│       ├── day2_exercise.rs # Borrowing and references
│       └── day3_exercise.rs # Collections and iterators
├── ROADMAP.md               # Complete 11-phase learning path
├── CLAUDE.md                # AI mentor guidance (for Claude Code)
└── README.md                # This file
```

## 🎓 Learning Methodology

### Mentored Workflow
1. **Implement** - Write code with guided scaffolding
2. **Review** - Get feedback on idioms and best practices
3. **Refactor** - Improve toward clean, idiomatic Rust
4. **Master** - Complete a mastery check before advancing

### Daily Exercise Pattern
Each exercise in `src/bin/` is:
- **Self-contained** - Runs independently as its own binary
- **Concept-focused** - Targets specific Rust skills
- **Well-commented** - Explains why, not just what
- **Progressive** - Builds on previous days' knowledge

## 📖 Current Focus: Phase 1

**Core Rust Foundations (2-3 weeks)**

**Topics:**
- Ownership, borrowing, and lifetimes
- Traits, generics, and pattern matching
- Error handling with `Result`, `Option`, `anyhow`, `thiserror`
- Collections: `Vec`, `HashMap`, `HashSet`
- Iterators and closures
- Modules, crates, documentation, testing

**Mastery Goal:** Build 3 small CLI utilities demonstrating confident Rust syntax and ownership reasoning.

**Example Projects:**
- **Day 1-2:** Ownership explorer (move semantics, borrowing rules)
- **Day 3:** Word-frequency counter (HashMap, iterators, stdin handling)
- **Day 4+:** File processor, JSON parser, or text-based game

## 🛠️ Development Tips

### Common Commands
```bash
# Check code without building
cargo check

# Build with optimizations
cargo build --release

# Run with arguments
cargo run --bin day3_exercise -- --help

# Watch mode (requires cargo-watch)
cargo watch -x 'run --bin day3_exercise'
```

### Debugging
```bash
# Show detailed backtraces for panics
RUST_BACKTRACE=1 cargo run --bin day3_exercise

# Full backtrace with all details
RUST_BACKTRACE=full cargo run

# Verbose output
cargo run --verbose
```

**Quick debugging tips:**
- Use `dbg!(variable)` instead of `println!` for better debugging output
- The compiler's error messages are your best teacher - read them carefully!

### Troubleshooting
- **Compilation errors:** Read Rust compiler messages carefully - they're excellent!
- **Borrow checker issues:** Review [ownership rules](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- **"cannot move out of borrowed content":** You're trying to move data that's borrowed - try cloning or restructuring
- **"use of moved value":** You've moved ownership elsewhere - consider borrowing with `&` instead
- **Cargo not found:** Ensure Rust toolchain is installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## 🎯 Next Steps

1. **Start Phase 1:** Run `cargo run --bin day1_exercise` to begin
2. **Check your progress:** Review [`ROADMAP.md`](./ROADMAP.md) mastery criteria
3. **Get guidance:** If using Claude Code, it will read `CLAUDE.md` for context
4. **Ask questions:** The AI mentor will explain concepts, review code, and suggest improvements

## 📝 Learning Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)

---

**Goal:** Consistent progress, clean Rust code, and confident reasoning about ownership and data flow.
