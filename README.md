# fplr - a CLI for [Fantasy Premier League](https://fantasy.premierleague.com/) in Rust

[![crates.io](https://img.shields.io/crates/v/fplr.svg)](https://crates.io/crates/fplr)

Browse FPL data comfortably from your terminal.

Documentation: https://sakihet.github.io/fplr/

## Installation

### Homebrew

```sh
brew tap sakihet/tap
brew install fplr
```

### Cargo

```sh
cargo install fplr
```

### From source

```sh
git clone https://github.com/sakihet/fplr.git
cd fplr
cargo install --path .
```

## Configuration

Set your manager ID to enable personalized commands like `my-team` and `history`:

```sh
fplr config set manager-id YOUR_MANAGER_ID
```

### Finding your manager ID

1. Log in to [Fantasy Premier League](https://fantasy.premierleague.com/)
2. Click the "Points" link
3. Check the URL: `https://fantasy.premierleague.com/entry/{manager_id}/event/{event_id}`

## Commands

```
Commands:
  buy               Show replacement candidates for my team (position, budget, club limit)
  cache             Manage the HTTP response cache
  completions       Generate shell completion scripts
  config            Manage configuration
  differential      Show low-ownership players with high potential
  dream-team        Show dream team
  fixture           Show fixtures, difficulty ratings and results
  gameweek          Show gameweeks
  history           Show manager's season history
  live              Show live player stats for a specific event
  manager           Show a specific manager's team
  mini-league       Show mini-league standings
  most-valuable     Show the most valuable teams (squad value including bank)
  my-leagues        Show my leagues
  my-team           Show my team
  pick              Show a manager's team picks for a specific event
  player            Show player lists, stats and summaries
  price-change      Show official price change progress
  sell              Show sell candidates from my team (availability, fixtures, minutes)
  status            Show status
  table             Show league table
  team              Show team stats, form and trends
  template          Show template squad (top players by ownership per position)
  top               Show top teams in the overall league
  transfer          Show popular transfers
  transfer-history  Show manager's transfer history
  help              Print this message or the help of the given subcommand(s)
```

### fixture

```
$ fplr fixture --help
Commands:
  fdr      Show fixture difficulty rating
  list     Show upcoming fixtures
  results  Show season results matrix
  show     Show detailed points summary for a specific fixture
  swing    Show mathematically calculated Fixture Swings
  help     Print this message or the help of the given subcommand(s)
```

### player

```
$ fplr player --help
Commands:
  availability  Show player availability (injuries, suspensions, etc.)
  compare       Compare two players side-by-side
  history       Show a player's performance in previous seasons
  list          Show players
  region        Show regions, or players from a specific region if a name is given
  set-piece     Show set piece takers (penalties, free kicks, corners)
  show          Show player summary
  talisman      Show talisman players
  trend         Show player performance trends with sparklines
  xa            Show xA vs Assists analysis (creativity and efficiency)
  xg            Show xG vs Goals scored analysis (finishing ability and efficiency)
  xgc           Show xGC vs Goals Conceded analysis
  xgi           Show xGI vs Goal Involvements (Actual G + A) analysis
  help          Print this message or the help of the given subcommand(s)
```

### team

```
$ fplr team --help
Commands:
  availability  Show team availability statistics
  form          Show team form based on total player form
  fpl-rank      Show team FPL points rank vs Premier League position
  ha            Show team home/away performance stats
  list          Show teams
  perf          Show team performance based on player points per GW
  stats         Show team attack/defence stats per match (CS%, xG/match)
  trend         Show team performance trends with sparklines
  help          Print this message or the help of the given subcommand(s)
```

## Shell Completion

Generate and install tab completion scripts for your shell:

**zsh**

```sh
mkdir -p ~/.zfunc
fplr completions zsh > ~/.zfunc/_fplr
```

Add the following to `~/.zshrc` if not already present:

```sh
fpath+=~/.zfunc
autoload -Uz compinit && compinit
```

**bash**

```sh
fplr completions bash > ~/.local/share/bash-completion/completions/fplr
```

**fish**

```sh
fplr completions fish > ~/.config/fish/completions/fplr.fish
```

## Color output

Output is colored only when it goes to a terminal, so piping into `grep`, `awk`
or a file gives plain text:

```sh
fplr table | grep Arsenal
```

Color is turned off when any of these apply:

| | |
| --- | --- |
| `--no-color` | passed on the command line |
| `NO_COLOR` | set to a non-empty value ([no-color.org](https://no-color.org/)) |
| `FPLR_NO_COLOR` | set to a non-empty value |
| `TERM=dumb` | |
| stdout is not a terminal | e.g. a pipe or a redirect |
