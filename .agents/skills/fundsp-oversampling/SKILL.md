---
name: oversampling-antialiasing
description: 2x oversampling for antialiasing nonlinear processing (saturation, distortion, waveshaping). Use when working with real-time audio effects, buffer-based granulation, or synthesizers that need aliasing control.
---

# Oversampling & Antialiasing in FundDSP

## Quick start

Wrap any audio processing node with `Oversampler` for 2x antialiasing:

```rust
use fundsp::prelude32::*;

// Antialiased saturation
let saturated = oversample(
    sine_hz(440.0) >> shape(Shape::Tanh(2.0))
);

saturated.set_sample_rate(44100.0);  // Internal: 88200.0
```

For buffer-based granulation with pitch-shifting:

```rust
let grain = playwave_at(&buffer, channel, start, end, None)
    >> oversample(pan(position));
```

For arbitrarily different sample rates (not just 2x), use `ResampleFir`:

```rust
let resampler = resample_fir(
    source_rate,
    target_rate,
    Quality::High,
    your_generator
);
```

---

## What is Oversampling?

Oversampling processes audio at a **higher sample rate internally** to prevent aliasing artifacts from nonlinear processing.

### Why It Matters

**Problem**: Nonlinear processing (saturation, distortion) generates high-frequency harmonics that **fold back** as aliasing when the sample rate isn't high enough.

**Solution**: Process at 2Fs, then downsample with antialiasing filter.

```
Input @ Fs ──→ Interpolate ──→ Process @ 2Fs ──→ Decimate ──→ Output @ Fs
                (duplicate     (2 samples per      (filter +
                 samples)       input sample)       downsample)
```

### FundDSP's Approach

- **Fixed 2x upsampling** (not variable ratio)
- **Linear-phase Kaiser windowed filter** (80 dB stopband attenuation)
- **Ring buffer architecture** (128-sample circular buffers)
- **SIMD-optimized** (f32x8 operations for performance)
- **Minimal latency** (~5-10 samples)

---

## When to Use

### ✅ Use Oversampling For

- **Saturation/distortion** on synthesizers (especially high-frequency content)
- **Waveshaping** with steep nonlinearities (tanh, atan, softsign)
- **Buffer granulation with pitch-shifting** (40-70% overhead is acceptable)
- **FM synthesis** where aliasing is perceptually obvious
- **High-harmonic oscillators** (sawtooth, square, pulse waves)

### ❌ Don't Use For

- **Linear processing** (filters, delays, panning alone) — they don't create aliasing
- **Full granular synth wrapper** — instead, oversample individual grains
- **Subtractive synthesis chains** — oversample stages selectively instead
- **Already-oversampled content** — redundant processing

### 🔄 Alternatives

| Goal | Use Instead | Reason |
|------|------------|--------|
| Change sample rate arbitrarily | `resample_fir()` | Supports specific rates: 16k, 22.05k, 32k, 44.1k, 48k, 88.2k, 96k, 176.4k, 192k, 384k |
| Reduce aliasing without oversampling | Lower saturation hardness | Fewer harmonics = less aliasing; trade-off between sound and processing cost |
| Antialias specific frequency range | Bandpass + oversample | Selective oversampling only where needed |

---

## Performance

### CPU Overhead by Context

#### Oscillator-Based Synthesis
| Approach | Overhead | Recommendation |
|----------|----------|-----------------|
| Wrap entire `Granular` synth | ~2.2x per voice | ❌ Avoid — too expensive |
| Per-grain oscillator oversampling | ~30-50% | ✅ Localized, acceptable |
| Single saturation stage | ~40-60% | ✅ Minimal impact on overall |

#### Buffer-Based Granulation
| Approach | Overhead | Recommendation |
|----------|----------|-----------------|
| Per-grain buffer playback | ~30-50% | ✅ Buffer reads scale with pitch |
| With pitch-shifting | ~40-70% | ✅ Reasonable trade-off |
| Wrap entire granular | ~2.2x | ❌ Avoid |

**Key insight**: Buffer overhead is **not 2x** because antialiasing filter cost is amortized across multiple grains and playback speed directly affects read cost.

---

## Architecture

### How It Works Internally

FundDSP's `Oversampler<X>` uses ring buffers and parallel filter paths:

```rust
struct Oversampler<X> {
    x: X,
    inv: Frame<Frame<f32, U128>, X::Inputs>,   // Input buffers (128 samples)
    outv: Frame<Frame<f32, U128>, X::Outputs>, // Output buffers (128 samples)
    input_rb_index: usize,  // Current read/write position
    output_rb_index: usize,
}
```

**Ring Buffer Advantages**:
- Fixed 128-sample size matches SIMD width
- Modulo arithmetic via bitmask: `(index + 1) & 0x7f` (cheap)
- Sufficient for 85-tap filter kernels

### Filter Specification

Both interpolation and decimation use **Kaiser-windowed sinc filters**:

- **Cutoff**: Normalized frequency 0.22
- **Transition band**: 0.06
- **Stopband attenuation**: 80 dB
- **Phase response**: Linear (no frequency-dependent phase shift)
- **Source**: Generated at [fiiir.com](https://fiiir.com/)

Performance @ 88.2 kHz:
- Gain -1.5 dB @ 18.5 kHz (0.21 normalized)
- Attenuation > 79 dB @ 22 kHz (0.25 normalized)

---

## Usage Patterns

### Pattern 1: Antialiased Distortion

```rust
// Prevents aliasing on any saturation function
let distortion_unit = oversample(
    shape(Shape::Tanh(hardness))
);

let output = distortion_unit.tick(&input_frame);
```

**When to use**: Always wrap distortion/saturation. The overhead is worth the clean sound.

### Pattern 2: Selective Oversampling

Only oversample the nonlinear stage, leave linear processing unaffected:

```rust
let chain = 
    input
    >> highpass_hz(20.0, 1.0)              // Linear: no oversampling
    >> oversample(shape(Shape::Atan(2.0))) // Nonlinear: oversampled only here
    >> lowpass_hz(15000.0, 1.0);           // Linear: post-antialiasing filter
```

**CPU cost**: ~50% (only distortion oversampled, not full chain)

### Pattern 3: Buffer Granulation

Efficient for sample-based granular synthesis with pitch-shifting:

```rust
// Per-grain oversampling (NOT the entire granular system)
let grain_generator = |start_idx: usize, pitch: f32| {
    playwave_at(&sample_buffer, channel, start_idx, start_idx + 4410, None)
        >> oversample(pan(voice_position))
};

// Instantiate 24 grains: ~60% overhead each, totaling ~144% (not 200%)
```

**Why efficient here**: 
- Buffer reads at pitch-shifted rate (not 2x cost)
- Antialiasing filters operate at fixed cost
- Ring buffer overhead shared across multiple grains

### Pattern 4: FM Synthesis

Pitch modulation generates aliasing without oversampling:

```rust
// Modulated carrier with antialiasing
let fm = lfo(|t| {
    let carrier = 440.0;
    let modulator = sin_hz(5.0, t) * 50.0;
    carrier + modulator
})
>> oversample(sine())  // Prevent aliasing from FM
>> pan(0.0);
```

---

## Troubleshooting

### Symptom: High CPU Usage (>100% overhead)

**Cause**: Likely wrapping entire granular system  
**Solution**:
```rust
// ❌ Bad: wraps everything
let granular = Granular::new(...);
let oversampled = oversample(granular);  // ~2.2x CPU

// ✅ Good: wrap individual grains
let grain = |...| {
    playwave_at(...) >> oversample(pan(...))
};
```

### Symptom: Latency Issues

**Cause**: Oversampling adds ~5-10 samples latency  
**Solution**: Use `Wave::render_latency()` for offline processing:
```rust
let wave = Wave::render_latency(44100.0, duration, &mut oversample(...));
```

For real-time: Latency is usually imperceptible (< 1 ms @ 44.1 kHz)

### Symptom: Aliasing Still Audible

**Cause**: Several possibilities  
**Solutions**:
1. Verify `set_sample_rate()` called before processing
2. Confirm nonlinear processing is **inside** the `Oversampler`:
   ```rust
   // ✅ Correct: processing happens inside oversample()
   oversample(saturation())
   
   // ❌ Wrong: processing outside
   oversample(input) >> saturation()  // Aliasing not prevented
   ```
3. Increase saturation hardness to generate more harmonics (audibly test)
4. Consider 4x oversampling (extend code; ~4.8x CPU cost)

---

## Advanced Topics

### Extending to 4x Oversampling

Compose two `Oversample` stages for 4x:

```rust
pub struct Oversampler4x<X> {
    stage: Oversampler<Oversampler<X>>,
}

// Cost: ~2.2² ≈ 4.8x CPU (quadrupling quality costs exponential CPU)
```

**Use only if**: 2x aliasing is still audible AND CPU budget allows

### Custom Filter Kernels

Replace default filter coefficients for different characteristics:

- **Lower latency**: Shorter kernel (~40 taps instead of 85)
- **Higher cutoff**: Preserve more high-frequency content
- **Different passband shape**: Customize for specific use cases

Regenerate at [fiiir.com](https://fiiir.com/) with your specifications.

---

## See Also

- **Saturation functions**: Tanh, Atan, Softsign (all available in `Shape` enum)
- **Sample rate conversion**: Use `resample_fir()` for arbitrary rate changes
- **Granular synthesis**: Buffer-based examples in `examples/grain2.rs`
- **Filter design**: Kaiser windowing, sinc interpolation references