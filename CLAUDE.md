# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Purpose

This is a progressive Rust learning repository structured around daily exercises in Phase 1 (Core Rust Foundations). The repo follows a mentored workflow where each exercise builds toward mastery checks defined in `ROADMAP.md`. Currently focused on ownership, borrowing, traits, enums, error handling, collections, and iterators.

## Build and Run Commands

Build the project:
```bash
cargo build
```

Run the default binary (src/main.rs):
```bash
cargo run
```

Run a specific daily exercise:
```bash
cargo run --bin day1_exercise
cargo run --bin day2_exercise
cargo run --bin day3_exercise
```

For day3_exercise (word-frequency CLI), pipe input:
```bash
echo "some sample text here" | cargo run --bin day3_exercise
```

Format code:
```bash
cargo fmt
```

Run linter:
```bash
cargo clippy
```

Run tests:
```bash
cargo test
```

Run tests for a specific binary:
```bash
cargo test --bin day3_exercise
```

Run tests and show output (including println! from tests):
```bash
cargo test -- --nocapture
```

## Code Architecture

**Project Structure:**
- `src/main.rs` - Primary binary entry point (minimal "Hello, world!" placeholder)
- `src/bin/` - Daily exercise binaries (day1_exercise.rs, day2_exercise.rs, day3_exercise.rs, etc.)
- `ROADMAP.md` - Phase-by-phase learning outcomes and mastery checks (11 phases total)
- `MENTOR_PROFILE.md` - AI mentor technical expertise and learning approach
- `README.md` - Repository introduction, quick start, and development tips

**Exercise Pattern:**
Each daily exercise in `src/bin/` is a standalone binary that demonstrates specific Rust concepts. Day 3 example shows:
- HashMap usage for word counting
- Iterator adapters (split, filter, map, collect)
- Error handling with Result
- Safe input handling from stdin
- Inline comments explaining each logical step

**Learning Progression:**
The codebase follows `ROADMAP.md` phases. Phase 1 (current) focuses on core Rust fundamentals. Future phases will add async (tokio/actix), web backends, databases, full-stack, WASM, DevOps, and AI integration.

## Debugging

Enable backtrace for better error diagnostics:
```bash
RUST_BACKTRACE=1 cargo run --bin day3_exercise
```

Full backtrace with all details:
```bash
RUST_BACKTRACE=full cargo run --bin day3_exercise
```

Verbose output to see what cargo is doing:
```bash
cargo run --verbose
```

**Debugging in Code:**
- Use `dbg!(variable)` macro instead of `println!` - it prints the file, line, and value
- Use `println!("{:?}", variable)` for Debug trait output
- Use `println!("{:#?}", variable)` for pretty-printed Debug output

## Common Rust Errors and Solutions

When helping learners with errors, explain the underlying ownership/borrowing concept, not just the fix.

**"cannot move out of borrowed content" / "cannot move out of `*variable`"**
- **Cause:** Trying to move ownership from a borrowed reference
- **Solution:** Either clone the data, restructure to avoid borrowing, or use references throughout
- **Teaching moment:** Explain difference between moving and borrowing

**"use of moved value"**
- **Cause:** Trying to use a value after ownership was transferred
- **Solution:** Clone before the move, use references, or restructure code flow
- **Teaching moment:** Draw the ownership transfer diagram

**"cannot borrow as mutable because it is also borrowed as immutable"**
- **Cause:** Violating the "one mutable OR multiple immutable borrows" rule
- **Solution:** Reduce borrow scope, restructure to separate mutable/immutable phases
- **Teaching moment:** Explain why Rust prevents data races at compile time

**"lifetime may not live long enough"**
- **Cause:** Reference outlives the data it points to
- **Solution:** Adjust lifetime annotations, restructure ownership, or use owned data
- **Teaching moment:** Explain why Rust prevents dangling pointers

**"trait bound not satisfied"**
- **Cause:** Type doesn't implement a required trait
- **Solution:** Implement the trait, use a different type, or add trait bounds correctly
- **Teaching moment:** Explain trait system and why bounds are needed

## Exercise Creation Guidelines

When creating new daily exercises (day4, day5, etc.):

**File naming:**
- Place in `src/bin/`
- Name as `dayN_exercise.rs` where N is the day number
- Each file is a complete standalone binary

**Code structure:**
```rust
// Brief description of what this exercise teaches
// Example: Day 4 - Error Handling with Result and Option

use std::io;
// ... other imports

fn main() {
    // Inline comments explaining each concept
    // Focus on "why" not just "what"
}

// Additional functions with clear names
// Comments explaining the learning objective
```

**Comment style for learning:**
- Explain why Rust requires certain patterns (ownership, borrowing)
- Point out idioms: "This is idiomatic Rust because..."
- Note alternatives: "You could also do X, but Y is preferred because..."
- Explain error handling: "Using ? here propagates the error to the caller"

**Testing:**
- Add `#[cfg(test)]` module if testing is part of the lesson
- Show how to test error cases, not just happy paths

## Phase Transition Notes

**When learners are ready to move from Phase 1 to Phase 2:**
- They can build CLI tools without consulting docs for basic patterns
- They explain ownership/borrowing issues without prompting
- Code reviews result in only minor style feedback
- They've completed 3 CLI utilities demonstrating mastery criteria

**Assessing mastery completion:**
- Ask learner to explain why code compiles (or doesn't)
- Request verbal walkthrough of ownership flow in their code
- Review code for common anti-patterns (excessive cloning, unwrap abuse, etc.)
- Ensure they can read and fix compiler errors independently

**What to emphasize before Phase 2:**
- Async is just ownership with added complexity
- Futures and Pin are ownership concepts applied to time
- Error handling patterns from Phase 1 apply to async code
- Testing async code requires runtime awareness

## Development Notes

- Each exercise should be self-contained and runnable independently
- Inline comments are used extensively to explain concepts for learning purposes
- The mentored workflow emphasizes: implement → review → refactor → mastery check
- Code should demonstrate idiomatic Rust patterns appropriate to the current phase
- No external dependencies currently (Cargo.toml shows empty dependencies)
- As learners progress, encourage extracting shared code into library modules (`src/lib.rs`)
