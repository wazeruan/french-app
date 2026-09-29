#!/bin/sh
# Rebuild FrenchApp.app from course/ + mac/. Run from anywhere.
set -e
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
APP="$ROOT/FrenchApp.app"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$ROOT/mac/Info.plist" "$APP/Contents/Info.plist"
cp "$ROOT/mac/launcher.sh" "$APP/Contents/MacOS/FrenchApp"
cp "$ROOT/mac/AppIcon.icns" "$APP/Contents/Resources/AppIcon.icns"
cp -R "$ROOT/course" "$APP/Contents/Resources/Course"
chmod +x "$APP/Contents/MacOS/FrenchApp"
echo "Built $APP"
