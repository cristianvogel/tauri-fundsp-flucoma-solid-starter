## fundsp-skill: FunDSP Composable Graph Notation

**FunDSP** is a Rust library for audio synthesis. It uses a **composable graph notation** where networks are expressed as Rust types (zero-cost abstractions).

### Core Systems

* **AudioNode (Static):** Stack-allocated, fixed arity at compile-time. Fastest.
* **AudioUnit (Dynamic):** Heap-allocated, arity fixed after construction. Flexible.

### Graph Operators (High to Low Precedence)

| Op | Meaning | Connection |
| --- | --- | --- |
| `-A` | Negate | `a`  `a` |
| `!A` | Thru | Passes extra inputs through |
| `A * B` | Multiply | `a+b`  `a=b` (Ring Mod) |
| `A + B` | Sum | `a+b`  `a=b` (Mixing) |
| `A >> B` | Pipe | `a`  `b` (Series) |
| `A & B` | Bus | `a=b`  `a=b` (Mix same inputs) |
| `A ^ B` | Branch | `a=b`  `a+b` (Parallel outputs) |
| `A | B` | Stack | `a+b`  `a+b` (Parallel independent) |

### Key Opcodes

`white()`, `pink()`, `sine_hz(f)`, `saw_hz(f)`, `dc(v)`, `lowpass_hz(f, q)`, `lfo(|t| ...)`, `shared(v)`, `var()`, `multipass::<U>()`, `playwave_at()`.

---

### Implementation Patterns

**Block Process (Stereo Windowing)**

```rust
let mut input = BufferArray::<U2>::new();
let mut output = BufferArray::<U2>::new();
let mut patch = multipass::<U2>() * lfo(|t| tri_hz(1.0, t));
patch.set_sample_rate(44100.0);

patch.process(64, &input.buffer_ref(), &mut output.buffer_mut());

```

**Sequencer (Granular Overlap)**

```rust
let mut sequencer = Sequencer::new(0, 2, ReplayMode::None);
let wave = Arc::new(Wave::render(44100.0, 1.0, &mut (pink() | pink())));
let grain = playwave_at(&wave, 0, 0, wave.length(), None) 
          | playwave_at(&wave, 1, 0, wave.length(), None);

sequencer.push(0.5, 0.6, Fade::Smooth, 0.02, 0.02, Box::new(grain));

let mut backend = sequencer.backend();
backend.process(64, &BufferRef::empty(), &mut output.buffer_mut());

```