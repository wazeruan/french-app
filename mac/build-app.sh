#!/bin/sh
# Build a self-contained native macOS app bundle from the Rust app and course.
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
APP="$ROOT/FrenchApp.app"
STAGING="$ROOT/.FrenchApp.app.build.$$"
BACKUP="$ROOT/.FrenchApp.app.previous.$$"

cleanup() {
  rm -rf "$STAGING"
}
trap cleanup EXIT HUP INT TERM

cd "$ROOT"
cargo build --release --locked

mkdir -p "$STAGING/Contents/MacOS" "$STAGING/Contents/Resources"
cp "$ROOT/mac/Info.plist" "$STAGING/Contents/Info.plist"
cp "$ROOT/mac/AppIcon.icns" "$STAGING/Contents/Resources/AppIcon.icns"
cp "$ROOT/target/release/french-app" "$STAGING/Contents/MacOS/FrenchApp"
cp -R "$ROOT/course" "$STAGING/Contents/Resources/Course"
chmod +x "$STAGING/Contents/MacOS/FrenchApp"

if [ ! -s "$STAGING/Contents/Resources/Course/course.json" ]; then
  echo "Course data is missing from the app bundle." >&2
  exit 1
fi

if [ -e "$APP" ]; then
  mv "$APP" "$BACKUP"
fi
if mv "$STAGING" "$APP"; then
  rm -rf "$BACKUP"
else
  if [ -e "$BACKUP" ]; then
    mv "$BACKUP" "$APP"
  fi
  exit 1
fi

echo "Built $APP"
