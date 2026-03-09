---
name: wav-to-wt
description: Convert mono WAV files with 4096 samples per frame into valid WT wavetable files for Incline, using the WT header/flags derived from src-tauri/src/wt_parser.rs and an incline_preset_<A-Z> naming convention. Use when generating WT files from WAVs for preset-windows/converted.
---

# WAV to WT Conversion

## Quick start

Run the bundled script to convert a mono WAV into a WT file:

```bash
python3 /Users/cristianvogel/RustroverProjects/incline-app/.agents/skills/wav-to-wt/scripts/wav_to_wt.py /path/to/input.wav
```

By default the script:

- Expects mono PCM WAV input.
- Enforces 4096 samples per frame.
- Writes a WT file with float32 samples and `use_full_range` flag set.
- Saves into `/Users/cristianvogel/RustroverProjects/incline-app/src/assets/preset-windows/converted`.
- Uses the next available `incline_preset_<A-Z>.wt` name (starting at `_A`).

## Workflow

1. Verify the WAV is mono and that total samples is a multiple of 4096.
2. Convert PCM samples to float32 in [-1.0, 1.0], clamping if needed.
3. Split into 4096-sample frames, count zero crossings per frame, and reorder frames from lowest to highest zero crossings.
4. Write WT header + float32 audio data (see `references/wt_format.md`).
5. Save with the next `incline_preset_<A-Z>.wt` name in the converted directory.

## Script

Use `scripts/wav_to_wt.py`.

Optional flags:

- `--out-dir`: override the output directory.
- `--stem`: override the filename stem (default `incline_preset`).

If no filename slots remain (past `Z`), the script should error.

## Reference

Use `references/wt_format.md` for the WT header layout and flag bits derived from `src-tauri/src/wt_parser.rs`.
