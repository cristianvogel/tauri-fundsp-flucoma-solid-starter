---
name: solid-primitives-keyboard
description: SolidJS keyboard primitives workflow for @solid-primitives/keyboard. Use when implementing, reviewing, or fixing keyboard input behavior in Solid components, including keydown events, held-key tracking, key sequences, key-specific hold state, and shortcut handling.
---

# Solid Primitives Keyboard

Use this skill to implement keyboard behavior with `@solid-primitives/keyboard` using the package's documented APIs only.

## Workflow

1. Read `references/keyboard-top-level.md` before writing code.
2. Select the primitive by behavior:
- Use `useKeyDownEvent` to observe the latest keydown event.
- Use `useKeyDownList` to track currently held keys.
- Use `useCurrentlyHeldKey` when exactly one key must be held.
- Use `useKeyDownSequence` to track press/release order for currently held keys.
- Use `createKeyHold` to expose boolean hold-state for a specific key.
- Use `createShortcut` to run callbacks for key combinations.
3. Preserve documented semantics:
- Treat root singleton primitives as shared listener/signal sources.
- Respect `preventDefault` and `requireReset` options when using shortcut or hold observers.
- Keep shortcut key matching explicit and deterministic.
4. Avoid demo-driven APIs and ignore examples from the docs page.

## Constraints

- Ignore examples and demos from the package docs.
- Use only information from the top-level keyboard package page.
- Prefer package primitives over ad-hoc keyboard listener state logic when behavior matches.
