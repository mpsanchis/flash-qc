#!/bin/bash

# SESSION CREATION
SESSION_NAME="flashqc"
## Kill existing session if it exists
tmux kill-session -t $SESSION_NAME 2>/dev/null
## Create new session
tmux new-session -d -s $SESSION_NAME
## Enable mouse support for this session (scroll across logs)
tmux set-option -t $SESSION_NAME mouse on
tmux set-option -t $SESSION_NAME history-limit 10000

# PANE CREATION
## Split horizontally to create bottom pane across full width
tmux split-window -v

## Split window vertically
tmux select-pane -t 0
tmux split-window -h

## Make the bottom pane smaller (about 20% of screen height)
tmux resize-pane -t 2 -y 20% 

## Set pane names
tmux select-pane -t 0 -T "backend"
tmux select-pane -t 1 -T "frontend"
tmux select-pane -t 2 -T "control"
## Serve backend and frontend
tmux send-keys -t 0 "cargo run" Enter
tmux send-keys -t 1 "pnpm --filter \"flash-qc\" dev" Enter
tmux send-keys -t 2 "echo \"Use this panel to run any commands. Run 'mise stop' to exit. \"" Enter

# MOVE TO CONTROL AND ATTACH TO SESSION
tmux select-pane -t 2
tmux attach-session -t $SESSION_NAME
