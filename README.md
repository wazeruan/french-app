# Français Pas à Pas — Objectif TEF Canada

A private French course for TEF Canada and TCF Canada learners, taught in English
from beginner level. Lessons, progress, audio, and illustrations work locally;
the app does not require an account or send learning data online.

## The Mac app

Requires an Apple Silicon Mac running macOS 11 or later.

Open `FrenchApp.app`. The Rust app presents 25 lessons in a flat, readable
workspace, with search and level/status filters, a guided five-section lesson
flow, audio with visible stop controls, quizzes, and a dated learning record.
Navigation remains available in smaller windows. Preferences offer three reading
sizes, a saved audio pace, and a daily vocabulary review goal.

Save individual vocabulary words, or add a lesson's vocabulary to your personal
review deck. Reveal each word's meaning, then choose **Remembered** or **Practice
again**. Remembered words return after 1, 3, 7, 14, then 30 days; practice-again
words remain due. Reviewing words records a study day and contributes to review
achievements. Completed ratings are saved immediately. Lessons resume at the last
visited section; unfinished quizzes and review sessions survive navigation during
the current app session.

Opening a lesson records a local study day; each finished quiz adds a dated score
record. Older imported lesson completions count toward course milestones, while
day and quiz history begins in the native app. Course progress is saved to
`~/Library/Application Support/FrenchApp/progress.json`; review schedules and
preferences are saved alongside it in `learning.json`.

Use **Preferences → Export backup** to save a JSON containing course progress,
quiz history, the review deck, schedules, and preferences. **Import backup** merges
learning records and restores backed-up preferences. Classic course backups remain
compatible. Unreadable saved files are preserved, and recovery retains a separate
copy of the original.

Shortcuts: **⌘1** Learn, **⌘2** Review, **⌘3** Record, **⌘F** lesson search,
**Escape** stop audio. In review: **Space** reveal, **J** practice again, **K**
remembered.

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
2. Return to the native app and choose **Preferences → Import backup**.

The classic browser course remains available by opening `course/index.html`.
Its browser progress remains in that browser's local storage.

## Rebuild

Requirements: Rust stable with Cargo, Node.js (only when regenerating course
data), and macOS.

```sh
cargo check --locked
cargo test --locked
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
- `src/interface.rs` — responsive workspace, lesson, review, and preferences UI
- `src/learning.rs` — review scheduling and saved preferences
- `course/course.json` — native app's serialized course data
- `course/` — browser edition and shared offline media
- `mac/` — app bundle metadata, icon, and build script

For an isolated development run, set `FRENCH_APP_DATA_DIR` to a separate absolute
directory before launching the executable. This directs save files away from the
normal user profile. Tests use temporary data and never modify the normal profile.
