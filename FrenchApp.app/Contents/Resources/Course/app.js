/* Français Pas à Pas — Objectif TEF Canada. file:// local app, no network. */
(function () {
  "use strict";

  var STORE_KEY = "fr-pas-a-pas-tef-v1";
  var PASS_SCORE = 70;

  var view = { name: "home" };
  var quiz = null;
  var slowMode = false;

  var VIDEOS = [
    { file: "bienvenue.mp4", title: "Bienvenue !", icon: "🎬",
      desc: "Welcome to your course — what the TEF Canada is and how this course gets you there." },
    { file: "au-cafe.mp4", title: "Au café", icon: "☕",
      desc: "A café dialogue to shadow: ordering, paying, small talk. Repeat after it until it's automatic." }
  ];

  /* ---------------- store ---------------- */
  function loadStore() {
    try {
      var s = JSON.parse(localStorage.getItem(STORE_KEY));
      if (s && typeof s === "object") return { completed: s.completed || {}, scores: s.scores || {} };
    } catch (e) {}
    return { completed: {}, scores: {} };
  }
  var store = loadStore();
  function saveStore() {
    try { localStorage.setItem(STORE_KEY, JSON.stringify(store)); } catch (e) {}
  }

  /* ---------------- helpers ---------------- */
  function esc(s) {
    return String(s == null ? "" : s)
      .replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }
  function attr(s) { return esc(s).replace(/'/g, "&#39;"); }
  function $(sel, root) { return (root || document).querySelector(sel); }
  function $all(sel, root) { return Array.prototype.slice.call((root || document).querySelectorAll(sel)); }
  function getUnit(id) {
    for (var i = 0; i < COURSE.units.length; i++) if (COURSE.units[i].id === id) return COURSE.units[i];
    return null;
  }
  function unitNumber(u) { return COURSE.units.indexOf(u) + 1; }

  /* audio id -> French text index (for speech fallback) */
  var AUDIO_TEXT = {};
  function buildAudioIndex() {
    COURSE.units.forEach(function (u) {
      function add(id, fr) { if (id && fr) AUDIO_TEXT[id] = fr; }
      (u.vocab || []).forEach(function (v) { add(v.audio, v.fr); });
      (u.phrases || []).forEach(function (p) { add(p.audio, p.fr); });
      ((u.grammar || {}).examples || []).forEach(function (g) { add(g.audio, g.fr); });
      ((u.dialogue || {}).lines || []).forEach(function (d) { add(d.audio, d.fr); });
      (u.quiz || []).forEach(function (q) { if (q.type === "listening") add(q.audio, q.transcript); });
    });
  }

  /* ---------------- speech ---------------- */
  function warmVoices() {
    try {
      if ("speechSynthesis" in window) {
        speechSynthesis.getVoices();
        speechSynthesis.onvoiceschanged = function () { speechSynthesis.getVoices(); };
      }
    } catch (e) {}
  }
  function speakWithVoice(text) {
    try {
      if (!("speechSynthesis" in window)) return;
      speechSynthesis.cancel();
      var u = new SpeechSynthesisUtterance(text);
      u.lang = "fr-FR";
      u.rate = slowMode ? 0.62 : 0.95;
      var voices = speechSynthesis.getVoices() || [];
      for (var i = 0; i < voices.length; i++) {
        if ((voices[i].lang || "").toLowerCase().indexOf("fr") === 0) { u.voice = voices[i]; break; }
      }
      speechSynthesis.speak(u);
    } catch (e) {}
  }
  var currentAudio = null;
  function playAudio(id) {
    try { if (currentAudio) { currentAudio.pause(); currentAudio = null; } } catch (e) {}
    try { speechSynthesis.cancel(); } catch (e) {}
    var text = AUDIO_TEXT[id];
    var done = false;
    function fallback() {
      if (done) return; done = true;
      if (text) speakWithVoice(text);
    }
    try {
      var a = new Audio("audio/" + id + ".mp3");
      currentAudio = a;
      a.playbackRate = slowMode ? 0.7 : 1.0;
      a.onerror = fallback;
      var pr = a.play();
      if (pr && pr.catch) pr.catch(fallback);
      a.onended = function () { if (currentAudio === a) currentAudio = null; };
    } catch (e) { fallback(); }
  }

  /* ---------------- progress ---------------- */
  function completedCount() {
    var n = 0;
    COURSE.units.forEach(function (u) { if (store.completed[u.id]) n++; });
    return n;
  }
  function renderProgress() {
    var done = completedCount(), total = COURSE.units.length;
    var pt = $("#progressText"), pf = $("#progressFill");
    if (pt) pt.textContent = done + " / " + total + " units";
    if (pf) pf.style.width = (total ? (done / total) * 100 : 0) + "%";
  }

  /* ---------------- home ---------------- */
  function eiffelSVG() {
    return '<svg class="eiffel" width="86" height="120" viewBox="0 0 86 120" aria-hidden="true">' +
      '<path fill="#1f2a5a" d="M43 2 L47 22 L52 40 L58 62 L66 92 L74 116 L70 118 L60 118 L54 96 L50 118 L36 118 L32 96 L26 118 L12 118 L16 92 L28 62 L34 40 L39 22 Z"/>' +
      '<rect x="30" y="60" width="26" height="4" fill="#faf7f1"/>' +
      '<rect x="38" y="30" width="10" height="10" fill="#faf7f1"/>' +
      "</svg>";
  }

  function unitCard(u) {
    var done = !!store.completed[u.id];
    var score = store.scores[u.id];
    return '<div class="unit-card" data-open="' + attr(u.id) + '" style="border-left-color:' + esc(u.color || "#4a90d9") + '">' +
      '<span class="u-num">Unit ' + unitNumber(u) + "</span>" +
      '<div class="u-icon">' + esc(u.icon || "📘") + "</div>" +
      "<h3>" + esc(u.title) + "</h3>" +
      '<p class="u-fr">' + esc(u.titleFr) + "</p>" +
      '<p class="u-obj">' + esc((u.objectives && u.objectives[0]) || "") + "</p>" +
      '<div class="u-meta">' +
      (done ? '<span class="badge done">✓ Completed</span>' : '<span class="badge todo">Not started</span>') +
      (score != null ? '<span class="score-tag">Best quiz: ' + score + "%</span>" : "") +
      "</div></div>";
  }

  function renderHome() {
    var html = "";
    html += '<div class="hero">' + eiffelSVG() +
      "<div><h2>Bienvenue ! Objectif TEF Canada. 🇨🇦</h2>" +
      "<p>25 units from zero to exam-ready: foundations (A1), autonomy (A2), then the TEF itself — format, scoring, and section-by-section training.</p>" +
      "<ol><li>Work each level in order — nothing appears before you've learned it.</li>" +
      "<li>Tap 🔊 on any French line to hear studio audio. 🐢 replays slowly.</li>" +
      "<li>Every quiz has a 🎧 listening round, like the real exam. " + PASS_SCORE + "% completes a unit.</li></ol></div></div>";

    (COURSE.meta.levels || []).forEach(function (lv) {
      var units = COURSE.units.filter(function (u) { return u.level === lv.n; });
      if (!units.length) return;
      html += '<img class="level-hero" src="images/level' + lv.n + '.jpg" onerror="this.style.display=\'none\'">' +
        '<h2 class="section-title">Niveau ' + lv.n + " — " + esc(lv.title) +
        ' <span class="cefr-badge">' + esc(lv.cefr) + "</span></h2>";
      html += '<p class="level-desc">' + esc(lv.desc) + "</p>";
      html += '<div class="unit-grid">' + units.map(unitCard).join("") + "</div>";
    });

    html += '<h2 class="section-title">🎬 Cinéma — watch &amp; shadow</h2>';
    html += '<p class="level-desc">Short videos with French voiceover. Shadow them: pause, repeat out loud, imitate the rhythm.</p>';
    html += '<div class="unit-grid">';
    VIDEOS.forEach(function (v, i) {
      html += '<div class="unit-card" style="border-left-color:#E63946;cursor:default">' +
        '<div class="u-icon">' + v.icon + "</div>" +
        "<h3>" + esc(v.title) + "</h3>" +
        '<p class="u-obj">' + esc(v.desc) + "</p>" +
        '<video class="study-video" controls preload="none" src="videos/' + attr(v.file) + '" data-vid="' + i + '"></video>' +
        '<p class="video-missing" data-vidmsg="' + i + '" style="display:none">📥 Video file not in the app folder yet — it will appear here once added.</p>' +
        "</div>";
    });
    html += "</div>";

    $("#app").innerHTML = html;
    $all("[data-open]").forEach(function (el) {
      el.addEventListener("click", function () { openUnit(el.getAttribute("data-open")); });
    });
    // gracefully handle missing video files (file:// can't probe the folder)
    $all(".study-video").forEach(function (vid) {
      vid.addEventListener("error", function () {
        var i = vid.getAttribute("data-vid");
        vid.style.display = "none";
        var msg = document.querySelector('[data-vidmsg="' + i + '"]');
        if (msg) msg.style.display = "block";
      }, true);
    });
    renderProgress();
  }

  /* ---------------- unit page ---------------- */
  var TABS = [
    ["words", "📝 Words"],
    ["phrases", "💬 Phrases"],
    ["grammar", "📖 Grammar"],
    ["dialogue", "🎭 Dialogue"],
    ["quiz", "✅ Quiz"]
  ];

  function openUnit(id) {
    view = { name: "unit", id: id, tab: "words" };
    quiz = null;
    renderUnit();
    window.scrollTo(0, 0);
  }

  function audioRow(audioId, fr, en, tip) {
    return '<div class="row-item">' +
      '<button class="speak-btn" data-audio="' + attr(audioId || "") + '" title="Listen">🔊</button>' +
      '<div style="flex:1"><span class="fr">' + esc(fr) + "</span> " +
      '<span class="en">— ' + esc(en) + "</span>" +
      (tip ? '<span class="tip">💡 ' + esc(tip) + "</span>" : "") +
      "</div></div>";
  }

  function renderUnit() {
    var u = getUnit(view.id);
    if (!u) { renderHome(); return; }
    var lv = null;
    (COURSE.meta.levels || []).forEach(function (l) { if (l.n === u.level) lv = l; });

    var html = '<button class="back-btn" id="backBtn">← All units</button>';
    html += '<div class="unit-head" style="border-top-color:' + esc(u.color || "#4a90d9") + '">' +
      '<p class="uh-level">' + (lv ? "Niveau " + lv.n + " · " + esc(lv.title) : "") + "</p>" +
      '<h2><span style="margin-right:8px">' + esc(u.icon || "📘") + "</span>Unit " + unitNumber(u) + ": " + esc(u.title) + "</h2>" +
      '<p class="uh-fr">' + esc(u.titleFr) + "</p>" +
      "<ul>" + (u.objectives || []).map(function (o) { return "<li>" + esc(o) + "</li>"; }).join("") + "</ul></div>";
    html += '<img class="unit-hero" src="images/' + attr(u.id) + '.jpg" onerror="this.style.display=\'none\'">';

    html += '<div class="tabs">';
    TABS.forEach(function (t) {
      html += '<button class="tab' + (view.tab === t[0] ? " active" : "") + '" data-tab="' + t[0] + '">' + t[1] + "</button>";
    });
    html += "</div>";
    html += '<div id="tabBody">' + renderTab(u, view.tab) + "</div>";

    $("#app").innerHTML = html;
    $("#backBtn").addEventListener("click", function () { view = { name: "home" }; quiz = null; renderHome(); });
    $all("[data-tab]").forEach(function (el) {
      el.addEventListener("click", function () {
        view.tab = el.getAttribute("data-tab");
        if (view.tab === "quiz" && !quiz) startQuiz(u.id);
        renderUnit();
        window.scrollTo(0, 0);
      });
    });
    bindAudioButtons($("#app"));
    bindQuizButtons(u);
    renderProgress();
  }

  function renderTab(u, tab) {
    if (tab === "words") {
      var h = "";
      if (u.pronunciation) {
        h += '<div class="tip-box"><strong>🗣️ ' + esc(u.pronunciation.title || "Pronunciation") + "</strong><ul>" +
          (u.pronunciation.points || []).map(function (p) { return "<li>" + esc(p) + "</li>"; }).join("") + "</ul></div>";
      }
      h += '<div class="card"><h3>Key words</h3>';
      h += (u.vocab || []).map(function (v) { return audioRow(v.audio, v.fr, v.en, v.tip); }).join("");
      return h + "</div>";
    }
    if (tab === "phrases") {
      var p = '<div class="card"><h3>Useful phrases</h3><p style="color:var(--muted);font-size:.9rem">Say each one out loud, then tap 🔊 to compare.</p>';
      p += (u.phrases || []).map(function (x) { return audioRow(x.audio, x.fr, x.en, x.tip); }).join("");
      return p + "</div>";
    }
    if (tab === "grammar") {
      var g = u.grammar || {};
      var gh = '<div class="card"><h3>📖 ' + esc(g.title || "Grammar") + "</h3>";
      gh += '<p class="explain">' + esc(g.explanation || "") + "</p>";
      (g.examples || []).forEach(function (ex) { gh += audioRow(ex.audio, ex.fr, ex.en); });
      return gh + "</div>";
    }
    if (tab === "dialogue") {
      var d = u.dialogue || { lines: [] };
      var dh = '<div class="card"><h3>🎭 ' + esc(d.title || "Dialogue") + "</h3>";
      dh += '<p style="color:var(--muted);font-size:.9rem">Read it like a play — you be A, then B. Shadow the audio.</p><div class="dialogue">';
      (d.lines || []).forEach(function (ln) {
        dh += '<div class="bubble ' + esc(ln.speaker) + '"><span class="who">' + esc(ln.speaker === "A" ? "Person A" : "Person B") + "</span>" +
          '<div class="fr">' + esc(ln.fr) +
          ' <button class="speak-btn" data-audio="' + attr(ln.audio || "") + '" title="Listen">🔊</button></div>' +
          '<div class="en">' + esc(ln.en) + "</div></div>";
      });
      return dh + "</div></div>";
    }
    if (tab === "quiz") return renderQuiz(u);
    return "";
  }

  /* ---------------- quiz ---------------- */
  function startQuiz(unitId) {
    quiz = { unitId: unitId, idx: 0, correct: 0, picked: -1, finished: false };
  }

  function renderQuiz(u) {
    var qs = u.quiz || [];
    if (!qs.length) return '<div class="card"><p>No quiz for this unit yet.</p></div>';
    if (quiz.finished) return renderQuizResult(u, qs);

    var q = qs[quiz.idx];
    var h = '<div class="card">';
    h += '<p class="quiz-count">Question ' + (quiz.idx + 1) + " of " + qs.length +
      (q.type === "listening" ? ' · <span class="cefr-badge">🎧 listening</span>' : "") + "</p>";
    h += '<p class="quiz-q">' + esc(q.q) + "</p>";
    if (q.type === "listening" && q.audio) {
      h += '<button class="listen-btn" data-audio="' + attr(q.audio) + '">🎧 Écouter (listen)</button>';
      h += '<p class="listen-hint">You can replay it as many times as you like — in the real exam, only once!</p>';
    }
    q.choices.forEach(function (c, i) {
      var cls = "choice";
      if (quiz.picked >= 0) {
        if (i === q.answer) cls += " correct";
        else if (i === quiz.picked) cls += " wrong";
        else cls += " dim";
      }
      var dis = quiz.picked >= 0 ? " disabled" : "";
      h += '<button class="' + cls + '" data-choice="' + i + '"' + dis + ">" + esc(c) + "</button>";
    });
    if (quiz.picked >= 0) {
      var ok = quiz.picked === q.answer;
      h += '<div class="feedback ' + (ok ? "good" : "bad") + '"><strong>' + (ok ? "✓ Correct !" : "✗ Not quite.") + "</strong><br>" + esc(q.explain || "");
      if (q.type === "listening" && q.transcript) {
        h += '<br><br><em>You heard:</em> <span class="fr-heard">' + esc(q.transcript) + "</span> " +
          '<button class="speak-btn" data-audio="' + attr(q.audio || "") + '" title="Listen again" style="width:30px;height:30px;font-size:.85rem">🔊</button>';
      }
      h += "</div>";
      h += '<div class="quiz-nav"><button class="btn" data-next="1">' + (quiz.idx + 1 < qs.length ? "Next →" : "See results →") + "</button></div>";
    }
    return h + "</div>";
  }

  function renderQuizResult(u, qs) {
    var pct = Math.round((quiz.correct / qs.length) * 100);
    var passed = pct >= PASS_SCORE;
    if (passed) store.completed[u.id] = true;
    if (store.scores[u.id] == null || pct > store.scores[u.id]) store.scores[u.id] = pct;
    saveStore();

    var h = '<div class="card"><div class="quiz-result">';
    h += '<div class="big">' + (passed ? "🎉" : "💪") + "</div>";
    h += "<h3>You scored " + quiz.correct + " / " + qs.length + " (" + pct + "%)</h3>";
    h += passed
      ? "<p>Bravo ! Unit complete. 🎊</p>"
      : "<p>Good effort ! Review the unit and retry — " + PASS_SCORE + "% completes the unit.</p>";
    h += '<div style="margin-top:18px;display:flex;gap:10px;justify-content:center;flex-wrap:wrap">' +
      '<button class="btn secondary" data-retry="1">🔁 Retry quiz</button>' +
      '<button class="btn" data-backhome="1">All units</button></div>';
    return h + "</div></div>";
  }

  function bindQuizButtons(u) {
    $all("[data-choice]").forEach(function (el) {
      el.addEventListener("click", function () {
        if (!quiz || quiz.picked >= 0) return;
        var i = parseInt(el.getAttribute("data-choice"), 10);
        quiz.picked = i;
        var q = (u.quiz || [])[quiz.idx] || {};
        if (i === q.answer) quiz.correct++;
        refreshTab(u);
      });
    });
    $all("[data-next]").forEach(function (el) {
      el.addEventListener("click", function () {
        quiz.idx++;
        quiz.picked = -1;
        if (quiz.idx >= (u.quiz || []).length) quiz.finished = true;
        refreshTab(u);
        window.scrollTo(0, 0);
      });
    });
    $all("[data-retry]").forEach(function (el) {
      el.addEventListener("click", function () { startQuiz(u.id); refreshTab(u); });
    });
    $all("[data-backhome]").forEach(function (el) {
      el.addEventListener("click", function () { view = { name: "home" }; quiz = null; renderHome(); window.scrollTo(0, 0); });
    });
  }

  function refreshTab(u) {
    var body = $("#tabBody");
    if (!body) return;
    body.innerHTML = renderTab(u, view.tab);
    bindAudioButtons(body);
    bindQuizButtons(u);
    renderProgress();
  }

  function bindAudioButtons(root) {
    $all("[data-audio]", root).forEach(function (el) {
      el.addEventListener("click", function (ev) {
        ev.stopPropagation();
        var id = el.getAttribute("data-audio");
        if (id) playAudio(id);
      });
    });
  }

  /* ---------------- boot ---------------- */
  document.addEventListener("DOMContentLoaded", function () {
    warmVoices();
    if (typeof COURSE === "undefined") {
      $("#app").innerHTML = '<div class="card"><h3>⚠️ Course data missing</h3><p>Could not find <code>course.js</code>. Make sure it sits next to <code>index.html</code>.</p></div>';
      return;
    }
    buildAudioIndex();
    var rt = $("#rateToggle");
    rt.addEventListener("click", function () {
      slowMode = !slowMode;
      rt.textContent = slowMode ? "🐢 Slow: on" : "🐢 Slow: off";
      rt.classList.toggle("on", slowMode);
    });
    renderHome();
  });
})();
