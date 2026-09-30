# Français Pas à Pas — Objectif TEF Canada

A private, fully local French course for one learner: from complete beginner to the
**TEF Canada / TCF Canada** immigration exam (target: CLB 7+ for CRS points).
Taught in English. No accounts, no tracking, no internet needed. The recommended
desktop experience is `../FrenchApp.app`; this folder also contains the classic
browser edition.

## How to open it

Double-click **`index.html`** to use the classic browser edition offline. Its
progress is stored in that browser's local storage. Choose **Export backup** in
the header to move it into the native app, then use **Import progress…** there.

## The course

- **Niveau 1 — Les fondations (A1):** 10 units. The sounds of French, greetings,
  numbers 0–100, family, food, café, time, être/avoir, places & directions.
  Heavy listening training from day one.
- **Niveau 2 — L'autonomie (A2):** 10 units. Passé composé vs imparfait, futur,
  pronouns, reflexive verbs, giving opinions — the grammar the TEF actually tests.
- **Niveau 3 — Objectif TEF Canada (B1 · examen):** 5 units. The exam decoded
  (format, timing, CLB scoring), then section-by-section training: listening,
  reading, speaking (with model answers to shadow), writing (with model texts).

Every unit has: key words, useful phrases, one grammar point, a dialogue,
a pronunciation tip box — and a quiz with a **🎧 listening round** like the real
exam. 70% completes a unit. Progress is saved in the browser (localStorage).

## Suggested routine

One unit per sitting, in order. Say every French line out loud before tapping
the speaker — then compare.

## Media

- **Audio:** every French word, phrase, example, dialogue line, and listening
  quiz item has studio-generated French audio (`audio/<id>.mp3`, ~860 clips).
  Tap 🔊 anywhere. The 🐢 toggle replays slowly. If an audio file is missing,
  the app falls back to the device's French voice.
- **Images:** study illustrations (`images/`) — level banners plus scenes for
  family, food, café, town directions, work, travel, doctor, exam, speaking.
- **Videos:** `videos/bienvenue.mp4` (course welcome) and
  `videos/au-cafe.mp4` (café dialogue) — French voiceover included. The native
  app opens them in macOS's default video player. Shadow them: pause, repeat out
  loud, imitate the rhythm.

## Files

- `index.html` — app entry point (open this)
- `styles.css` — styling
- `app.js` — app logic
- `course.js` — course data loaded by the browser app
- `levels/n1.js`, `levels/n2.js`, `levels/n3.js` — legacy level source modules
- `tools/export-course-json.js` — converts `course.js` to the native app's
  `course.json` (`node tools/export-course-json.js`)
- `audio/`, `images/`, `videos/` — media assets

## Rebuilding

The browser app reads `course.js` directly. After updating it, run
`node tools/export-course-json.js` to refresh the native app data. The previous
`tools/build-course.js` and audio-generation scripts are not included in this
checkout, so changes in `levels/` do not regenerate `course.js` automatically.
Refresh the browser after editing `course.js`.

*Bonne chance — et amusez-vous bien !* 🎉
