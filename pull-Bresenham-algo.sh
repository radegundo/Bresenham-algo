#!/bin/bash

# --- Colors & Styles ---
RESET="\033[0m"
BOLD="\033[1m"
DIM="\033[2m"
GREEN="\033[38;5;82m"
BLUE="\033[38;5;39m"
YELLOW="\033[38;5;220m"
CYAN="\033[38;5;51m"
GRAY="\033[38;5;240m"
RED="\033[38;5;196m"

# --- Spinner ---
SPINNER=("⠋" "⠙" "⠹" "⠸" "⠼" "⠴" "⠦" "⠧" "⠇" "⠏")
spin_index=0

show_spinner() {
    printf "\r  ${BLUE}${SPINNER[$spin_index]}${RESET}  ${DIM}Watching for changes...${RESET}   "
    spin_index=$(( (spin_index + 1) % ${#SPINNER[@]} ))
}

clear

# --- Header ---
echo ""
echo -e "  ${BOLD}${BLUE}╔════════════════════════════════════════╗${RESET}"
echo -e "  ${BOLD}${BLUE}║${RESET}  ${BOLD}${CYAN}⬡  Bevy Sync${RESET}                         ${BOLD}${BLUE}║${RESET}"
echo -e "  ${BOLD}${BLUE}╚════════════════════════════════════════╝${RESET}"
echo ""
echo -e "  ${BOLD}Project${RESET}  ${CYAN}Bresenham-algo${RESET}"
echo -e "  ${BOLD}Source${RESET}   ${GRAY}root@192.168.1.26${RESET}"
echo -e "  ${BOLD}Target${RESET}   ${GRAY}/home/rade/prj/Bresenham-algo/${RESET}"
echo ""
echo -e "  ${GRAY}────────────────────────────────────────${RESET}"
echo -e "  ${DIM}Press Ctrl+C to stop${RESET}"
echo -e "  ${GRAY}────────────────────────────────────────${RESET}"
echo ""

PULL_COUNT=0
LAST_PULL=""

# --- Sync Loop ---
while true; do
    RSYNC_OUTPUT=$(rsync -auzi \
        --exclude 'target/' \
        --exclude '.git/' \
        --exclude '.rustc_info.json' \
        "root@192.168.1.26:/mnt/datastore/sync/prj/Bresenham-algo/" \
        "/home/rade/prj/Bresenham-algo/" 2>&1)

    CHANGED_FILES=$(echo "$RSYNC_OUTPUT" | grep -v '^$')

    if [ -n "$CHANGED_FILES" ]; then
        PULL_COUNT=$((PULL_COUNT + 1))
        LAST_PULL=$(date "+%H:%M:%S")
        echo -e "\r  ${GREEN}✔${RESET}  ${BOLD}Synced${RESET} at ${CYAN}$LAST_PULL${RESET}  ${GRAY}(pull #$PULL_COUNT)${RESET}          "
        echo "$CHANGED_FILES" | while read -r file; do
            [ -n "$file" ] && echo -e "     ${GRAY}↳ $file${RESET}"
        done
        echo ""
    fi

    for i in $(seq 1 20); do
        show_spinner
        sleep 0.1
    done
done
