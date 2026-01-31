# RepoMedic

[![CI](https://github.com/Brutus1066/repomedic/actions/workflows/ci.yml/badge.svg)](https://github.com/Brutus1066/repomedic/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

**Fast, zero-dependency repository health scanner** — scans repos in milliseconds, generates missing files, outputs JSON/SARIF for CI/CD.

<p align="center">
  <img src="screenshots/examples.png" alt="RepoMedic Examples" width="600">
</p>

---

## Why RepoMedic?

| Feature | RepoMedic | repolinter | git-sizer |
|---------|:---------:|:----------:|:---------:|
| **Single binary** | ✅ | ❌ (Node.js) | ✅ |
| **Zero runtime deps** | ✅ | ❌ | ✅ |
| **Health score** | ✅ | ❌ | ❌ |
| **Generate missing files** | ✅ | ❌ | ❌ |
| **Secret detection** | ✅ | ❌ | ❌ |
| **Lock file checks** | ✅ | ❌ | ❌ |
| **SARIF output** | ✅ | ✅ | ❌ |
| **Cross-platform** | ✅ | ✅ | ✅ |
| **Binary size** | ~450-650KB | ~200MB | ~2MB |
| **Scan speed** | <100ms | ~2s | ~500ms |

---

## Quick Start

### Download Binary

**Windows:**
```powershell
Invoke-WebRequest -Uri "https://github.com/Brutus1066/repomedic/releases/latest/download/repomedic-windows-x64.exe" -OutFile repomedic.exe
```

**Linux:**
```sh
curl -LO https://github.com/Brutus1066/repomedic/releases/latest/download/repomedic-linux-x64
chmod +x repomedic-linux-x64 && sudo mv repomedic-linux-x64 /usr/local/bin/repomedic
```

**macOS:**
```sh
curl -LO https://github.com/Brutus1066/repomedic/releases/latest/download/repomedic-macos-x64
chmod +x repomedic-macos-x64 && sudo mv repomedic-macos-x64 /usr/local/bin/repomedic
```

### Build from Source

```sh
git clone https://github.com/Brutus1066/repomedic
cd repomedic
cargo build --release
# Binary: target/release/repomedic (or .exe on Windows)
```

**Requirements:** Rust 1.70+ (stable)

---

## Usage

```sh
# Basic scan
repomedic

# Health score with badge
repomedic scan --score

# Fix suggestions
repomedic scan --suggest

# JSON output for CI
repomedic --format json

# SARIF for GitHub Code Scanning
repomedic --format sarif > results.sarif

# Generate all missing files
repomedic generate --all

# Generate specific files
repomedic generate --readme --license --author "Your Name"
```

---

## Health Score

RepoMedic calculates a **0-100 health score** based on:

| Factor | Impact |
|--------|--------|
| Missing README/LICENSE | -15 each |
| Missing .gitignore, CONTRIBUTING, etc. | -5 each |
| Missing tests/docs | -1 each |
| Has tests directory | +5 |
| Has docs directory | +3 |
| Has CI/CD configured | +5 |
| Potential secrets detected | -15 each |

**Grades:** A (90-100), B (80-89), C (70-79), D (60-69), F (<60)

<p align="center">
  <img src="screenshots/info.png" alt="RepoMedic Info" width="600">
</p>

---

## Commands

| Command | Description |
|---------|-------------|
| `scan` | Scan repository (default) |
| `scan --score` | Show health score and badge |
| `scan --suggest` | Show fix commands |
| `doctor` | One-line summary (great for scripts/prompts) |
| `export <file>` | Export to .json, .md, .txt, or .csv |
| `info` | Detailed help with feature explanations |
| `examples` | Show colorful usage examples |
| `init` | Generate all recommended files at once |
| `report` | Generate REPO_REPORT.md file |
| `generate` | Generate specific missing files |

## Flags

| Flag | Description |
|------|-------------|
| `-f, --format` | Output: `console`, `json`, `markdown`, `sarif` |
| `-q, --quiet` | Exit code only (for scripts) |
| `-v, --verbose` | Show scan stats and timing |
| `--no-color` | Disable colored output |
| `--fail-on-warning` | Exit 2 on warnings (not just errors) |

## Generate Options

| Flag | Description |
|------|-------------|
| `--all` | Generate all missing files |
| `--readme` | README.md template |
| `--license` | MIT LICENSE |
| `--gitignore` | Language-appropriate .gitignore |
| `--contributing` | CONTRIBUTING.md |
| `--changelog` | CHANGELOG.md |
| `--editorconfig` | .editorconfig |
| `--codeofconduct` | CODE_OF_CONDUCT.md |
| `--security` | SECURITY.md |
| `--dry-run` | Preview without writing |
| `--author <name>` | Author name for LICENSE |

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Clean — no issues |
| 1 | System error (IO, path not found) |
| 2 | Issues found (missing README, secrets, etc.) |

---

## CI/CD Integration

### GitHub Actions

```yaml
- name: Repository Health Check
  run: |
    curl -LO https://github.com/Brutus1066/repomedic/releases/latest/download/repomedic-linux-x64
    chmod +x repomedic-linux-x64
    ./repomedic-linux-x64 --fail-on-warning
```

### GitHub Code Scanning (SARIF)

```yaml
- name: Run RepoMedic
  run: ./repomedic --format sarif > repomedic.sarif
  
- name: Upload SARIF
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: repomedic.sarif
```

### Pre-commit Hook

```sh
#!/bin/sh
repomedic -q || exit 1
```

---

## Detection Coverage

**Languages (23):** Rust, Python, JavaScript, TypeScript, Go, Java, C#, C++, C, Ruby, PHP, Swift, Kotlin, Scala, Haskell, Elixir, Zig, Nim, Lua, R, Perl, Dart, Crystal

**Build Systems (19):** Cargo, pip, Poetry, npm, Yarn, pnpm, Go modules, Maven, Gradle, MSBuild, CMake, Make, Bundler, Composer, Mix, Cabal, Stack, Zig, Nimble

**CI/CD (6):** GitHub Actions, GitLab CI, Jenkins, CircleCI, Travis CI, Azure Pipelines

**Secrets:** API keys, tokens, passwords, private keys (patterns)

---

## Screenshots

<details>
<summary>📊 Export Report</summary>
<p align="center">
  <img src="screenshots/export-txt.png" alt="Export Text Report" width="600">
</p>
</details>

<details>
<summary>📖 Detailed Info</summary>
<p align="center">
  <img src="screenshots/info.png" alt="Info Command" width="600">
</p>
</details>

---

## License

MIT License — see [LICENSE](LICENSE)

## Author

LazyFrog / [Kindware.dev](https://kindware.dev)

---

<p align="center">
  <sub>⭐ Star this repo if RepoMedic helped your project!</sub>
</p>
