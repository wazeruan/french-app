use crate::{
    audio::AudioEngine,
    learning::{LearningState, word_key},
    model::*,
    store,
};
use eframe::egui::{
    self, Align, Align2, Button, Color32, CornerRadius, FontId, Frame, Image, Label, Layout,
    Margin, Panel, RichText, ScrollArea, Sense, Stroke, Vec2,
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

const PASS_SCORE: u32 = 70;

#[path = "interface.rs"]
mod interface;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Page {
    Home,
    Journey,
    Review,
    Preferences,
    Lesson(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FocusTarget {
    Home,
    Journey,
    Lesson,
    Review,
    Preferences,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum LessonTab {
    Words,
    Phrases,
    Grammar,
    Dialogue,
    Quiz,
}

impl LessonTab {
    const ALL: [Self; 5] = [
        Self::Words,
        Self::Phrases,
        Self::Grammar,
        Self::Dialogue,
        Self::Quiz,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::Words => "Vocabulary",
            Self::Phrases => "Phrases",
            Self::Grammar => "Grammar",
            Self::Dialogue => "Dialogue",
            Self::Quiz => "Practice quiz",
        }
    }
}

#[derive(Clone, Debug)]
struct QuizSession {
    unit_index: usize,
    question_index: usize,
    correct: usize,
    selected: Option<usize>,
    finished: bool,
    saved: bool,
    save_error: Option<String>,
    focus_first_answer: bool,
    focus_next_question: bool,
    focus_result: bool,
}

#[derive(Clone, Debug)]
struct JourneyCardInfo {
    title: String,
    detail: String,
    icon: &'static str,
    current: u32,
    target: u32,
    unlocked: bool,
}

#[derive(Clone, Debug)]
struct ReviewSession {
    keys: Vec<String>,
    position: usize,
    revealed: bool,
    remembered: usize,
}

impl ReviewSession {
    fn reconcile(&mut self, available: &HashSet<String>) {
        self.position = self.position.min(self.keys.len());
        let previous = self.keys.get(self.position).cloned();
        let remaining = self.keys.split_off(self.position);
        self.keys
            .extend(remaining.into_iter().filter(|key| available.contains(key)));
        if self.keys.get(self.position) != previous.as_ref() {
            self.revealed = false;
        }
    }
}

impl QuizSession {
    fn new(unit_index: usize) -> Self {
        Self {
            unit_index,
            question_index: 0,
            correct: 0,
            selected: None,
            finished: false,
            saved: false,
            save_error: None,
            focus_first_answer: true,
            focus_next_question: false,
            focus_result: false,
        }
    }
}

pub struct FrenchApp {
    course: Course,
    progress: Progress,
    resources: PathBuf,
    today: Option<String>,
    today_checked: Instant,
    page: Page,
    pending_page_focus: Option<FocusTarget>,
    lesson_tab: LessonTab,
    quiz: Option<QuizSession>,
    slow_audio: bool,
    pending_level_scroll: Option<u32>,
    textures: HashMap<String, egui::TextureHandle>,
    unavailable_textures: HashSet<String>,
    audio: AudioEngine,
    notice: Option<(String, Instant, bool)>,
    notice_sticky: bool,
    progress_load_error: bool,
    learning: LearningState,
    learning_load_error: bool,
    learning_save_failed: bool,
    paused_quizzes: HashMap<usize, QuizSession>,
    audio_active: bool,
    playing_word: Option<String>,
    search_query: String,
    level_filter: Option<u32>,
    status_filter: u8,
    search_focus: bool,
    review_session: Option<ReviewSession>,
}

impl FrenchApp {
    pub fn new(ctx: &egui::Context) -> Result<Self, String> {
        apply_theme(ctx);
        let resources = resolve_resources();
        let data = fs::read(resources.join("course.json")).map_err(|error| {
            format!(
                "Could not open course.json in {}: {error}",
                resources.display()
            )
        })?;
        let course: Course = serde_json::from_slice(&data)
            .map_err(|error| format!("The course data is not valid JSON: {error}"))?;
        if course.units.is_empty() {
            return Err("The course file has no lessons.".to_owned());
        }
        let (progress, load_notice) = store::load_progress();
        let progress_load_error = load_notice.is_some();
        let (learning, learning_notice) = store::load_learning();
        let learning_load_error = learning_notice.is_some();
        ctx.set_zoom_factor(learning.text_scale);
        let load_notice = load_notice.or(learning_notice);
        let slow_audio = learning.slow_audio;
        Ok(Self {
            course,
            progress,
            resources,
            today: store::local_day_key(),
            today_checked: Instant::now(),
            page: Page::Home,
            pending_page_focus: None,
            lesson_tab: LessonTab::Words,
            quiz: None,
            slow_audio,
            pending_level_scroll: None,
            textures: HashMap::new(),
            unavailable_textures: HashSet::new(),
            audio: AudioEngine::new(),
            notice: load_notice.map(|text| (text, Instant::now(), false)),
            notice_sticky: progress_load_error || learning_load_error,
            progress_load_error,
            learning,
            learning_load_error,
            learning_save_failed: false,
            paused_quizzes: HashMap::new(),
            audio_active: false,
            playing_word: None,
            search_query: String::new(),
            level_filter: None,
            status_filter: 0,
            search_focus: false,
            review_session: None,
        })
    }

    fn done_count(&self) -> usize {
        self.progress.completed_count(&self.course)
    }

    fn recognition_snapshot(&self) -> Vec<(bool, String)> {
        milestone_cards(&self.course, &self.progress)
            .into_iter()
            .filter(|card| card.unlocked)
            .map(|card| (false, card.title))
            .chain(
                achievement_cards(&self.course, &self.progress)
                    .into_iter()
                    .chain(learning_achievements(&self.learning))
                    .filter(|card| card.unlocked)
                    .map(|card| (true, card.title)),
            )
            .collect()
    }

    fn new_recognition_message(&self, previous: &[(bool, String)]) -> Option<String> {
        let current = self.recognition_snapshot();
        current
            .iter()
            .find(|award| award.0 && !previous.contains(award))
            .or_else(|| {
                current
                    .iter()
                    .find(|award| !award.0 && !previous.contains(award))
            })
            .map(|(achievement, title)| {
                format!(
                    "{} unlocked: {title}!",
                    if *achievement {
                        "Achievement"
                    } else {
                        "Milestone"
                    }
                )
            })
    }

    fn next_index(&self) -> Option<usize> {
        self.progress
            .next_unit(&self.course)
            .and_then(|next| self.course.units.iter().position(|unit| unit.id == next.id))
    }

    fn refresh_today(&mut self) {
        if self.today_checked.elapsed() >= Duration::from_secs(60) {
            self.refresh_today_now();
        }
    }

    fn refresh_today_now(&mut self) {
        self.today = store::local_day_key();
        self.today_checked = Instant::now();
    }

    fn record_study_day(&mut self) {
        self.refresh_today_now();
        let Some(today) = self.today.as_deref() else {
            return;
        };
        let previous_awards = self.recognition_snapshot();
        let mut updated = self.progress.clone();
        if !updated.record_study_day(today) {
            return;
        }
        if self.progress_load_error {
            self.progress = updated;
            return;
        }
        match store::save_progress(&updated) {
            Ok(()) => {
                self.progress = updated;
                if let Some(message) = self.new_recognition_message(&previous_awards) {
                    self.set_notice(message, true);
                }
            }
            Err(error) => self.set_sticky_notice(error),
        }
    }

    fn open_unit(&mut self, index: usize) {
        let Some(unit) = self.course.units.get(index) else {
            return;
        };
        if !self.progress.can_open(&self.course, unit) {
            self.set_notice("Pass the previous quiz to unlock this lesson.", false);
            return;
        }
        let unit_id = unit.id.clone();
        self.record_study_day();
        self.audio.stop();
        self.page = Page::Lesson(index);
        self.pending_page_focus = Some(FocusTarget::Lesson);
        let mut learning = self.learning.clone();
        let same_lesson = learning.last_unit.as_deref() == Some(&unit_id);
        learning.last_unit = Some(unit_id);
        if !same_lesson {
            learning.last_section = 0;
        }
        self.lesson_tab = LessonTab::ALL[learning.last_section.min(4)];
        if self
            .quiz
            .as_ref()
            .is_some_and(|quiz| quiz.unit_index != index)
            && let Some(quiz) = self.quiz.take()
        {
            self.paused_quizzes.insert(quiz.unit_index, quiz);
        }
        if self.quiz.is_none() {
            self.quiz = self.paused_quizzes.remove(&index);
        }
        self.save_learning(learning);
    }

    fn return_home(&mut self) {
        self.audio.stop();
        self.page = Page::Home;
        self.pending_page_focus = Some(FocusTarget::Home);
    }

    fn set_notice(&mut self, message: impl Into<String>, success: bool) {
        if self.notice_sticky {
            return;
        }
        self.notice = Some((message.into(), Instant::now(), success));
        self.notice_sticky = false;
    }

    fn set_sticky_notice(&mut self, message: impl Into<String>) {
        self.learning_save_failed = false;
        self.notice = Some((message.into(), Instant::now(), false));
        self.notice_sticky = true;
    }

    fn save_learning(&mut self, learning: LearningState) -> bool {
        match store::save_learning(&learning) {
            Ok(()) => {
                if self.learning_save_failed
                    && !self.progress_load_error
                    && !self.learning_load_error
                {
                    self.notice = None;
                    self.notice_sticky = false;
                }
                self.learning_save_failed = false;
                self.learning = learning;
                true
            }
            Err(error) => {
                self.set_sticky_notice(error);
                self.learning_save_failed = true;
                false
            }
        }
    }

    fn persist_quiz_result(
        &mut self,
        unit_id: &str,
        score: u32,
        passed: bool,
    ) -> Result<Option<String>, String> {
        self.refresh_today_now();
        let previous_awards = self.recognition_snapshot();
        let mut updated = self.progress.clone();
        updated.record_quiz(unit_id, score, passed, self.today.as_deref());
        store::save_progress(&updated)?;
        self.progress = updated;
        Ok(self.new_recognition_message(&previous_awards))
    }

    fn play_audio(&mut self, line: &StudyLine) {
        match self.audio.play(
            &self.resources,
            line.audio.as_deref(),
            &line.fr,
            self.slow_audio,
        ) {
            Ok(()) => {
                self.audio_active = true;
                self.playing_word = Some(line.fr.clone());
            }
            Err(error) => self.set_notice(error, false),
        }
    }

    fn play_text_audio(&mut self, audio: Option<&str>, text: &str) {
        match self
            .audio
            .play(&self.resources, audio, text, self.slow_audio)
        {
            Ok(()) => {
                self.audio_active = true;
                self.playing_word = None;
            }
            Err(error) => self.set_notice(error, false),
        }
    }

    fn load_texture(
        &mut self,
        ctx: &egui::Context,
        key: &str,
        path: &Path,
    ) -> Option<egui::TextureHandle> {
        if let Some(texture) = self.textures.get(key) {
            return Some(texture.clone());
        }
        if self.unavailable_textures.contains(key) {
            return None;
        }
        let image = match image::open(path) {
            Ok(image) => image.thumbnail(1024, 768).to_rgba8(),
            Err(_) => {
                self.unavailable_textures.insert(key.to_owned());
                return None;
            }
        };
        let size = [image.width() as usize, image.height() as usize];
        let color = egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw());
        let texture = ctx.load_texture(key, color, egui::TextureOptions::LINEAR);
        self.textures.insert(key.to_owned(), texture.clone());
        Some(texture)
    }

    fn import_progress(&mut self) {
        let picked = rfd::FileDialog::new()
            .add_filter("French course backup", &["json"])
            .pick_file();
        let Some(path) = picked else { return };
        let imported_learning = match store::import_learning(&path) {
            Ok(learning) => learning,
            Err(error) => {
                self.set_sticky_notice(format!("Could not import review data: {error}"));
                return;
            }
        };
        match store::import_progress(&path, &self.course) {
            Ok(imported) => {
                let mut merged = self.progress.clone();
                merged.merge(imported);
                let recovered = if self.progress_load_error {
                    store::recover_progress(&merged)
                } else {
                    store::save_progress(&merged).map(|()| None)
                };
                match recovered {
                    Ok(backup) => {
                        let count = merged.completed_count(&self.course);
                        self.progress = merged;
                        self.progress_load_error = false;
                        if let Some(imported_learning) = imported_learning {
                            let mut learning = self.learning.clone();
                            learning.merge(imported_learning);
                            let result = if self.learning_load_error {
                                store::recover_learning(&learning)
                            } else {
                                store::save_learning(&learning)
                            };
                            if let Err(error) = result {
                                self.set_sticky_notice(format!("Course progress imported; review data could not be restored. {error}"));
                                return;
                            }
                            self.learning = learning;
                            self.learning_load_error = false;
                            self.slow_audio = self.learning.slow_audio;
                        }
                        if self.learning_load_error {
                            self.set_sticky_notice("Course progress imported. Your review deck remains unreadable; use a full native app backup to recover it.");
                            return;
                        }
                        self.notice_sticky = false;
                        self.set_notice(
                            if let Some(backup) = backup {
                                format!(
                                    "Imported progress for {count} completed units. The unreadable file was preserved as {}.",
                                    backup.display()
                                )
                            } else {
                                format!("Imported progress for {count} completed units. Existing scores and completions were kept.")
                            },
                            true,
                        );
                    }
                    Err(error) => self.set_sticky_notice(error),
                }
            }
            Err(error) => self.set_sticky_notice(error),
        }
    }

    fn export_progress(&mut self) {
        if self.progress_load_error || self.learning_load_error {
            self.set_sticky_notice("Recover unreadable learning data before exporting a complete backup. Your original files are still preserved.");
            return;
        }
        let Some(path) = rfd::FileDialog::new()
            .add_filter("French app backup", &["json"])
            .set_file_name("french-learning-backup.json")
            .save_file()
        else {
            return;
        };
        match store::export_backup(&path, &self.progress, &self.learning) {
            Ok(()) => {
                self.notice_sticky = false;
                self.set_notice("Backup exported: course progress, review deck, learning record, and preferences.", true);
            }
            Err(error) => self.set_sticky_notice(error),
        }
    }

    fn open_default_browser_course(&mut self) {
        let index = self.resources.join("index.html");
        if !index.exists() {
            self.set_notice(
                "The classic course page is not bundled with this app.",
                false,
            );
            return;
        }

        #[cfg(target_os = "macos")]
        let result = Command::new("open").arg(&index).spawn();
        #[cfg(not(target_os = "macos"))]
        let result = Command::new("xdg-open").arg(&index).spawn();

        match result {
            Ok(_) => self.set_notice(
                "Classic course opened in your default browser. Use the same browser profile where you studied to access its saved progress.",
                true,
            ),
            Err(error) => self.set_notice(format!("Could not open the classic course: {error}"), false),
        }
    }

    fn reveal_course_file(&mut self) {
        let index = self.resources.join("index.html");
        if !index.exists() {
            self.set_notice(
                "The classic course page is not bundled with this app.",
                false,
            );
            return;
        }

        #[cfg(target_os = "macos")]
        let result = Command::new("open").arg("-R").arg(&index).spawn();
        #[cfg(not(target_os = "macos"))]
        let result = Command::new("xdg-open")
            .arg(index.parent().unwrap_or(&index))
            .spawn();

        match result {
            Ok(_) => self.set_notice(
                "Course file selected in Finder. Use Open With and choose the browser profile where you studied.",
                true,
            ),
            Err(error) => self.set_notice(format!("Could not reveal the course file: {error}"), false),
        }
    }

    fn open_classic_course(&mut self) {
        let index = self.resources.join("index.html");
        if !index.exists() {
            self.set_notice(
                "The classic course page is not bundled with this app.",
                false,
            );
            return;
        }
        let profile = store::data_dir().join("ChromeProfile");
        if let Err(error) = fs::create_dir_all(&profile) {
            self.set_notice(
                format!("Could not open the classic Chrome profile ({error}); opening it in your default browser instead."),
                false,
            );
            self.open_default_browser_course();
            return;
        }

        #[cfg(target_os = "macos")]
        {
            let chrome = Path::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome");
            let url = url::Url::from_file_path(&index).ok();
            let launched = url.is_some_and(|url| {
                Command::new(chrome)
                    .arg(format!("--app={url}"))
                    .arg(format!("--user-data-dir={}", profile.display()))
                    .arg("--no-first-run")
                    .spawn()
                    .is_ok()
            });
            if launched {
                self.set_notice(
                    "Classic course opened. Export your backup there, then import the JSON here.",
                    true,
                );
            } else {
                self.open_default_browser_course();
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (index, profile);
            self.open_default_browser_course();
        }
    }

    fn open_video(&mut self, file: &str) {
        let path = self.resources.join("videos").join(file);
        #[cfg(target_os = "macos")]
        let result = Command::new("open").arg(&path).spawn();
        #[cfg(not(target_os = "macos"))]
        let result = Command::new("xdg-open").arg(&path).spawn();
        match result {
            Ok(_) => self.set_notice("Opening the lesson video…", true),
            Err(error) => self.set_notice(format!("Could not open the video: {error}"), false),
        }
    }

    fn show_quiz(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, index: usize, unit: &Unit) {
        if unit.quiz.is_empty() {
            empty_state(ui, "This lesson does not have a quiz yet.");
            return;
        }
        if self
            .quiz
            .as_ref()
            .is_none_or(|session| session.unit_index != index)
        {
            self.quiz = Some(QuizSession::new(index));
        }
        let mut pick_answer = None;
        let mut next_question = false;
        let mut retry = false;
        let mut retry_save = false;
        let mut open_next = false;
        let mut go_home = false;
        let mut audio_request: Option<(Option<String>, String)> = None;
        let Some(session) = self.quiz.as_ref() else {
            return;
        };
        if session.finished {
            let next = self.next_index();
            let can_continue = session.saved || self.progress.is_complete(&unit.id);
            let focused = quiz_result_card(
                ui,
                session,
                unit,
                index + 1,
                self.progress.scores.get(&unit.id).copied().unwrap_or(0),
                if can_continue {
                    next.map(|next_index| self.course.units[next_index].title.as_str())
                } else {
                    None
                },
                can_continue,
                session.save_error.as_deref(),
                session.focus_result,
                &mut retry_save,
                &mut retry,
                &mut open_next,
                &mut go_home,
            );
            if focused && let Some(session) = self.quiz.as_mut() {
                session.focus_result = false;
            }
        } else if let Some(question) = unit.quiz.get(session.question_index) {
            let (focused_first, focused_next) = quiz_question_card(
                ui,
                ctx,
                question,
                session,
                unit.quiz.len(),
                session.focus_first_answer,
                session.focus_next_question,
                &mut pick_answer,
                &mut next_question,
                &mut audio_request,
            );
            if let Some(session) = self.quiz.as_mut() {
                if focused_first {
                    session.focus_first_answer = false;
                }
                if focused_next {
                    session.focus_next_question = false;
                }
            }
        }

        let audio_triggered = audio_request.is_some();
        if let Some((audio, text)) = audio_request.take() {
            self.play_text_audio(audio.as_deref(), &text);
        }
        if let Some(choice) = pick_answer
            && let Some(session) = self.quiz.as_mut()
            && session.selected.is_none()
        {
            session.selected = Some(choice);
            session.focus_next_question = true;
            if unit.quiz[session.question_index].answer == choice {
                session.correct += 1;
            }
        }
        let mut result_to_save = None;
        if next_question {
            self.audio.stop();
            if let Some(session) = self.quiz.as_mut() {
                if session.question_index + 1 < unit.quiz.len() {
                    session.question_index += 1;
                    session.selected = None;
                    session.focus_first_answer = true;
                    session.focus_next_question = false;
                } else {
                    session.finished = true;
                    session.focus_result = true;
                    let score =
                        ((session.correct as f32 / unit.quiz.len() as f32) * 100.0).round() as u32;
                    result_to_save = Some((score, score >= PASS_SCORE));
                }
            }
        }
        if retry_save {
            result_to_save = self
                .quiz
                .as_ref()
                .filter(|session| session.finished)
                .map(|session| {
                    let score =
                        ((session.correct as f32 / unit.quiz.len() as f32) * 100.0).round() as u32;
                    (score, score >= PASS_SCORE)
                });
        }
        if let Some((score, passed)) = result_to_save {
            match self.persist_quiz_result(&unit.id, score, passed) {
                Ok(recognition) => {
                    if let Some(session) = self.quiz.as_mut() {
                        session.saved = true;
                        session.save_error = None;
                        if retry_save {
                            session.focus_result = true;
                        }
                    }
                    self.progress_load_error = false;
                    if !self.learning_load_error {
                        self.notice_sticky = false;
                    }
                    let result_message = if passed {
                        format!("Unit complete — {score}%! The next lesson is unlocked.")
                    } else {
                        format!("Saved your {score}% score. Try again when you’re ready.")
                    };
                    let celebration = recognition.is_some();
                    let message = recognition
                        .map(|recognition| format!("{recognition}  {result_message}"))
                        .unwrap_or(result_message);
                    self.set_notice(message, passed || celebration);
                }
                Err(error) => {
                    if let Some(session) = self.quiz.as_mut() {
                        session.saved = false;
                        session.save_error = Some(error.clone());
                        session.focus_result = true;
                    }
                    self.set_sticky_notice(error);
                }
            }
        }
        if retry {
            self.quiz = Some(QuizSession::new(index));
        }
        if open_next {
            if let Some(next) = self.next_index() {
                self.open_unit(next);
            } else {
                self.return_home();
            }
        }
        if go_home {
            self.return_home();
        }
        if pick_answer.is_some()
            || next_question
            || retry_save
            || retry
            || open_next
            || go_home
            || audio_triggered
        {
            ctx.request_repaint();
        }
    }
}

impl eframe::App for FrenchApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.refresh_today();
        let ctx = ui.ctx().clone();
        if (ctx.zoom_factor() - self.learning.text_scale).abs() > 0.001 {
            ctx.set_zoom_factor(self.learning.text_scale);
        }
        self.shortcuts(&ctx);
        match self.audio.is_playing() {
            Ok(active) => {
                self.audio_active = active;
                if !active {
                    self.playing_word = None;
                }
            }
            Err(error) => {
                self.audio_active = false;
                self.playing_word = None;
                self.set_notice(error, false);
            }
        }
        if self.audio_active {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
        let compact = ctx.content_rect().width() < 1000.0;
        if !compact {
            Panel::left("workspace_navigation")
                .exact_size(216.0)
                .resizable(false)
                .frame(
                    Frame::new()
                        .fill(WHITE)
                        .stroke(Stroke::new(1.0, BORDER))
                        .inner_margin(Margin::symmetric(18, 16)),
                )
                .show(ui, |ui| self.show_sidebar(ui));
        }
        Panel::top("workspace_header")
            .resizable(false)
            .frame(
                Frame::new()
                    .fill(WHITE)
                    .stroke(Stroke::new(1.0, BORDER))
                    .inner_margin(Margin::symmetric(24, 10)),
            )
            .show(ui, |ui| self.show_header(ui, compact));
        if self.notice.as_ref().is_some_and(|(_, created, _)| {
            !self.notice_sticky && created.elapsed() >= Duration::from_secs(5)
        }) {
            self.notice = None;
        }
        let mut import = false;
        let mut dismiss = false;
        Panel::bottom("workspace_status")
            .resizable(false)
            .frame(
                Frame::new()
                    .fill(WHITE)
                    .stroke(Stroke::new(1.0, BORDER))
                    .inner_margin(Margin::symmetric(24, 8)),
            )
            .show(ui, |ui| {
                if let Some((message, _, success)) = &self.notice {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(message).size(13.0).color(if *success {
                            GREEN
                        } else {
                            CORAL
                        }));
                        if self.progress_load_error || self.learning_load_error {
                            if ui.small_button("Import backup").clicked() {
                                import = true;
                            }
                        } else if ui.small_button("Dismiss").clicked() {
                            dismiss = true;
                        }
                    });
                    if !self.notice_sticky {
                        ctx.request_repaint_after(Duration::from_millis(500));
                    }
                } else {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(if self.audio_active {
                                "Audio playing · Escape to stop"
                            } else {
                                "Saved locally on this Mac"
                            })
                            .size(13.0)
                            .color(MUTED),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(RichText::new("⌘F  Find a lesson").size(13.0).color(MUTED));
                        });
                    });
                }
            });
        if dismiss {
            self.notice = None;
            self.notice_sticky = false;
        }
        if import {
            self.import_progress();
        }
        egui::CentralPanel::default()
            .frame(Frame::new().fill(BACKGROUND).inner_margin(Margin::same(0)))
            .show(ui, |ui| {
                let gutter = if compact { 20.0 } else { 32.0 };
                let maximum = match self.page {
                    Page::Lesson(_) => 920.0,
                    Page::Review => 760.0,
                    Page::Preferences => 820.0,
                    _ => CONTENT_MAX_WIDTH,
                };
                let width = (ui.available_width() - gutter * 2.0).min(maximum).max(1.0);
                let offset = (ui.available_width() - width) * 0.5;
                let rect = egui::Rect::from_min_size(
                    ui.max_rect().min + Vec2::new(offset, 0.0),
                    Vec2::new(width, ui.available_height()),
                );
                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(rect)
                        .layout(Layout::top_down(Align::Min)),
                    |ui| {
                        ui.set_width(width);
                        ui.set_clip_rect(rect.intersect(ui.clip_rect()));
                        ui.add_space(26.0);
                        match self.page {
                            Page::Home => self.show_home(ui, &ctx),
                            Page::Journey => self.show_journey(ui),
                            Page::Review => self.show_review(ui),
                            Page::Preferences => self.show_preferences(ui),
                            Page::Lesson(index) => self.show_lesson(ui, &ctx, index),
                        }
                    },
                );
            });
    }
}

fn resolve_resources() -> PathBuf {
    if let Ok(executable) = std::env::current_exe()
        && let Some(contents) = executable.parent().and_then(Path::parent)
    {
        let bundled = contents.join("Resources").join("Course");
        if bundled.join("course.json").exists() {
            return bundled;
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("course")
}

fn apply_theme(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    #[cfg(target_os = "macos")]
    if let Ok(data) = fs::read("/System/Library/Fonts/SFNS.ttf") {
        let name = "system-sans".to_owned();
        fonts.font_data.insert(
            name.clone(),
            std::sync::Arc::new(egui::FontData::from_owned(data)),
        );
        if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            family.insert(0, name);
        }
    }
    #[cfg(target_os = "macos")]
    if let Ok(data) = fs::read("/System/Library/Fonts/Supplemental/STIXGeneral.otf") {
        let name = "system-ipa-fallback".to_owned();
        fonts.font_data.insert(
            name.clone(),
            std::sync::Arc::new(egui::FontData::from_owned(data)),
        );
        if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            family.push(name.clone());
        }
        if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
            family.push(name);
        }
    }
    ctx.set_fonts(fonts);

    let mut visuals = egui::Visuals::light();
    visuals.panel_fill = BACKGROUND;
    visuals.window_fill = WHITE;
    visuals.faint_bg_color = PALE_SURFACE;
    visuals.extreme_bg_color = WHITE;
    visuals.hyperlink_color = BLUE;
    visuals.selection.bg_fill = PALE_BLUE;
    visuals.widgets.noninteractive.bg_fill = WHITE;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, INK);
    visuals.widgets.inactive.bg_fill = WHITE;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, NAVY);
    visuals.widgets.hovered.bg_fill = PALE_BLUE;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, NAVY);
    visuals.widgets.active.bg_fill = TEAL;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, WHITE);
    visuals.widgets.open.bg_fill = PALE_BLUE;
    visuals.widgets.inactive.corner_radius = CornerRadius::same(4);
    visuals.widgets.hovered.corner_radius = CornerRadius::same(4);
    visuals.widgets.active.corner_radius = CornerRadius::same(4);
    visuals.widgets.open.corner_radius = CornerRadius::same(4);
    ctx.set_visuals(visuals);
    ctx.style_mut_of(egui::Theme::Light, |style| {
        style
            .text_styles
            .insert(egui::TextStyle::Body, FontId::proportional(16.0));
        style
            .text_styles
            .insert(egui::TextStyle::Button, FontId::proportional(14.0));
        style
            .text_styles
            .insert(egui::TextStyle::Small, FontId::proportional(12.0));
        style
            .text_styles
            .insert(egui::TextStyle::Heading, FontId::proportional(24.0));
        style.spacing.interact_size = Vec2::new(40.0, 40.0);
        style.spacing.item_spacing = Vec2::new(10.0, 10.0);
        style.spacing.button_padding = Vec2::new(13.0, 8.0);
        style.visuals.window_corner_radius = CornerRadius::same(6);
    });
}

fn progress_track(
    ui: &mut egui::Ui,
    progress: f32,
    height: f32,
    width: f32,
    fill: Color32,
) -> egui::Response {
    let progress = progress.clamp(0.0, 1.0);
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(width.max(1.0), height), Sense::hover());
    let radius = CornerRadius::same((height / 2.0).round() as u8);
    ui.painter()
        .rect_filled(rect, radius, Color32::from_rgb(232, 236, 241));
    if progress > 0.0 {
        let fill_rect = egui::Rect::from_min_size(
            rect.min,
            Vec2::new((rect.width() * progress).max(height), rect.height()),
        );
        ui.painter().rect_filled(fill_rect, radius, fill);
    }
    response.widget_info(|| {
        let mut info = egui::WidgetInfo::new(egui::WidgetType::ProgressIndicator);
        info.value = Some((progress * 100.0).round() as f64);
        info
    });
    response
}

fn milestone_cards(course: &Course, progress: &Progress) -> Vec<JourneyCardInfo> {
    let total = course.units.len() as u32;
    let completed = progress.completed_count(course) as u32;
    let mut cards = Vec::new();
    let mut push =
        |title: String, detail: String, current: u32, target: u32, icon: &'static str| {
            if target > 0 {
                cards.push(JourneyCardInfo {
                    title,
                    detail,
                    icon,
                    current: current.min(target),
                    target,
                    unlocked: current >= target,
                });
            }
        };

    push(
        "First step".to_owned(),
        "Pass your first lesson quiz".to_owned(),
        completed,
        1.min(total),
        "↗",
    );
    if total >= 5 {
        push(
            "Five lessons".to_owned(),
            "Build a strong foundation".to_owned(),
            completed,
            5,
            "✦",
        );
    }

    let level_one_total = course.units.iter().filter(|unit| unit.level == 1).count() as u32;
    if level_one_total > 0 {
        let level_one_done = course
            .units
            .iter()
            .filter(|unit| unit.level == 1 && progress.is_complete(&unit.id))
            .count() as u32;
        push(
            "A1 foundations".to_owned(),
            "Complete every Niveau 1 lesson".to_owned(),
            level_one_done,
            level_one_total,
            "◆",
        );
    }

    let first_two_levels: Vec<&Unit> = course.units.iter().filter(|unit| unit.level <= 2).collect();
    let first_two_total = first_two_levels.len() as u32;
    if first_two_total > level_one_total {
        let first_two_done = first_two_levels
            .iter()
            .filter(|unit| progress.is_complete(&unit.id))
            .count() as u32;
        push(
            "A2 autonomy".to_owned(),
            "Finish the beginner and independent levels".to_owned(),
            first_two_done,
            first_two_total,
            "◆",
        );
    }

    push(
        "Full course complete".to_owned(),
        "Complete the TEF learning path".to_owned(),
        completed,
        total,
        "★",
    );
    cards
}

fn achievement_cards(course: &Course, progress: &Progress) -> Vec<JourneyCardInfo> {
    let attempts = progress.quiz_history.len() as u32;
    let best_score = progress.scores.values().copied().max().unwrap_or(0);
    let longest_streak = progress.longest_streak();
    let study_days = progress.study_days.len() as u32;
    let completed = progress.completed_count(course) as u32;
    let total = course.units.len() as u32;

    vec![
        achievement_info(
            "Quiz starter",
            "Finish your first quiz".to_owned(),
            attempts,
            1,
            "✓",
        ),
        achievement_info(
            "Quiz regular",
            "Finish 10 quiz attempts".to_owned(),
            attempts,
            10,
            "✦",
        ),
        achievement_info(
            "Perfect score",
            "Earn 100% on any quiz".to_owned(),
            best_score,
            100,
            "★",
        ),
        achievement_info(
            "Three-day rhythm",
            "Study on three consecutive days".to_owned(),
            longest_streak,
            3,
            "↗",
        ),
        achievement_info(
            "A week of French",
            "Study on seven different days".to_owned(),
            study_days,
            7,
            "◆",
        ),
        achievement_info(
            "Course finisher",
            "Complete every lesson in the course".to_owned(),
            completed,
            total,
            "★",
        ),
    ]
}

fn achievement_info(
    title: &str,
    detail: String,
    current: u32,
    target: u32,
    icon: &'static str,
) -> JourneyCardInfo {
    JourneyCardInfo {
        title: title.to_owned(),
        detail,
        icon,
        current: current.min(target),
        target,
        unlocked: current >= target,
    }
}

fn learning_achievements(learning: &LearningState) -> Vec<JourneyCardInfo> {
    let reviews = learning
        .reviews_by_day
        .values()
        .fold(0u32, |sum, count| sum.saturating_add(*count));
    vec![
        achievement_info(
            "First recall",
            "Review your first saved word".to_owned(),
            reviews,
            1,
            "↻",
        ),
        achievement_info(
            "Word collector",
            "Save 10 vocabulary words".to_owned(),
            learning.cards.len().min(u32::MAX as usize) as u32,
            10,
            "+",
        ),
        achievement_info(
            "Recall regular",
            "Complete 50 word reviews".to_owned(),
            reviews,
            50,
            "↗",
        ),
    ]
}

#[allow(clippy::too_many_arguments)] // Explicit view inputs keep quiz state changes in the caller.
fn quiz_question_card(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    question: &QuizQuestion,
    session: &QuizSession,
    total: usize,
    focus_first_answer: bool,
    focus_next_question: bool,
    pick_answer: &mut Option<usize>,
    next_question: &mut bool,
    audio_request: &mut Option<(Option<String>, String)>,
) -> (bool, bool) {
    let mut focused_first = false;
    let mut focused_next = false;
    Frame::new()
        .fill(WHITE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::same(24))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "QUESTION {:02} / {:02}",
                        session.question_index + 1,
                        total
                    ))
                    .size(13.0)
                    .strong()
                    .color(BLUE),
                );
                if question.kind == "listening" {
                    ui.label(
                        RichText::new("LISTENING PRACTICE")
                            .size(13.0)
                            .strong()
                            .color(TEAL),
                    );
                }
            });
            let animated = ctx.animate_value_with_time(
                egui::Id::new(("quiz-progress", session.unit_index)),
                (session.question_index + 1) as f32 / total.max(1) as f32,
                0.28,
            );
            progress_track(ui, animated, 6.0, ui.available_width(), TEAL);
            ui.add_space(16.0);
            ui.label(RichText::new(&question.q).size(22.0).strong().color(NAVY));
            if question.kind == "listening" {
                ui.add_space(12.0);
                if ui
                    .add(action_button("▶  Play the French audio", true))
                    .clicked()
                {
                    *audio_request = Some((
                        question.audio.clone(),
                        question.transcript.clone().unwrap_or_default(),
                    ));
                }
                ui.label(
                    RichText::new("Listen as many times as you need.")
                        .size(13.0)
                        .color(MUTED),
                );
                ui.add_space(10.0);
            } else {
                ui.add_space(14.0);
            }
            for (choice_index, choice) in question.choices.iter().enumerate() {
                let selected = session.selected == Some(choice_index);
                let correct_choice = session.selected.is_some() && choice_index == question.answer;
                let (fill, text_color, stroke) = if correct_choice {
                    (PALE_GREEN, GREEN, Color32::from_rgb(186, 222, 198))
                } else if selected && choice_index != question.answer {
                    (
                        Color32::from_rgb(255, 240, 235),
                        CORAL,
                        Color32::from_rgb(240, 207, 197),
                    )
                } else {
                    (Color32::from_rgb(248, 249, 251), INK, BORDER)
                };
                let label = format!(
                    "{}.  {}",
                    (b'A' + choice_index.min(25) as u8) as char,
                    choice
                );
                let button = Button::new(RichText::new(label).size(16.0).color(text_color))
                    .fill(fill)
                    .stroke(Stroke::new(1.0, stroke))
                    .corner_radius(CornerRadius::same(5))
                    .min_size(Vec2::new(ui.available_width(), 48.0));
                let response = ui.add_enabled(session.selected.is_none(), button);
                if focus_first_answer && choice_index == 0 && response.enabled() {
                    response.request_focus();
                    focused_first = true;
                }
                if response.clicked() {
                    *pick_answer = Some(choice_index);
                }
                ui.add_space(6.0);
            }
            if let Some(selected) = session.selected {
                ui.add_space(10.0);
                let correct = selected == question.answer;
                Frame::new()
                    .fill(if correct {
                        PALE_GREEN
                    } else {
                        Color32::from_rgb(255, 244, 239)
                    })
                    .stroke(Stroke::new(
                        1.0,
                        if correct {
                            Color32::from_rgb(195, 226, 205)
                        } else {
                            Color32::from_rgb(242, 211, 199)
                        },
                    ))
                    .corner_radius(CornerRadius::same(5))
                    .inner_margin(Margin::same(15))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(if correct {
                                "Bonne réponse !"
                            } else {
                                "Not quite — keep going."
                            })
                            .size(14.0)
                            .strong()
                            .color(if correct {
                                GREEN
                            } else {
                                CORAL
                            }),
                        );
                        ui.add_space(3.0);
                        ui.label(RichText::new(&question.explain).size(13.0).color(INK));
                        if question.kind == "listening"
                            && let Some(transcript) = &question.transcript
                        {
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new(format!("You heard: “{transcript}”"))
                                    .size(13.0)
                                    .strong()
                                    .color(NAVY),
                            );
                        }
                    });
                ui.add_space(14.0);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let label = if session.question_index + 1 == total {
                        "See results  →"
                    } else {
                        "Next question  →"
                    };
                    let response = ui.add(action_button(label, true));
                    if focus_next_question {
                        response.request_focus();
                        let feedback = if correct {
                            "Correct answer. Bonne réponse!"
                        } else {
                            "Incorrect answer. Not quite."
                        };
                        let accessible_label = format!(
                            "{feedback} {}. {}",
                            question.explain,
                            label.trim_end_matches("  →")
                        );
                        response.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Button,
                                true,
                                accessible_label.clone(),
                            )
                        });
                        focused_next = true;
                    }
                    if response.clicked() {
                        *next_question = true;
                    }
                });
            }
        });
    (focused_first, focused_next)
}

#[allow(clippy::too_many_arguments)] // Result rendering shares the explicit action boundary of question rendering.
fn quiz_result_card(
    ui: &mut egui::Ui,
    session: &QuizSession,
    unit: &Unit,
    unit_number: usize,
    best_score: u32,
    next_title: Option<&str>,
    can_continue: bool,
    save_error: Option<&str>,
    focus_result: bool,
    retry_save: &mut bool,
    retry: &mut bool,
    open_next: &mut bool,
    go_home: &mut bool,
) -> bool {
    let total = unit.quiz.len().max(1);
    let score = ((session.correct as f32 / total as f32) * 100.0).round() as u32;
    let passed = score >= PASS_SCORE;
    let mut focused = false;
    Frame::new()
        .fill(if passed && can_continue { Color32::from_rgb(31, 62, 68) } else { Color32::from_rgb(54, 49, 66) })
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::same(28))
        .show(ui, |ui| {
            ui.label(RichText::new(if passed && can_continue { "UNIT COMPLETE" } else { "PRACTICE COMPLETE" }).size(13.0).strong().color(if passed && can_continue { Color32::from_rgb(164, 223, 200) } else { Color32::from_rgb(213, 203, 231) }));
            ui.add_space(8.0);
            ui.label(RichText::new(if passed && can_continue { "Bravo !" } else if passed { "You passed — save needed." } else { "Good effort — keep at it." }).size(29.0).strong().color(WHITE));
            ui.label(RichText::new(format!("You scored {} / {} · {score}%", session.correct, unit.quiz.len())).size(15.0).color(WHITE));
            ui.add_space(7.0);
            progress_track(
                ui,
                score as f32 / 100.0,
                9.0,
                ui.available_width(),
                if passed {
                    TEAL
                } else {
                    Color32::from_rgb(218, 170, 97)
                },
            );
            ui.add_space(8.0);
            let detail = if passed && can_continue {
                next_title
                    .map(|title| format!("Unit {unit_number} is complete. Next up: {title}.", unit_number = unit_number))
                    .unwrap_or_else(|| "You completed every unit in the course. Félicitations !".to_owned())
            } else if passed {
                format!("You passed, but this result could not be saved. {}", save_error.unwrap_or("Try saving again or import a valid progress backup."))
            } else if let Some(error) = save_error {
                format!("Your score could not be saved. {error}")
            } else {
                format!("You need {PASS_SCORE}% to unlock the next lesson. Your best score is {best_score}%.")
            };
            ui.label(RichText::new(detail).size(13.0).color(Color32::from_rgb(232, 235, 243)));
            ui.add_space(16.0);
            ui.horizontal_wrapped(|ui| {
                if save_error.is_some() {
                    let response = ui.add(action_button("Try saving again", true));
                    focus_result_button(&response, focus_result, &mut focused, "Try saving this quiz result again");
                    if response.clicked() {
                        *retry_save = true;
                    }
                } else if passed && can_continue && next_title.is_some() {
                    let response = ui.add(action_button("Continue to next lesson  →", true));
                    focus_result_button(&response, focus_result, &mut focused, "Continue to the next lesson");
                    if response.clicked() {
                        *open_next = true;
                    }
                } else if passed && can_continue {
                    let response = ui.add(action_button("Back to course  →", true));
                    focus_result_button(&response, focus_result, &mut focused, "Back to the course overview");
                    if response.clicked() {
                        *go_home = true;
                    }
                } else {
                    let response = ui.add(action_button("Retry quiz  ↻", true));
                    focus_result_button(&response, focus_result, &mut focused, "Retry this quiz");
                    if response.clicked() {
                        *retry = true;
                    }
                }
                if save_error.is_some() && !passed {
                    if ui.add(action_button("Retry quiz", false)).clicked() {
                        *retry = true;
                    }
                } else if (!passed || !can_continue) && ui.add(action_button("Back to lessons", false)).clicked() {
                    *go_home = true;
                }
            });
        });
    focused
}

fn focus_result_button(
    response: &egui::Response,
    should_focus: bool,
    focused: &mut bool,
    accessible_label: &str,
) {
    if should_focus {
        response.request_focus();
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, true, accessible_label)
        });
        *focused = true;
    }
}

struct ActionButton<'a> {
    label: &'a str,
    accessibility_label: &'a str,
    primary: bool,
    min_width: f32,
}

impl egui::Widget for ActionButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let text_color = if self.primary { WHITE } else { NAVY };
        let font = FontId::proportional(14.0);
        let galley = ui
            .painter()
            .layout_no_wrap(self.label.to_owned(), font, text_color);
        let width = if ui.layout().cross_justify() {
            ui.available_width()
        } else {
            (galley.size().x + 30.0)
                .max(self.min_width)
                .min(ui.available_width())
        };
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width.max(1.0), 42.0), Sense::click());
        let fill = if self.primary {
            if response.is_pointer_button_down_on() {
                TEAL_DOWN
            } else if response.hovered() {
                TEAL_HOVER
            } else {
                TEAL
            }
        } else if response.hovered() {
            PALE_BLUE
        } else {
            PALE_SURFACE
        };
        ui.painter().rect_filled(rect, CornerRadius::same(4), fill);
        if !self.primary {
            ui.painter().rect_stroke(
                rect,
                CornerRadius::same(4),
                Stroke::new(1.0, BORDER),
                egui::StrokeKind::Inside,
            );
        }
        if response.has_focus() {
            ui.painter().rect_stroke(
                rect.shrink(1.0),
                CornerRadius::same(4),
                Stroke::new(1.5, FOCUS_RING),
                egui::StrokeKind::Inside,
            );
        }
        ui.painter()
            .galley(rect.center() - galley.size() * 0.5, galley, text_color);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Button, true, self.accessibility_label)
        });
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    }
}

fn action_button(label: &str, primary: bool) -> ActionButton<'_> {
    ActionButton {
        label,
        accessibility_label: label,
        primary,
        min_width: 112.0,
    }
}

fn icon_action_button<'a>(
    label: &'a str,
    accessibility_label: &'a str,
    primary: bool,
) -> ActionButton<'a> {
    ActionButton {
        label,
        accessibility_label,
        primary,
        min_width: 38.0,
    }
}

fn sidebar_nav_item(ui: &mut egui::Ui, icon: &str, label: &str, active: bool) -> egui::Response {
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 42.0), Sense::click());
    let fill = if active {
        SIDEBAR_ACTIVE
    } else if response.hovered() {
        SIDEBAR_HOVER
    } else {
        Color32::TRANSPARENT
    };
    if fill != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, CornerRadius::same(4), fill);
    }
    if active {
        ui.painter().rect_filled(
            egui::Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height())),
            CornerRadius::same(2),
            SIDEBAR_ACCENT,
        );
    }
    if response.has_focus() {
        ui.painter().rect_stroke(
            rect.shrink(1.0),
            CornerRadius::same(4),
            Stroke::new(1.5, FOCUS_RING),
            egui::StrokeKind::Inside,
        );
    }
    let icon_color = if active {
        SIDEBAR_ACCENT
    } else {
        SIDEBAR_MUTED
    };
    let text_color = if active { TEAL } else { SIDEBAR_TEXT };
    ui.painter().text(
        rect.min + Vec2::new(17.0, rect.height() * 0.5),
        Align2::LEFT_CENTER,
        icon,
        FontId::proportional(16.0),
        icon_color,
    );
    ui.painter().text(
        rect.min + Vec2::new(45.0, rect.height() * 0.5),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(14.0),
        text_color,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label.to_owned())
    });
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

fn empty_state(ui: &mut egui::Ui, text: &str) {
    Frame::new()
        .fill(WHITE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::same(20))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(13.0).color(MUTED));
        });
}

const fn color(hex: u32) -> Color32 {
    Color32::from_rgb(
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    )
}

const BACKGROUND: Color32 = color(0xf5f6f8);
const PALE_SURFACE: Color32 = color(0xf7f8fb);
const WHITE: Color32 = color(0xffffff);
const NAVY: Color32 = color(0x1a2740);
const INK: Color32 = color(0x2c384b);
const MUTED: Color32 = color(0x5e6b7c);
const BLUE: Color32 = color(0x4b68c8);
const TEAL: Color32 = color(0x138f83);
const TEAL_HOVER: Color32 = color(0x159b8e);
const TEAL_DOWN: Color32 = color(0x0d736b);
const CORAL: Color32 = color(0xc85f54);
const GREEN: Color32 = color(0x28795c);
const BORDER: Color32 = color(0xe3e8f0);
const PALE_BLUE: Color32 = color(0xf0f3f8);
const PALE_GREEN: Color32 = color(0xe8f5ef);
const FOCUS_RING: Color32 = color(0x7c9cff);
const SIDEBAR_ACTIVE: Color32 = color(0xe8f5ef);
const SIDEBAR_HOVER: Color32 = color(0xf0f3f8);
const SIDEBAR_TEXT: Color32 = color(0x2c384b);
const SIDEBAR_MUTED: Color32 = color(0x738095);
const SIDEBAR_ACCENT: Color32 = TEAL;
const CONTENT_MAX_WIDTH: f32 = 1080.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removing_current_and_pending_words_skips_them_without_erasing_completed_reviews() {
        let mut session = ReviewSession {
            keys: [
                "already-rated",
                "deleted-current",
                "deleted-later",
                "remaining",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            position: 1,
            revealed: true,
            remembered: 1,
        };
        let available = HashSet::from(["remaining".to_owned()]);
        session.reconcile(&available);
        assert_eq!(session.keys, ["already-rated", "remaining"]);
        assert_eq!(session.position, 1);
        assert_eq!(session.remembered, 1);
        assert!(
            !session.revealed,
            "The next word must require its own reveal"
        );
        session.revealed = true;
        session.reconcile(&HashSet::new());
        assert_eq!(session.keys, ["already-rated"]);
        assert_eq!(session.position, session.keys.len());
        assert_eq!(session.remembered, 1);
        assert!(!session.revealed);
    }

    #[test]
    fn removing_only_later_words_keeps_current_reveal_and_completion_counts() {
        let mut session = ReviewSession {
            keys: [
                "remembered",
                "forgotten",
                "current",
                "deleted-later",
                "remaining",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            position: 2,
            revealed: true,
            remembered: 1,
        };
        let available = HashSet::from(["current".to_owned(), "remaining".to_owned()]);
        session.reconcile(&available);
        assert_eq!(
            session.keys,
            ["remembered", "forgotten", "current", "remaining"]
        );
        assert_eq!(session.position, 2);
        assert_eq!(session.remembered, 1);
        assert!(
            session.revealed,
            "Removing a later card must not hide the current meaning"
        );
        session.reconcile(&available);
        assert_eq!(session.keys.len(), 4);
        assert_eq!(session.position, 2);
        assert_eq!(session.remembered, 1);
        assert!(session.revealed);
    }
}
