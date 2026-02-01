# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs` is the primary binary entry point.
- `src/bin/` contains additional runnable exercises (e.g., `day1_exercise.rs`).
- `Cargo.toml` defines the crate metadata and dependencies; `Cargo.lock` pins versions.
- Planning and curriculum docs live at `ROADMAP.md` and `CURRICULLUM.md`.
- Build artifacts are under `target/` (ignored by default).

## Build, Test, and Development Commands
- `cargo build` — compile the crate.
- `cargo run` — build and run the default binary (`src/main.rs`).
- `cargo run --bin day1_exercise` — run a specific exercise in `src/bin/`.
- `cargo test` — run tests (none are present yet; see Testing Guidelines).

## Coding Style & Naming Conventions
- Use standard Rust formatting via `rustfmt` (4-space indentation; no tabs).
- Prefer idiomatic Rust naming: `snake_case` for functions/variables, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for constants.
- Keep modules small and focused; place new binaries in `src/bin/<topic>_exercise.rs`.

## Testing Guidelines
- No tests are currently defined. When adding tests, use Rust’s built-in test framework.
- Place unit tests in the same module with `#[cfg(test)]` and `mod tests { ... }`.
- For integration tests, add files under `tests/`.
- Recommended: run `cargo test` before opening a PR.

## Commit & Pull Request Guidelines
- This repository has no commits yet, so there is no established commit message convention.
- Suggested format: short, imperative summary (e.g., "Add day2 exercise") and optional body for context.
- PRs should describe the change, include relevant commands run (e.g., `cargo test`), and link issues if applicable.

## Agent-Specific Instructions
- Keep documentation concise (200–400 words) and repo-specific.
- Avoid editing `target/` outputs; only change source and doc files.
- Before proposing next steps, self-verify progress by reading `ROADMAP.md`, `CURRICULLUM.md`, and existing exercises under `src/bin/` to align with what’s already done.
- Always read `CURRICULLUM.md` and `ROADMAP.md` before starting new work in this repo.
- After reading this file, adopt a Rust mastery mentor role grounded in `ROADMAP.md`: guide tasks toward the current phase outcomes, propose mastery checks when relevant, and keep feedback actionable and encouraging.
- Whenever an AI reads this file, it must act as a Rust expert mentor and immediately start roleplay to help the user master Rust based on `ROADMAP.md`.
