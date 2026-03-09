# @solid-primitives/keyboard (Top-level page notes)

Source page: https://primitives.solidjs.community/package/keyboard/

## Package scope

- Reactive primitives for handling user keyboard input in SolidJS.
- Top-level exports listed on the package page:
- `useKeyDownEvent`
- `useKeyDownList`
- `useCurrentlyHeldKey`
- `useKeyDownSequence`
- `createKeyHold`
- `createShortcut`

## Installation

- Package name: `@solid-primitives/keyboard`

## API notes from top-level docs

### `useKeyDownEvent`

- Provides a signal with the latest keydown event.
- Described as a singleton root primitive that reuses listeners/signals across dependents.

### `useKeyDownList`

- Provides a signal with currently held keys.
- Keys are ordered from least recent to most recent.
- Described as a singleton root primitive that reuses listeners/signals across dependents.

### `useCurrentlyHeldKey`

- Provides a signal with the currently held single key.
- If another key is pressed simultaneously, signal resets to `null`.
- Described as a singleton root primitive that reuses listeners/signals across dependents.

### `useKeyDownSequence`

- Provides a signal with sequence of currently held keys based on keydown/keyup order.
- Described as a singleton root primitive that reuses listeners/signals across dependents.

### `createKeyHold`

- Provides a boolean signal indicating whether a specified key is currently held.
- Notes state behavior for multiple simultaneous keys versus only the specified key.
- Accepts:
- `key`: keyboard key to observe.
- `options.preventDefault`: whether to call `preventDefault` when key is pressed (default `true`).

### `createShortcut`

- Observes keyboard shortcuts and calls a callback when specified keys are pressed.
- Accepts:
- `keys`: list of keys for the shortcut.
- `callback`: function to run when matched.
- `options.preventDefault`: whether to call `preventDefault` for relevant keydown events (default `true`).
- `options.requireReset`: when `true`, trigger once until all shortcut keys are released.
- Top-level notes include shortcut default-prevention behavior across keydown events that can lead to the full shortcut.

## Guidance constraints for this skill

- Exclude examples and demos from implementation guidance.
- Restrict usage guidance to these top-level API descriptions only.
