# Français Pas à Pas — Objectif TEF Canada

A private, fully local French course for one learner: from complete beginner to the
**TEF Canada / TCF Canada** immigration exam (target: CLB 7+ for CRS points).
Taught in English. No accounts, no tracking, no internet needed — just open
`index.html` in a browser.

## How to open it

Double-click **`index.html`**. That's it. It opens in your browser and works fully offline.

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
  `videos/au-cafe.mp4` (café dialogue) — French voiceover included. Shadow them:
  pause, repeat out loud, imitate the rhythm.

## Files

- `index.html` — app entry point (open this)
- `styles.css` — styling
- `app.js` — app logic
- `course.js` — built course data (generated from `levels/` — don't edit by hand)
- `levels/n1.js`, `levels/n2.js`, `levels/n3.js` — course content source
- `tools/build-course.js` — rebuilds `course.js` with validation
  (`node tools/build-course.js`)
- `tools/gen-audio.py` — regenerates audio from the content files
- `audio/`, `images/`, `videos/` — media assets

## Rebuilding

After editing anything in `levels/`, run `node tools/build-course.js` to
regenerate `course.js` (it validates audio ids, quiz structure, and content
fields). Then refresh the browser.

*Bonne chance — et amusez-vous bien !* 🎉
