# ADR 2: Rust for Backend Logic

Describe here the forces that influence the design decision, including technological, cost-related, and project local.

- Need for high performance, safety, and concurrency in database operations.
- Desire for robust type checking and memory safety.
- Integration with Tauri for desktop application backend.

## Decision

We will use Rust for all backend logic, including database connection management, query execution, streaming, and type checking.

## Rationale

- Rust's performance and safety are ideal for handling database operations and large data sets.
- Rust integrates natively with Tauri, enabling secure communication with the frontend.
- Alternatives (Node.js, Python, Go) were rejected due to lower performance, less safety, or less seamless Tauri integration.

## Status

Accepted

## Consequences

- High performance and safety for backend operations.
- Some learning curve for Rust development.
- Strong type guarantees and error handling.
