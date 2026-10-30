---
title: Shell Completion
description: Generate tab completion scripts for zsh, bash and fish.
---

Generate and install tab completion scripts for your shell.

## zsh

```sh
mkdir -p ~/.zfunc
fplr completions zsh > ~/.zfunc/_fplr
```

Add the following to `~/.zshrc` if not already present:

```sh
fpath+=~/.zfunc
autoload -Uz compinit && compinit
```

## bash

```sh
fplr completions bash > ~/.local/share/bash-completion/completions/fplr
```

## fish

```sh
fplr completions fish > ~/.config/fish/completions/fplr.fish
```
