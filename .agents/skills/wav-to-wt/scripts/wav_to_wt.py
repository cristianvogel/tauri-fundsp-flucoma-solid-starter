#!/usr/bin/env python3
"""Convert a mono WAV (4096 samples per frame) into a WT file."""

from __future__ import annotations

import argparse
import os
import re
import struct
import sys
import wave
from array import array
from pathlib import Path

WAVE_SIZE = 4096
MAX_WAVES = 512
DEFAULT_STEM = "incline_preset"
DEFAULT_OUT_DIR = (
    "/Users/cristianvogel/RustroverProjects/incline-app/src/assets/preset-windows/converted"
)


def read_pcm_mono(path: Path) -> list[float]:
    with wave.open(str(path), "rb") as wf:
        channels = wf.getnchannels()
        if channels != 1:
            raise ValueError(f"Expected mono WAV (1 channel), got {channels} channels")
        sampwidth = wf.getsampwidth()
        frame_count = wf.getnframes()
        raw = wf.readframes(frame_count)

    if frame_count == 0:
        raise ValueError("WAV contains no samples")

    samples: list[float] = []

    if sampwidth == 1:
        # 8-bit PCM is unsigned.
        for b in raw:
            samples.append((b - 128) / 128.0)
    elif sampwidth == 2:
        fmt = f"<{frame_count}h"
        for v in struct.unpack(fmt, raw):
            samples.append(v / 32768.0)
    elif sampwidth == 3:
        for i in range(0, len(raw), 3):
            v = int.from_bytes(raw[i : i + 3], "little", signed=True)
            samples.append(v / 8388608.0)
    elif sampwidth == 4:
        fmt = f"<{frame_count}i"
        for v in struct.unpack(fmt, raw):
            samples.append(v / 2147483648.0)
    else:
        raise ValueError(f"Unsupported sample width: {sampwidth} bytes")

    # Clamp to [-1.0, 1.0] to avoid out-of-range float data.
    for i, v in enumerate(samples):
        if v > 1.0:
            samples[i] = 1.0
        elif v < -1.0:
            samples[i] = -1.0

    return samples


def next_letter(out_dir: Path, stem: str) -> str:
    pattern = re.compile(rf"^{re.escape(stem)}_([A-Z])\\.wt$")
    letters = []
    if out_dir.exists():
        for entry in out_dir.iterdir():
            if not entry.is_file():
                continue
            match = pattern.match(entry.name)
            if match:
                letters.append(match.group(1))

    if not letters:
        return "A"

    max_letter = max(letters)
    next_code = ord(max_letter) + 1
    if next_code > ord("Z"):
        raise ValueError("No available filename slots after Z")
    return chr(next_code)


def write_wt(path: Path, samples: list[float]) -> None:
    total_samples = len(samples)
    if total_samples % WAVE_SIZE != 0:
        raise ValueError(
            f"Total samples ({total_samples}) is not a multiple of {WAVE_SIZE}"
        )

    wave_count = total_samples // WAVE_SIZE
    if not (1 <= wave_count <= MAX_WAVES):
        raise ValueError(f"Invalid wave count: {wave_count} (must be 1..{MAX_WAVES})")

    # Reorder frames by zero crossing count (ascending).
    frames: list[tuple[int, list[float]]] = []
    for frame_index in range(wave_count):
        start = frame_index * WAVE_SIZE
        end = start + WAVE_SIZE
        frame = samples[start:end]
        zero_crossings = 0
        prev = frame[0]
        for v in frame[1:]:
            if (prev <= 0.0 and v > 0.0) or (prev >= 0.0 and v < 0.0):
                zero_crossings += 1
            prev = v
        frames.append((zero_crossings, frame))

    frames.sort(key=lambda item: item[0])
    ordered_samples = [v for _, frame in frames for v in frame]

    # Flags: use_full_range only, float32 data.
    flags = 0x0008

    header = b"vawt" + struct.pack("<IHH", WAVE_SIZE, wave_count, flags)

    data = array("f", ordered_samples)
    if sys.byteorder != "little":
        data.byteswap()

    with path.open("wb") as f:
        f.write(header)
        f.write(data.tobytes())


def main() -> int:
    parser = argparse.ArgumentParser(description="Convert mono WAV to WT.")
    parser.add_argument("wav", help="Path to mono WAV file")
    parser.add_argument(
        "--out-dir",
        default=DEFAULT_OUT_DIR,
        help=f"Output directory (default: {DEFAULT_OUT_DIR})",
    )
    parser.add_argument(
        "--stem",
        default=DEFAULT_STEM,
        help=f"Output filename stem (default: {DEFAULT_STEM})",
    )
    args = parser.parse_args()

    wav_path = Path(args.wav).expanduser().resolve()
    out_dir = Path(args.out_dir).expanduser().resolve()
    stem = args.stem

    if not wav_path.exists():
        raise FileNotFoundError(f"Input WAV not found: {wav_path}")

    out_dir.mkdir(parents=True, exist_ok=True)
    suffix = next_letter(out_dir, stem)
    out_path = out_dir / f"{stem}_{suffix}.wt"

    samples = read_pcm_mono(wav_path)
    write_wt(out_path, samples)

    print(out_path)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
