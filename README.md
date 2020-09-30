# tmux_chooser

Choose a tmux session to attach to or create a new one.
Inspired by [this shell script](https://cedaei.com/posts/ideas-from-my-dev-setup-always-tmux/).

## Install

```
cargo install tmux_chooser
```

## Deviations

This tmux_chooser deviates from the shell script in the following ways:

- if an invalid session number is entered, the user is asked for one again
- if the given session name already exists, we connect to the session instead of erroring out
- sessions are formatted in a custom way (e.g. displaying the duration for which a session has lived)
