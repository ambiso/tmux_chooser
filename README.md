# tmux_chooser

Inspired by https://cedaei.com/posts/ideas-from-my-dev-setup-always-tmux/

This tmux_chooser deviates from the shell script in the following ways:

- if an invalid session number is entered, the user is asked for one again
- if the given session name already exists, we connect to the session instead of erroring out
- sessions are formatted in a custom way
