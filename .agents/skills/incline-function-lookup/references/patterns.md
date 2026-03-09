# Lookup Patterns

## Scope First
```bash
rg --files src src-tauri/src
rg --files src | rg "AppLogic|state|MainLayout"
rg --files src-tauri/src | rg "commands|audio_fundsp|lib.rs"
```

## Rust
```bash
# free functions
rg -n "^\s*(pub\s+)?(async\s+)?fn\s+NAME\b" src-tauri/src --glob "*.rs"

# methods inside impl blocks
rg -n "^\s*impl\b|^\s*(pub\s+)?(async\s+)?fn\s+NAME\b" src-tauri/src --glob "*.rs"

# command registration / dispatch clues
rg -n "invoke_handler|generate_handler|AudioCommand|match\s+command" src-tauri/src --glob "*.rs"
```

## TypeScript / TSX
```bash
# named functions
rg -n "(export\s+)?function\s+NAME\b" src --glob "*.ts" --glob "*.tsx"

# const function expressions and arrows
rg -n "const\s+NAME\s*=\s*(async\s*)?\(" src --glob "*.ts" --glob "*.tsx"
rg -n "const\s+NAME\s*=\s*(async\s*)?\([^)]*\)\s*=>" src --glob "*.ts" --glob "*.tsx"

# object members and class methods (best-effort)
rg -n "\bNAME\s*\([^)]*\)\s*\{|\bNAME\s*:\s*(async\s*)?\([^)]*\)\s*=>" src --glob "*.ts" --glob "*.tsx"
```

## Fallback Strategy
1. Search broad symbol text: `rg -n "NAME" src src-tauri/src`.
2. Filter to definition-like lines with one extra regex pass.
3. Open likely files and confirm declaration context.
