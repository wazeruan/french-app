# Français Pas à Pas — Objectif TEF Canada

A private French course for TEF Canada and TCF Canada learners, taught in English
from beginner level. Lessons, progress, audio, and illustrations work locally;
the app does not require an account or send learning data online.

## The Mac app

Requires an Apple Silicon Mac running macOS 11 or later.

Open `FrenchApp.app`. The Rust app presents the 25 lessons in a focused native
window, with guided lesson navigation, vocabulary and phrase audio, slow
playback, quizzes, progress tracking, and a responsive home screen. Lesson
progress is saved to
`~/Library/Application Support/FrenchApp/progress.json`.

The two course videos open in the macOS default video player. The original
browser course is also bundled under `Contents/Resources/Course`. To move
existing browser progress into the native app, open it in the browser profile
where you studied. **Open classic course** uses the app's dedicated Chrome
profile from the earlier Mac launcher; **Open in default browser** uses your
current default browser profile. If the earlier launcher used a different
browser, choose **Find course file…** and use Finder's **Open With** menu to
open it in that browser. Each route needs the same browser profile that holds
your old progress. Then:

1. Choose **Export backup** on the classic course page.
2. Return to the native app and choose **Import progress…**.

The classic browser course remains available by opening `course/index.html`.
Its browser progress remains in that browser's local storage.

## Rebuild

Requirements: Rust stable with Cargo, Node.js (only when regenerating course
data), and macOS.

```sh
cargo check --locked
sh mac/build-app.sh
```

`mac/build-app.sh` compiles the Rust release binary and assembles
`FrenchApp.app` with the local course, audio, images, and videos. Course JSON is
exported from the browser course data when the source changes:

```sh
node course/tools/export-course-json.js
```

## Source layout

- `src/` — Rust desktop app, course model, audio playback, and progress storage
- `course/course.json` — native app's serialized course data
- `course/` — browser edition and shared offline media
- `mac/` — app bundle metadata, icon, and build script
