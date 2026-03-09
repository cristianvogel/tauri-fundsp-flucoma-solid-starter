---
name: incline-function-lookup
description: Fast symbol lookup for the Incline codebase. Use when you need to locate where a Rust or TypeScript/TSX function, method, trait impl member, Tauri command, or exported handler is defined, and report precise file paths and line numbers.
---

# Incline Function Lookup

Use this workflow to find function locations quickly and return exact paths with line numbers.

## Workflow
1. Identify symbol type from the request.
2. Narrow candidate files with `rg --files` and path filters.
3. Locate definitions with language-specific `rg` patterns.
4. Confirm exact definition lines and nearby context with `rg -n` or `sed -n`.
5. Return absolute file paths and 1-based line numbers.

## Quick Commands
```bash
# Repo-wide first pass
rg -n "<symbol>" src src-tauri

# Rust function definitions
rg -n "^\s*(pub\s+)?(async\s+)?fn\s+<symbol>\b" src-tauri/src

# Rust impl method definitions
rg -n "^\s*(pub\s+)?(async\s+)?fn\s+<symbol>\b" src-tauri/src --glob "*.rs"

# Tauri command handlers (attribute + fn)
rg -n "#\[tauri::command\]|^\s*(pub\s+)?(async\s+)?fn\s+<symbol>\b" src-tauri/src

# TypeScript/TSX function forms
rg -n "(export\s+)?function\s+<symbol>\b|const\s+<symbol>\s*=\s*\(|const\s+<symbol>\s*=\s*async\s*\(|<symbol>\s*:\s*\([^)]*\)\s*=>" src --glob "*.ts" --glob "*.tsx"
```

## Disambiguation Rules
- If multiple matches exist, list all plausible definitions and label likely primary one by directory context.
- Prefer non-test definitions unless user asks for tests.
- For overloaded semantics (same name in frontend and backend), return both and separate by subsystem.
- If only references are found, state that and provide closest entry points.

## Output Format
- `<absolute-path>:<line>`
- One line per definition candidate.
- Add a one-line note for ambiguity when needed.

## Reference File
Use [`references/patterns.md`](references/patterns.md) for extended regex patterns and path-scoping hints.
