#!/usr/bin/env bash
set -euo pipefail

# Generate OSS acknowledgement markdown from npm and Cargo lockfiles.
# Output defaults to stdout; pass a path as first argument to save to file.

out_path="${1:-}"
tmp_file="$(mktemp)"

python3 - <<'PY' > "$tmp_file"
import json
import pathlib
import re
import sys

root = pathlib.Path(".")
pkg_lock = root / "package-lock.json"
cargo_lock = root / "src-tauri" / "Cargo.lock"

def heading(text):
    print(text)
    print()

def safe(s):
    return s if s else "Unknown (verify)"

heading("# Open Source Acknowledgements")
print("This product uses open source software. The following components were detected from the current codebase lockfiles.")
print()

if pkg_lock.exists():
    heading("## JavaScript/TypeScript Dependencies")
    try:
        data = json.loads(pkg_lock.read_text())
        packages = data.get("packages", {})
        rows = []
        for key, meta in packages.items():
            if key == "":
                continue
            name = meta.get("name")
            if not name:
                if key.startswith("node_modules/"):
                    name = key.split("node_modules/", 1)[1]
                else:
                    continue
            version = meta.get("version", "Unknown (verify)")
            license_name = safe(meta.get("license"))
            rows.append((name, version, license_name))
        dedup = {}
        for name, version, license_name in rows:
            dedup[(name, version)] = license_name
        for (name, version), license_name in sorted(dedup.items(), key=lambda x: x[0][0].lower()):
            print(f"- {name} {version} - License: {license_name}")
    except Exception as e:
        print(f"- Unable to parse package-lock.json: {e}")
    print()
else:
    heading("## JavaScript/TypeScript Dependencies")
    print("- No package-lock.json found.")
    print()

if cargo_lock.exists():
    heading("## Rust Dependencies")
    text = cargo_lock.read_text()
    blocks = re.split(r"\n\[\[package\]\]\n", "\n" + text)
    rows = []
    for block in blocks:
        name_m = re.search(r'\nname = "([^"]+)"', block)
        ver_m = re.search(r'\nversion = "([^"]+)"', block)
        if not (name_m and ver_m):
            continue
        name = name_m.group(1)
        version = ver_m.group(1)
        rows.append((name, version))
    dedup = sorted(set(rows), key=lambda x: x[0].lower())
    for name, version in dedup:
        print(f"- {name} {version} - License: Unknown (verify via crates.io/Cargo metadata)")
    print()
else:
    heading("## Rust Dependencies")
    print("- No src-tauri/Cargo.lock found.")
    print()

print("## Verification Notes")
print("- Confirm exact license expressions before publication.")
print("- If required by dependency license terms, include full license texts in distribution artifacts.")
print("- Replace any 'Unknown (verify)' entries during release QA.")
PY

if [[ -n "$out_path" ]]; then
  mkdir -p "$(dirname "$out_path")"
  cp "$tmp_file" "$out_path"
  echo "Wrote acknowledgements to $out_path"
else
  cat "$tmp_file"
fi

rm -f "$tmp_file"
