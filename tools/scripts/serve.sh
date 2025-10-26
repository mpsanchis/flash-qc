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

# Get the pane base index for this session to handle custom configurations
BASE_INDEX=$(tmux show-option -gv pane-base-index 2>/dev/null || echo 0)
PANE_0=$BASE_INDEX
PANE_1=$((BASE_INDEX + 1))
PANE_2=$((BASE_INDEX + 2))

# PANE CREATION
## Split horizontally to create bottom pane across full width
tmux split-window -v

## Split window vertically
tmux select-pane -t "$PANE_0"
tmux split-window -h

## Make the bottom pane smaller (about 20% of screen height)
tmux resize-pane -t "$PANE_2" -y 20%

## Set pane names
tmux select-pane -t "$PANE_0" -T "backend"
tmux select-pane -t "$PANE_1" -T "frontend"
tmux select-pane -t "$PANE_2" -T "control"
## Serve backend and frontend
tmux send-keys -t "$PANE_0" "cargo run" Enter
tmux send-keys -t "$PANE_1" "pnpm --filter \"@flash-qc/frontend\" dev" Enter
tmux send-keys -t "$PANE_2" "echo \"Use this panel to run any commands. Run 'mise stop' to exit. \"" Enter

# MOVE TO CONTROL AND ATTACH TO SESSION
tmux select-pane -t "$PANE_2"
tmux attach-session -t $SESSION_NAME
