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
  config                     Manage configuration
  dream-team                 Show dream team
  fixture                    Show upcoming fixtures
  fixture-difficulty-rating  Show fixture difficulty rating
  gameweek                   Show gameweeks
  history                    Show manager's season history
  live                       Show live player stats for a specific event
  my-team                    Show my team
  player                     Show players
  pick                       Show a manager's team picks for a specific event
  player-summary             Show player summary
  set-piece                  Show set piece takers (penalties, free kicks, corners)
  status                     Show status
  table                      Show league table
  team                       Show teams
  team-perf                  Show team performance based on player points per GW
  transfer                   Show popular transfers
  help                       Print this message or the help of the given subcommand(s)
```
