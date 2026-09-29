#!/bin/sh
# Français Pas à Pas — launcher.
# Opens the course in its own dedicated Chrome app window (no browser chrome),
# with a private profile so progress stays on this Mac.
APP_RES="$(cd "$(dirname "$0")/../Resources" && pwd)"
COURSE_DIR="$APP_RES/Course"
PROFILE_DIR="$HOME/Library/Application Support/FrenchApp/ChromeProfile"
mkdir -p "$PROFILE_DIR"
CHROME="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
if [ -x "$CHROME" ]; then
  exec "$CHROME" --app="file://$COURSE_DIR/index.html" \
    --user-data-dir="$PROFILE_DIR" --no-first-run >/dev/null 2>&1
else
  # Chrome not installed: open in the default browser instead.
  open "$COURSE_DIR/index.html"
fi
