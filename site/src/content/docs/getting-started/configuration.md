---
title: Configuration
description: Set your manager ID to enable personalized commands.
---

Set your manager ID to enable personalized commands like `my-team` and `history`:

```sh
fplr config set manager-id YOUR_MANAGER_ID
```

The config is stored at `~/.config/fplr/config.toml`.

## Finding your manager ID

1. Log in to [Fantasy Premier League](https://fantasy.premierleague.com/)
2. Click the "Points" link
3. Check the URL: `https://fantasy.premierleague.com/entry/{manager_id}/event/{event_id}`
