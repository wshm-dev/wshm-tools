[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

# wshm (wishmaster)

> Your repo's wish is my command.

AI-powered GitHub agent for OSS maintainers. Triage issues, auto-fix simple bugs, analyze PRs, resolve conflicts. Built in Rust. Zero infra. One binary.

## Install

```bash
cargo install wshm
```

Or build from source:

```bash
git clone https://github.com/pszymkowiak/wshm.git
cd wshm
cargo build --release
```

## Quick Start

```bash
# 1. Initialize config
wshm config init          # creates .wshm/config.toml

# 2. Set environment variables
export GITHUB_TOKEN="ghp_..."
export ANTHROPIC_API_KEY="sk-ant-..."

# 3. Sync repo state from GitHub
wshm sync

# 4. Triage open issues (dry-run)
wshm triage

# 5. Apply triage actions (label + comment)
wshm triage --apply
```

## CLI Reference

| Command | Description |
|---------|-------------|
| `wshm` | Show status from cache (instant) |
| `wshm sync` | Force full sync from GitHub |
| `wshm triage [--issue <N>]` | Classify issues (or single issue) |
| `wshm triage --apply` | Classify + label + comment |
| `wshm pr analyze [--pr <N>]` | Analyze PRs (or single PR) |
| `wshm pr analyze --apply` | Analyze + label + comment |
| `wshm queue` | Show ranked merge queue |
| `wshm queue --apply` | Merge top PR if above threshold |
| `wshm conflicts scan` | Detect conflicting PRs |
| `wshm conflicts scan --apply` | Attempt AI conflict resolution |
| `wshm run` | Full cycle: sync + triage + analyze + queue + conflicts |
| `wshm run --apply` | Full cycle with actions |
| `wshm config init` | Create `.wshm/config.toml` template |

### Global Flags

| Flag | Description |
|------|-------------|
| `--apply` | Perform actions (dry-run by default) |
| `--offline` | Skip GitHub sync, use cached data only |
| `--verbose` / `-v` | Detailed output |
| `--json` | JSON output for scripting |
| `--repo <owner/repo>` | Override detected repo |

## GitHub Action

```yaml
- uses: pszymkowiak/wshm@v1
  with:
    github-token: ${{ secrets.GITHUB_TOKEN }}
    anthropic-api-key: ${{ secrets.ANTHROPIC_API_KEY }}
```

## Architecture

See [CLAUDE.md](CLAUDE.md) for the full architecture document, including:

- SQLite cache strategy and sync rules
- The 4 pipelines (triage, PR analysis, merge queue, conflict resolution)
- Config reference
- Project structure
- AI integration patterns
- Safety principles

## License

[MIT](LICENSE)
