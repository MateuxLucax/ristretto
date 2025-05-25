# ADR 1: Ristretto Overall Architecture

Describe here the forces that influence the design decision, including technological, cost-related, and project local. 

- Need for a fast, lightweight, and modern SQL client for PostgreSQL.
- Desire to leverage Rust for performance and safety, Svelte for a reactive UI, and Tauri for secure desktop integration.
- Requirement to support multiple database connections, large result sets, and advanced features (AI, autocompletion, history).
- Maintainability, extensibility, and cross-platform support.

## Decision 

We will architect Ristretto as a Tauri desktop application with a Svelte frontend and a Rust backend. The backend will manage database connections, query execution, streaming, and type checking. The frontend will handle UI, state, and API calls to the backend. Data will be streamed for large result sets. AI and advanced features will be modular.

## Rationale

- Rust provides safety, performance, and robust async database libraries.
- Svelte offers a modern, fast, and maintainable UI framework.
- Tauri enables secure, cross-platform desktop apps with a small footprint.
- Streaming and type checking in Rust avoid JS memory issues and ensure correctness.
- Modular design allows for future extensibility (other DBs, more AI, etc).
- Alternatives (Electron, React, Python) were rejected due to performance, size, or maintainability concerns.

## Status

Accepted

## Consequences

- Clear separation of concerns between UI and backend logic.
- Efficient handling of large data sets.
- Secure, cross-platform, and maintainable codebase.
- Some learning curve for Rust/Tauri/Svelte integration.
