# fplr - a CLI for [Fantasy Premier League](https://fantasy.premierleague.com/) in Rust

Browse FPL data comfortably from your terminal.

## Installation

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
  availability               Show player availability (injuries, suspensions, etc.)
  compare                    Compare two players side-by-side
  completions                Generate shell completion scripts
  config                     Manage configuration
  differential               Show low-ownership players with high potential
  dream-team                 Show dream team
  fdr-form                   Show form-adjusted fixture difficulty rating
  fixture                    Show upcoming fixtures
  fixture-difficulty-rating  Show fixture difficulty rating [aliases: fdr]
  fixture-summary            Show detailed points summary for a specific fixture
  gameweek                   Show gameweeks
  history                    Show manager's season history
  history-past               Show a player's performance in previous seasons
  live                       Show live player stats for a specific event
  mini-league                Show mini-league standings
  manager                    Show a specific manager's team
  my-leagues                 Show my leagues
  my-team                    Show my team
  pick                       Show a manager's team picks for a specific event
  player                     Show players
  player-summary             Show player summary
  region                     Show regions
  results                    Show season results matrix
  set-piece                  Show set piece takers (penalties, free kicks, corners)
  status                     Show status
  swing                      Show mathematically calculated Fixture Swings
  table                      Show league table
  talisman                   Show talisman players
  team                       Show teams
  team-availability          Show team availability statistics [aliases: ta]
  team-fpl-rank              Show team FPL points rank vs Premier League position
  team-form                  Show team form based on total player form
  team-ha                    Show team home/away performance stats
  team-perf                  Show team performance based on player points per GW
  team-trend                 Show team performance trends with sparklines
  template                   Show template squad (top players by ownership per position)
  top                        Show top teams in the overall league
  transfer                   Show popular transfers
  transfer-history           Show manager's transfer history
  trend                      Show player performance trends with sparklines
  xa                         Show xA vs Assists analysis (creativity and efficiency)
  xg                         Show xG vs Goals scored analysis (finishing ability and efficiency)
  xgc                        Show xGC vs Goals Conceded analysis
  xgi                        Show xGI vs Goal Involvements (Actual G + A) analysis
  help                       Print this message or the help of the given subcommand(s)
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
