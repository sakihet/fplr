---
title: Color Output
description: When fplr colors its output and how to turn it off.
---

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
