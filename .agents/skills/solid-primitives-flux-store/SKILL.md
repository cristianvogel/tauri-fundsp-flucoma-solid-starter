---
name: solid-primitives-flux-store
description: SolidJS Flux Store primitives workflow for @solid-primitives/flux-store. Use when implementing, reviewing, or fixing store patterns with explicit getters and actions, including reusable store factories.
---

# Solid Primitives Flux Store

Use this skill to implement state management with `@solid-primitives/flux-store` using the package's top-level documentation and examples.

## Workflow

1. Read `references/flux-store-top-level.md` before writing code.
2. Read `references/flux-store-examples.md` when you need a concrete pattern.
3. Use `createFluxStore` for a single store instance with explicit getters and actions.
4. Use `createFluxStoreFactory` to create reusable store factories with overridable initial state.
5. Keep reads in getters and writes in actions, and rely on batched/untracked action behavior as documented.

## Constraints

- Use only information from the top-level package page.
- Ignore demo/live-site links for implementation decisions.
