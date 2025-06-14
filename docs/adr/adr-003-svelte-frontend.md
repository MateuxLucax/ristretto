# ADR 3: Svelte for Frontend UI

Describe here the forces that influence the design decision, including technological, cost-related, and project local.

- Need for a modern, reactive, and maintainable UI framework.
- Desire for fast development and a small bundle size.
- Integration with Tauri and Rust backend.

## Decision

We will use Svelte for the frontend UI, including all user interface components, state management, and API calls to the backend.

## Rationale

- Svelte provides a simple, reactive, and efficient UI framework.
- Svelte's small bundle size and fast reactivity improve user experience.
- Alternatives (React, Vue, Angular) were rejected due to larger bundle sizes or more complex state management.

## Status

Accepted

## Consequences

- Fast, modern, and maintainable UI.
- Some learning curve for Svelte.
- Easy integration with Tauri and backend APIs.
