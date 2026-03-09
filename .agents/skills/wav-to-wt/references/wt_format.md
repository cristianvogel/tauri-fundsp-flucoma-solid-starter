# WT Format (derived from src-tauri/src/wt_parser.rs)

## Header (12 bytes)

Offsets are little-endian.

- 0..4: ASCII magic `vawt`
- 4..8: `u32` wave_size (power of 2, 2..4096)
- 8..10: `u16` wave_count (1..512)
- 10..12: `u16` flags

### Flags (bitmask)

- `0x0001` is_sample
- `0x0002` is_looped
- `0x0004` is_int16 (if set, data is i16; if clear, data is f32)
- `0x0008` use_full_range (affects int16 scaling; still set for float data)
- `0x0010` has_metadata (if set, trailing null-terminated metadata string)

## Audio data

Immediately after the 12-byte header:

- If `is_int16` set: `wave_count * wave_size` samples of little-endian `i16`
- Else: `wave_count * wave_size` samples of little-endian `f32`

Total samples must equal `wave_count * wave_size`.

## Converter defaults

- Write float32 data (`is_int16` clear).
- Set `use_full_range` flag (`0x0008`) and clear all others.
- No metadata.
