# Rust Product Engineering Roadmap

Goal: We will go through this together until you master every skill in this roadmap. Each phase ends with a tangible outcome and a simple mastery check.

How we will work:
- Learn by building: every phase ships a small project or feature.
- Mastery checks: short quizzes, code reviews, and refactors.
- Portfolio focus: each milestone produces a demo, repo, or write-up.
- Feedback loop: we adapt scope based on your pace and interests.

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

## Phase 1: Core Rust Foundations (2-3 weeks)
Outcomes:
- Confident Rust syntax, ownership, lifetimes
- Strong standard library knowledge
Mastery check:
- Build 3 small CLI utilities

Checklist:
- Ownership, borrowing, lifetimes
- Traits, generics, enums, pattern matching
- Error handling: Result, anyhow, thiserror
- Collections, iterators, closures
- Modules, crates, docs, tests

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

## Next Step
Tell me your current level and which phase you want to start with. We will tailor the pace and projects to you.
