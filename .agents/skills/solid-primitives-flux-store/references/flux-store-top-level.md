# @solid-primitives/flux-store (Top-level page notes)

Source page: https://primitives.solidjs.community/package/flux-store/

## Package scope

- Library for creating Solid stores with explicit getters for reads and actions for writes.
- Primary exports on the top-level page:
- `createFluxStore`
- `createFluxStoreFactory`

## Installation

- Package name: `@solid-primitives/flux-store`

## API notes from top-level docs

### `createFluxStore`

- Creates a FluxStore instance that implements explicit getters and actions.
- Accepts two arguments:
- `initialState`: initial store state object.
- `createMethods`: object that defines:
- `getters`: functions that read from the state.
- `actions`: untracked and batched functions that update the state.

### `createFluxStoreFactory`

- Creates a FluxStore encapsulated in a factory function for reusable store implementations.
- Factory can be called with an optional override of the initial state.

## Guidance constraints for this skill

- Restrict guidance to top-level package page content.
- Include top-level usage examples for implementation guidance.
