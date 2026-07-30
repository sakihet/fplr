# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
make build       # cargo build --release
make run         # cargo run
make test        # cargo test
make fmt         # cargo fmt
make lint        # cargo clippy -- -D warnings
make lint-fix    # cargo clippy --fix --allow-dirty --allow-staged -- -D warnings
make install     # cargo install --path .
```

After generating or modifying code: run `make fmt` then `make lint-fix`.

## Architecture

**fplr** is a Rust CLI for Fantasy Premier League data, built with `clap` (derive API) and `tokio` for async HTTP calls to the official FPL API.

### Module layout

- `src/main.rs` — `clap` CLI definition (`Args`/`Commands` enums) and top-level dispatch via `match`. All subcommands are defined here and routed to handlers.
- `src/api.rs` — `FplClient` struct with async methods that fetch JSON from `https://fantasy.premierleague.com/api`. Each endpoint maps to one method (e.g. `fetch_bootstrap_static`, `fetch_live`).
- `src/models/` — serde structs for API responses (`api.rs`) and CLI types like `Position`, `SortBy` enums (`cli.rs`).
- `src/commands/` — one file per subcommand, each exporting a `handle_*` async function. Commands fetch data via `FplClient`, compute derived stats, and print formatted tables.
- `src/utils/` — shared helpers:
  - `constants.rs` — `WIDTH_*` constants used for all table column widths
  - `formatters.rs` — datetime formatting, difficulty colorization, color helpers
  - `player_helpers.rs`, `team_helpers.rs`, `region_helpers.rs`, `event_helpers.rs` — lookup utilities
- `src/config.rs` — TOML config at `~/.config/fplr/config.toml` storing `manager_id`.
- `src/error.rs` — `FplrError` enum (using `thiserror`) and `Result<T>` alias.

### Key conventions

- Use `WIDTH_*` constants from `src/utils/constants.rs` for all table column widths — never hardcode column widths.
- Use `owo_colors` for terminal color output — no raw ANSI escape codes.
- Keep CLI subcommands sorted alphabetically in `src/main.rs` (both the `Commands` enum variants and the `match` arms).
- Sort `match` arms alphabetically whenever possible.
- Code comments in English only.

## Commit messages

Follow Conventional Commits: `type: short summary` (or `type(scope): summary`).

- Types used in this repo: `feat`, `fix`, `refactor`, `style`, `docs`, `chore`, `ci`
- Summary: lowercase, imperative mood ("add", not "added"/"adds"), no trailing period
- Keep the subject line concise; add a body only when the "why" isn't obvious from the diff
