# Pipeline Patterns

## Probe

```bash
ffprobe -v error -show_entries format=duration -of csv=p=0 input.mp4
ffprobe -v error -select_streams v:0 -show_entries stream=width,height -of csv=p=0:s=x input.mp4
```

## Audio peak ranking (RMS)

```bash
ffmpeg -hide_banner -nostats -i input.mp4 -vn \
  -af "astats=metadata=1:reset=1,ametadata=print:key=lavfi.astats.Overall.RMS_level" \
  -f null - 2>&1
```

Parse `pts_time` + `RMS_level`, sort by loudness descending, and enforce spacing.

## Ken Burns filter pattern

Use animated scale plus center-biased crop:

```bash
-vf "scale=W*Z:H*Z:eval=frame,crop=W:H:X:Y"
```

Where `Z`, `X`, `Y` are time-based expressions over clip duration.

## Black/freeze rejection checks

```bash
ffmpeg -hide_banner -nostats -ss START -i input.mp4 -t 3 \
  -vf "<visual-filter>,blackdetect=d=0.10:pic_th=0.98,freezedetect=n=-55dB:d=1.2" \
  -an -f null - 2>&1
```

Extract:
- `black_duration:`
- `freeze_duration:`

Reject if either metric exceeds threshold ratio of clip length.

## Output validation

```bash
ffprobe -v error -select_streams v:0 -show_entries stream=codec_name -of csv=p=0 out.mp4
```

If empty, treat output as invalid and remove it.
