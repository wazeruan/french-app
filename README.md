# Français Pas à Pas — Objectif TEF Canada

A private, fully local French course for the TEF Canada / TCF Canada immigration
exam (target: CLB 7+ for CRS points). Taught in English, from complete beginner.
No accounts, no tracking, no internet needed.

## What this is

- **`FrenchApp.app`** — the Mac app. Double-click to launch; the course opens in
  its own dedicated window with a private browser profile (progress stays on
  your Mac). No installation needed — it runs from anywhere, e.g.
  `/Volumes/Samsung/Projects`.
- **`course/`** — the course itself: 25 units in 3 levels (A1 foundations,
  A2 autonomy, TEF exam training), ~860 studio-generated French audio clips,
  study illustrations, and 2 videos with French voiceover. Open
  `course/index.html` directly in any browser if you prefer.
- **`mac/`** — the app-bundle sources: `Info.plist`, the launcher script,
  the icon (`AppIcon.icns`), and `build-app.sh`.

## Rebuilding the app

After changing anything in `course/` (or `mac/`):

```sh
sh mac/build-app.sh
```

This reassembles `FrenchApp.app` from `course/` + `mac/`. No compiler needed.

To regenerate the course data file after editing `course/levels/`:

```sh
cd course && node tools/build-course.js
```

## Version history

See commit log.
