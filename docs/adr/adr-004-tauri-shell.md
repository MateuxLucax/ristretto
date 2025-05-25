# ADR 4: Tauri for Desktop Application Shell

Describe here the forces that influence the design decision, including technological, cost-related, and project local.

- Need for a secure, lightweight, and cross-platform desktop application shell.
- Desire to avoid the overhead of Electron.
- Integration with Rust backend and Svelte frontend.

## Decision

We will use Tauri as the application shell to package and run the Svelte frontend and Rust backend as a desktop application.

## Rationale

- Tauri provides a secure, lightweight, and efficient shell for desktop apps.
- Tauri integrates natively with Rust and supports modern frontend frameworks.
- Alternatives (Electron, native toolkits) were rejected due to size, security, or complexity.

## Status

Accepted

## Consequences

- Small, secure, and cross-platform desktop application.
- Some learning curve for Tauri-specific APIs and configuration.
- Efficient integration of Rust and Svelte components.
