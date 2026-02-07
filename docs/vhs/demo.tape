# TetroTime demo recording
#
# Records a terminal session demonstrating TetroTime's different modes
# (clock, stopwatch, countdown) and colorschemes.
#
# Usage: vhs demo.tape
# (Prefer using record.sh which handles build and cleanup)

Output demo.webm
Output demo.mp4

Set Shell "zsh"
Set FontFamily "JetBrainsMono NFM SemiBold"
Set FontSize 32
Set Width 1800
Set Height 800
Set Theme { "name": "TetroTime", "black": "#0f0f14", "red": "#ec4899", "green": "#5eca89", "yellow": "#fdd877", "blue": "#65aef7", "magenta": "#ec4899", "cyan": "#43c1be", "white": "#e5e5e5", "brightBlack": "#9ca3af", "brightRed": "#f472b6", "brightGreen": "#5eca89", "brightYellow": "#fdd877", "brightBlue": "#65aef7", "brightMagenta": "#f472b6", "brightCyan": "#43c1be", "brightWhite": "#ffffff", "background": "#0f0f14", "foreground": "#e5e5e5", "selection": "#3b1f33", "cursor": "#ec4899" }
Set Padding 20

# Hidden setup - set prompt and clear screen
Set TypingSpeed 0
Hide
Type "PROMPT='%F{#ec4899}> %f'"
Enter
Sleep 10ms
Type "clear"
Enter
Sleep 30ms
Show
Set TypingSpeed 47ms

# == VISIBLE RECORDING STARTS ==

Sleep 500ms

# Scene 1: Clock mode with classic Tetris colors (default)
Type "tetrotime"
Enter
Sleep 13s
Type "q"
Sleep 500ms

# Scene 2: Clock mode with warm colorscheme
Type "tetrotime -s warm"
Enter
Sleep 13s
Type "q"
Sleep 500ms

# Scene 3: Clock mode with neon colorscheme
Type "tetrotime -s neon"
Enter
Sleep 13s
Type "q"
Sleep 500ms

# Scene 4: Stopwatch mode with matrix colorscheme
Type "tetrotime --stopwatch -s matrix"
Enter
Sleep 13s
Type "q"
Sleep 500ms

# Scene 5: Countdown mode with ocean colorscheme
Type "tetrotime --countdown 00:01:30 -s ocean"
Enter
Sleep 13s
Type "q"

Sleep 3s
