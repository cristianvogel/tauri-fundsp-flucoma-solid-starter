# Tauri + flucoma.rs + funDSP + SolidJS Starter

This branch is now a template repository for desktop audio tools built with:

- Tauri 2 on the Rust backend
- `flucoma-rs` wired in as a work-in-progress analysis layer
- `fundsp` wired in as the DSP graph layer
- SolidJS + Vite on the frontend

The starter is intentionally small. It gives you a clean architecture, a runnable shell, and obvious extension points for analysis, playback, project state, and UI features.

## What This Starter Includes

- A minimal SolidJS app that calls into Tauri and renders a project overview
- A Rust backend split into `application`, `analysis`, `audio`, `commands`, and `state`
- A lightweight `fundsp` preview path that renders a short oscillator buffer summary
- A realtime `fundsp` transport path that opens the default output device through `cpal`
- A lightweight `flucoma-rs` path that runs a small normalization pass to confirm the analysis stack is wired
- A `symphonia` decode path exercised against an embedded WAV preview so the starter is ready for file-backed transport
- Placeholder folders for future resources and example assets

## Repository Layout

```text
.
├── public/
│   └── examples/                # Placeholder for starter assets
├── src/
│   ├── features/
│   │   ├── analysis/            # Analysis-facing Solid components
│   │   ├── audio/               # DSP / transport Solid components
│   │   └── project/             # App shell and project bootstrap UI
│   └── shared/
│       ├── styles/              # Shared CSS
│       └── ui/                  # Shared presentational components
└── src-tauri/
    ├── analysis_interface/      # Shared Rust DTOs for frontend/backend boundaries
    ├── crates/flucoma-rs/       # Git submodule from cristianvogel/flucoma-rs
    ├── resources/
    │   ├── audio/               # Placeholder for bundled starter audio
    │   └── models/              # Placeholder for analysis metadata / embeddings
    └── src/
        ├── application/         # App bootstrap and starter metadata
        ├── analysis/            # flucoma-rs integration points
        ├── audio/               # funDSP integration points
        ├── commands/            # Tauri commands exposed to Solid
        └── state/               # Shared runtime state
```

## Architecture Intent

### Frontend

- Keep feature logic in `src/features/*`.
- Keep generic UI primitives in `src/shared/ui`.
- Keep the frontend thin: invoke backend commands for analysis, rendering previews, filesystem work, and long-running jobs.

### Backend

- `application` owns starter metadata and high-level composition.
- `analysis` owns `flucoma-rs` experiments and future segmentation / feature extraction pipelines.
- `audio` owns `fundsp` graphs, realtime playback through `cpal`, and decode support through `symphonia`.
- `commands` stays shallow and serializable.
- `state` is where long-lived runtimes should be stored when you add streaming, project sessions, or analysis caches.

## Getting Started

### Clone with submodules

```bash
git clone --recurse-submodules <your-new-repo-url>
```

If you already cloned the repo without submodules:

```bash
git submodule update --init --recursive
```

### Install

```bash
npm install
```

### Run the frontend only

```bash
npm run dev
```

### Run the desktop app

```bash
npm run tauri dev
```

### Lint the frontend

```bash
npm run lint
```

### Lint the Rust backend

```bash
cd src-tauri
cargo clippy --all-targets --all-features
```

## Where To Extend First

- Add a real project/session model in `src-tauri/src/state`.
- Replace the starter `flucoma-rs` normalization demo with an audio-file analysis pipeline in `src-tauri/src/analysis`.
- Replace the embedded `symphonia` preview source with imported file decoding and transport buffer management.
- Add file import, transport, and inspector panels under `src/features`.

## Notes

- `flucoma-rs` is wired for now as WIP infrastructure, not as a finished product API.
- `src-tauri/crates/flucoma-rs` is expected to come from the Git submodule and to carry its vendor submodules recursively.
- `.taurignore` excludes the FluCoMa vendor tree from `tauri dev` watching, and `flucoma-sys` stages `flucoma-core` into Cargo's build output so CMake does not mutate the vendored source tree during configure.
- Empty folders are kept where the starter benefits from visible organization.
- Existing icons are preserved so the template can stay runnable while you rebrand it.
