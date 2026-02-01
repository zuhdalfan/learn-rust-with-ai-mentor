# Rust Product Engineering Roadmap

Goal: We will go through this together until you master every skill in this roadmap. Each phase ends with a tangible outcome and a simple mastery check.

How we will work:
- Learn by building: every phase ships a small project or feature.
- Mastery checks: short quizzes, code reviews, and refactors.
- Portfolio focus: each milestone produces a demo, repo, or write-up.
- Feedback loop: we adapt scope based on your pace and interests.

## Phase Summary (Quick Reference)

| Phase | Focus Area | Duration | Key Deliverable |
|-------|-----------|----------|-----------------|
| 0 | Setup & Workflow | 1 week | Clean repo with CI/CD |
| 1 | Core Rust Foundations | 2-3 weeks | 3 CLI utilities |
| 2 | Async & Networking | 2-3 weeks | Async TCP/HTTP service |
| 3 | Web Back-End | 3-4 weeks | CRUD API with auth |
| 4 | Data & Databases | 3-4 weeks | Data service with migrations |
| 5 | Full-Stack & Client | 3-4 weeks | Full-stack product |
| 6 | WASM & Graphics | 2-3 weeks | WASM module demo |
| 7 | DevOps & Cloud | 3-4 weeks | Deployed service (IaC) |
| 8 | Observability | 2-3 weeks | Monitored live service |
| 9 | Data Streaming | 2-3 weeks | Streaming pipeline |
| 10 | AI Integration | 3-4 weeks | Generative AI feature |
| 11 | Advanced Rust | Ongoing | Optimized real system |

## Where Am I? (Self-Assessment)

**New to programming?**
- Start with Phase 0 (setup)
- Budget 4-6 weeks for Phase 1
- Focus heavily on ownership concepts

**Some programming experience (Python, JavaScript, etc.)?**
- Skim Phase 0, start Phase 1
- Budget 2-3 weeks for Phase 1
- Ownership will be the main challenge

**Experienced developer (C++, Go, etc.)?**
- Review Phase 1 fundamentals quickly
- Consider jumping to Phase 2 after mastery check
- Budget 1-2 weeks for Phase 1 review

**Already know some Rust?**
- Take the Phase 1 mastery check immediately
- If you pass, jump to Phase 2 or your area of interest
- Use this roadmap for advanced topics (Phases 6-11)

## Phase 0: Setup and Workflow (1 week)
Outcomes:
- Solid Rust toolchain and editor setup
- Project scaffolding and repo hygiene
Mastery check:
- Create a clean repo with CI, formatting, and linting

Checklist:
- Rust toolchain, rustup, cargo, clippy, rustfmt
- Git workflow: branching, conventional commits
- CI basics: GitHub Actions

## Phase 1: Core Rust Foundations (2-3 weeks, ~40-60 hours)

**Outcomes:**
- Confident Rust syntax, ownership, lifetimes
- Strong standard library knowledge

**Mastery Check:**
- Build 3 small CLI utilities demonstrating ownership, error handling, and iterators
- Explain why your code compiles (or doesn't) in terms of ownership rules
- Code review: receive only minor feedback on idiomatic patterns

**High-Level Checklist:**
- Ownership, borrowing, lifetimes
- Traits, generics, enums, pattern matching
- Error handling: Result, anyhow, thiserror
- Collections, iterators, closures
- Modules, crates, docs, tests

### Week 1: Ownership & Core Syntax
**Learning Objectives:**
- Understand move semantics and ownership transfer
- Master borrowing rules (mutable vs. immutable)
- Work with basic types, enums, and pattern matching

**Exercises (existing):**
- **Day 1:** Ownership explorer ([`src/bin/day1_exercise.rs`](src/bin/day1_exercise.rs))
  - Move semantics, copy vs. clone
  - Basic data types and variables
- **Day 2:** Borrowing and references ([`src/bin/day2_exercise.rs`](src/bin/day2_exercise.rs))
  - Mutable and immutable borrows
  - The borrow checker in action

**Project Ideas:**
- Simple text parser (validates parentheses/brackets)
- Temperature converter with unit enums
- Basic inventory system (demonstrates ownership transfer)

**Key Concepts:**
- Stack vs. heap allocation
- Copy trait vs. Clone
- References and dereferencing
- Scope and drop

### Week 2: Collections, Iterators, Error Handling
**Learning Objectives:**
- Work fluently with Vec, HashMap, HashSet
- Chain iterator adapters (map, filter, collect)
- Handle errors with Result and Option

**Exercises (existing):**
- **Day 3:** Word-frequency counter ([`src/bin/day3_exercise.rs`](src/bin/day3_exercise.rs))
  - HashMap for counting
  - Iterator chains (split, filter, map)
  - stdin handling with error handling

**Project Ideas:**
- CSV parser with error handling
- JSON validator (basic structure checking)
- Todo list CLI (add, list, remove, persistence)

**Key Concepts:**
- Iterator adapters and combinators
- Result and Option propagation with `?`
- Error types: anyhow for applications, thiserror for libraries
- Collection performance characteristics

### Week 3: Traits, Generics, Testing, Modules
**Learning Objectives:**
- Implement and use traits (Display, Debug, custom traits)
- Write generic functions and structs
- Organize code into modules
- Write unit and integration tests

**Project Ideas:**
- Generic data structure (stack, queue, or cache)
- File processor with custom error types
- CLI app with subcommands (using clap or similar)
- Text-based game (demonstrates traits for game entities)

**Key Concepts:**
- Trait bounds and where clauses
- Trait objects (dyn Trait) vs. generics
- Module system (mod, pub, use)
- Testing with #[test] and #[cfg(test)]
- Documentation with ///

**Mastery Check Details:**
- Submit your 3 CLI projects for review
- Explain the ownership flow in one of your programs
- Refactor a provided snippet to be more idiomatic
- Pass: clean compilation, no major code smells, can explain design choices

## Phase 2: Async Rust and Networking (2-3 weeks)
Outcomes:
- Async services with tokio/actix
- Network protocol fluency
Mastery check:
- Build a small async TCP/HTTP service

Checklist:
- tokio runtime, async/await, channels
- actix or axum service patterns
- WebSocket basics
- gRPC with tonic
- QUIC basics
- LibP2P and WebRTC overview

## Phase 3: Web Back-End Services (3-4 weeks)
Outcomes:
- Production-grade REST and GraphQL APIs
Mastery check:
- Build a CRUD API with auth

Checklist:
- actix-web, axum, Rocket, hyper
- REST design, validation, pagination
- GraphQL basics
- Auth: JWT, OAuth 2.0
- Serde for JSON and config

## Phase 4: Data and Databases (3-4 weeks)
Outcomes:
- Data persistence across SQL and NoSQL
Mastery check:
- Build a data service with migrations and tests

Checklist:
- sqlx, diesel, rusqlite
- PostgreSQL, MySQL, SurrealDB, Firestore
- Migrations, indexing, transaction patterns
- Columnar DB overview: ClickHouse, Snowflake, BigQuery, Redshift

## Phase 5: Full-Stack and Client Apps (3-4 weeks)
Outcomes:
- Ship UI-backed apps using Rust
Mastery check:
- Build a small full-stack product

Checklist:
- Tauri desktop + mobile
- Dioxus, Leptos, Yew
- SWC plugins basics
- Web-to-app packaging

## Phase 6: WebAssembly and Graphics (2-3 weeks)
Outcomes:
- High-performance web modules
Mastery check:
- Build a WASM module and demo

Checklist:
- WASM build pipeline
- wgpu rendering basics
- rust-gpu and shader basics
- Simple SDL or parser design

## Phase 7: DevOps and Cloud (3-4 weeks)
Outcomes:
- Production deployment readiness
Mastery check:
- Deploy a service with infra-as-code

Checklist:
- Docker, Kubernetes
- Terraform
- Jenkins, GitHub CI/CD
- AWS, GCP, Azure

## Phase 8: Observability and Reliability (2-3 weeks)
Outcomes:
- Monitor, trace, and debug in production
Mastery check:
- Add observability to a live service

Checklist:
- Prometheus, Grafana
- ELK stack
- Logging, metrics, tracing patterns

## Phase 9: Data Streaming and Big Data (2-3 weeks)
Outcomes:
- Stream-based systems and pipelines
Mastery check:
- Build a small streaming pipeline

Checklist:
- Kafka, RabbitMQ
- Hadoop, Spark overview

## Phase 10: AI Integration and LLMs (3-4 weeks)
Outcomes:
- Ship AI-powered features responsibly
Mastery check:
- Build a Generative AI feature into a product

Checklist:
- OpenAI, Anthropic, Hugging Face APIs
- OpenLLaMA, BERT, Mistral, Candle Transformers
- Prompting, retrieval, and evaluation

## Phase 11: Advanced Rust Engineering (ongoing)
Outcomes:
- Deep Rust expertise for performance and scale
Mastery check:
- Code review and optimization of a real system

Checklist:
- Declarative and procedural macros
- Code generation
- Performance profiling and tuning
- Custom data structures (multidimensional metrics)

## Capstone Track (choose 2-3)
Pick projects to prove mastery across the roadmap:
- AI-powered SaaS in Rust
- High-performance API platform
- Cross-platform app with Tauri + WebAssembly
- Real-time collaboration app (WebRTC + WebSocket)
- Distributed system using LibP2P or QUIC

## Mastery Criteria
You have mastered the roadmap when you can:
- Design and ship a Rust product end-to-end
- Explain trade-offs in architecture, data, and runtime
- Pass a code review with minimal feedback
- Mentor others on Rust best practices

## Progress Tracking Template

Copy this template to track your journey through the roadmap:

```markdown
# My Rust Learning Progress

**Start Date:** [YYYY-MM-DD]
**Current Phase:** [Phase number]

## Phase 0: Setup ✅/❌
- [ ] Rust toolchain installed
- [ ] Editor configured
- [ ] First repo with CI created
- [ ] Date completed: ___________

## Phase 1: Core Foundations ✅/❌
- [ ] Week 1: Ownership exercises complete
- [ ] Week 2: Collections & iterators exercises complete
- [ ] Week 3: Traits & testing exercises complete
- [ ] CLI Utility 1: ___________
- [ ] CLI Utility 2: ___________
- [ ] CLI Utility 3: ___________
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 2: Async & Networking ✅/❌
- [ ] Async TCP service built
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 3: Web Back-End ✅/❌
- [ ] CRUD API with auth built
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 4: Data & Databases ✅/❌
- [ ] Data service with migrations built
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 5: Full-Stack ✅/❌
- [ ] Full-stack product built
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 6: WASM & Graphics ✅/❌
- [ ] WASM module demo built
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 7: DevOps & Cloud ✅/❌
- [ ] Service deployed with IaC
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 8: Observability ✅/❌
- [ ] Observability added to live service
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 9: Data Streaming ✅/❌
- [ ] Streaming pipeline built
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 10: AI Integration ✅/❌
- [ ] Generative AI feature built
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Phase 11: Advanced Rust ✅/❌
- [ ] System optimization completed
- [ ] Mastery check passed
- [ ] Date completed: ___________

## Capstone Projects
- [ ] Project 1: ___________
- [ ] Project 2: ___________
- [ ] Project 3 (optional): ___________

**Notes & Reflections:**
[Your thoughts on challenges, breakthroughs, favorite projects, etc.]
```

## Next Step
Tell me your current level and which phase you want to start with. We will tailor the pace and projects to you.

For setup guidance, see [`README.md`](./README.md).
