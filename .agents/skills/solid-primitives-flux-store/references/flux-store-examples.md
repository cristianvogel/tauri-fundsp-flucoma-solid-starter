# @solid-primitives/flux-store (Top-level usage examples)

Source page: https://primitives.solidjs.community/package/flux-store/

These examples are adapted from the top-level docs and keep the same API intent.

## 1) `createFluxStore` counter

```ts
import { createFluxStore } from "@solid-primitives/flux-store";

const counterState = createFluxStore(
  // initial state
  {
    value: 5,
  },
  {
    // reads
    getters: state => ({
      count() {
        return state.value;
      },
    }),
    // writes
    actions: setState => ({
      increment(by = 1) {
        setState("value", p => p + by);
      },
      reset() {
        setState("value", 0);
      },
    }),
  },
);

// read
counterState.getters.count(); // => 5

// write
counterState.actions.increment();
counterState.getters.count(); // => 6
```

## 2) `createFluxStoreFactory` reusable toggle store

```ts
import { createFluxStoreFactory } from "@solid-primitives/flux-store";

const createToggleState = createFluxStoreFactory(
  // initial state
  {
    value: false,
  },
  // reads
  getters: state => ({
    isOn() {
      return state.value;
    },
  }),
  // writes
  actions: setState => ({
    toggle() {
      setState("value", p => !p);
    },
  }),
);

// state factory can be reused in different components
const toggleState = createToggleState(
  // initial state can be overridden
  { value: true },
);

// read
toggleState.getters.isOn(); // => true

// write
toggleState.actions.toggle();
toggleState.getters.isOn(); // => false
```
