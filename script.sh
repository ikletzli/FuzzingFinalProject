# Create a new session named "newsess", split panes and change directory in each
tmux new-session -d -s newsess
tmux send-keys -t newsess "afl-fuzz -t 20000 -g 8 -G 8 -Q -i in -o out -M fuzzer01 ../target/debug/theseus_gui @@" Enter
tmux split-window -h -t newsess
tmux send-keys -t newsess "afl-fuzz -t 20000 -P exploit -g 8 -G 8 -Q -i in -o out -S fuzzer02 ../target/debug/theseus_gui @@" Enter
tmux split-window -v -p 75 -t newsess
tmux send-keys -t newsess "afl-fuzz -t 20000 -p coe -g 8 -G 8 -Q -i in -o out -S fuzzer03 ../target/debug/theseus_gui @@" Enter
tmux split-window -v -p 66 -t newsess
tmux send-keys -t newsess "afl-fuzz -t 20000 -p fast -g 8 -G 8 -Q -i in -o out -S fuzzer04 ../target/debug/theseus_gui @@" Enter
tmux split-window -v -p 50 -t newsess
tmux send-keys -t newsess "afl-fuzz -t 20000 -L 0 -g 8 -G 8 -Q -i in -o out -S fuzzer05 ../target/debug/theseus_gui @@" Enter
tmux select-pane -t newsess:0.0
tmux split-window -v -p 75 -t newsess
tmux send-keys -t newsess "afl-fuzz -t 20000 -c 0 -g 8 -G 8 -Q -i in -o out -S fuzzer06 ../target/debug/theseus_gui @@" Enter
# tmux split-window -v -p 66 -t newsess
# tmux send-keys -t newsess "afl-fuzz -t 20000 -g 8 -G 8 -Q -i in -o out ../target/debug/theseus_gui @@" Enter
# tmux split-window -v -p 50 -t newsess
# tmux send-keys -t newsess "afl-fuzz -t 20000 -g 8 -G 8 -Q -i in -o out ../target/debug/theseus_gui @@" Enter
# Set pane synchronization
tmux set-window-option -t newsess:0 synchronize-panes on
tmux attach -t newsess