# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Format check
cargo fmt --all -- --check

# Lint
cargo clippy

# Test
cargo test --release --all-features

# Run a single test
cargo test <test_name> --release --all-features

# Publish to crates.io
cargo publish --token ${CRATES_TOKEN}
```

## Architecture

**alisql** is a Rust library (crate types: `cdylib` + `rlib`) that analyzes SQL files written with Jinja2 template syntax, extracts table dependencies from `{{ ref(...) }}` macros, and generates Mermaid diagrams to visualize dependency graphs.

### Public API (`src/lib.rs`)

Two public functions serve as the entry point:
- `get_dependencies(root_dir, max_depth) -> Vec<Table>` — walks a directory for `.sql` files and returns each file's table name and its upstream dependencies
- `get_mermaid(root_dir, orientation, max_depth) -> String` — returns Mermaid diagram syntax for the dependency graph (orientations: TB, TD, BT, RL, LR)

### Internal Modules

**`src/sql_analyzer/analyzer.rs`**
- `Analyzer` trait — interface for SQL analysis implementations
- `RegexSQLAnalyser` — uses `minijinja` to render Jinja2 templates and `regex` to extract `ref()` calls, then maps each SQL file to a `Table` struct containing its name and dependencies
- `Table` — serializable struct representing a SQL file and its upstream refs
- `SQL` — holds the raw query, rendered query, and file path

**`src/sql_analyzer/graph.rs`**
- `Mermaid` — converts `Vec<Table>` dependency data into Mermaid flowchart syntax

### Data Flow

1. `walkdir` recursively finds `.sql` files under `root_dir` (up to `max_depth`)
2. Each file is rendered through `minijinja` to resolve Jinja2 templates
3. `ref(...)` calls are extracted via regex to identify upstream table dependencies
4. Table name is derived from the `.sql` filename
5. Results are returned as `Vec<Table>` or converted to a Mermaid diagram string
