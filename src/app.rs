use crate::{audio::AudioEngine, model::*, store};
use eframe::egui::{
    self, Align, Align2, Button, Color32, CornerRadius, FontId, Frame, Image, Label, Layout,
    Margin, Panel, RichText, ScrollArea, Sense, Stroke, Vec2,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

const PASS_SCORE: u32 = 70;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Page {
    Home,
    Lesson(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FocusTarget {
    HomeEntry,
    LessonEntry,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    page: Page,
    pending_page_focus: Option<FocusTarget>,
    lesson_tab: LessonTab,
    quiz: Option<QuizSession>,
    slow_audio: bool,
    pending_level_scroll: Option<u32>,
    textures: HashMap<String, egui::TextureHandle>,
    audio: AudioEngine,
    notice: Option<(String, Instant, bool)>,
    notice_sticky: bool,
    progress_load_error: bool,
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
        Ok(Self {
            course,
            progress,
            resources,
            page: Page::Home,
            pending_page_focus: None,
            lesson_tab: LessonTab::Words,
            quiz: None,
            slow_audio: false,
            pending_level_scroll: None,
            textures: HashMap::new(),
            audio: AudioEngine::new(),
            notice: load_notice.map(|text| (text, Instant::now(), false)),
            notice_sticky: progress_load_error,
            progress_load_error,
        })
    }

    fn done_count(&self) -> usize {
        self.progress.completed_count(&self.course)
    }

    fn next_index(&self) -> Option<usize> {
        self.progress
            .next_unit(&self.course)
            .and_then(|next| self.course.units.iter().position(|unit| unit.id == next.id))
    }

    fn open_unit(&mut self, index: usize) {
        let Some(unit) = self.course.units.get(index) else {
            return;
        };
        if !self.progress.can_open(&self.course, unit) {
            self.set_notice("Pass the previous quiz to unlock this lesson.", false);
            return;
        }
        self.audio.stop();
        self.page = Page::Lesson(index);
        self.pending_page_focus = Some(FocusTarget::LessonEntry);
        self.lesson_tab = LessonTab::Words;
        self.quiz = None;
    }

    fn return_home(&mut self) {
        self.audio.stop();
        self.page = Page::Home;
        self.pending_page_focus = Some(FocusTarget::HomeEntry);
        self.quiz = None;
    }

    fn set_notice(&mut self, message: impl Into<String>, success: bool) {
        self.notice = Some((message.into(), Instant::now(), success));
        self.notice_sticky = false;
    }

    fn set_sticky_notice(&mut self, message: impl Into<String>) {
        self.notice = Some((message.into(), Instant::now(), false));
        self.notice_sticky = true;
    }

    fn persist_quiz_result(
        &mut self,
        unit_id: &str,
        score: u32,
        passed: bool,
    ) -> Result<(), String> {
        let mut updated = self.progress.clone();
        updated.record_quiz(unit_id, score, passed);
        store::save_progress(&updated)?;
        self.progress = updated;
        Ok(())
    }

    fn play_audio(&mut self, line: &StudyLine) {
        match self.audio.play(
            &self.resources,
            line.audio.as_deref(),
            &line.fr,
            self.slow_audio,
        ) {
            Ok(()) => self.set_notice(format!("Playing: {}", line.fr), true),
            Err(error) => self.set_notice(error, false),
        }
    }

    fn play_text_audio(&mut self, audio: Option<&str>, text: &str) {
        match self
            .audio
            .play(&self.resources, audio, text, self.slow_audio)
        {
            Ok(()) => self.set_notice(format!("Playing: {text}"), true),
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
        let image = image::open(path).ok()?.to_rgba8();
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

    fn show_header(&mut self, ui: &mut egui::Ui) {
        let total = self.course.units.len();
        let done = self.done_count();
        let mut go_home = false;
        let mut toggle_slow = false;
        let mut import = false;
        ui.horizontal(|ui| {
            if matches!(self.page, Page::Lesson(_))
                && ui.add(action_button("‹  All lessons", false)).clicked()
            {
                go_home = true;
            }
            ui.add_space(2.0);
            let (logo_rect, _) = ui.allocate_exact_size(Vec2::splat(38.0), Sense::hover());
            ui.painter()
                .rect_filled(logo_rect, CornerRadius::same(12), NAVY);
            ui.painter().text(
                logo_rect.center(),
                Align2::CENTER_CENTER,
                "F",
                FontId::proportional(20.0),
                WHITE,
            );
            ui.vertical(|ui| {
                let short_title = self
                    .course
                    .meta
                    .title
                    .split_once('—')
                    .map(|(title, _)| title.trim())
                    .unwrap_or(&self.course.meta.title);
                ui.label(RichText::new(short_title).size(16.0).strong().color(NAVY));
                ui.label(
                    RichText::new(&self.course.meta.subtitle)
                        .size(9.5)
                        .strong()
                        .color(MUTED),
                );
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(action_button(
                        if self.slow_audio {
                            "Slow audio · On"
                        } else {
                            "Slow audio"
                        },
                        false,
                    ))
                    .clicked()
                {
                    toggle_slow = true;
                }
                if matches!(self.page, Page::Home)
                    && ui.add(action_button("Import progress…", false)).clicked()
                {
                    import = true;
                }
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("COURSE PROGRESS")
                                .size(9.0)
                                .strong()
                                .color(MUTED),
                        );
                        ui.label(
                            RichText::new(format!("{done} / {total}"))
                                .size(11.0)
                                .strong()
                                .color(NAVY),
                        );
                    });
                    progress_track(
                        ui,
                        if total == 0 {
                            0.0
                        } else {
                            done as f32 / total as f32
                        },
                        7.0,
                        142.0,
                        TEAL,
                    );
                });
            });
        });
        if go_home {
            self.return_home();
        }
        if toggle_slow {
            self.slow_audio = !self.slow_audio;
        }
        if import {
            self.import_progress();
        }
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        let done = self.done_count();
        let total = self.course.units.len();
        let next = self.next_index();
        let mut go_home = false;
        let mut scroll_level = None;
        let mut open_next = None;
        ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(8.0);
                    ui.label(RichText::new("LEARNING PATH").size(10.0).strong().color(MUTED));
                    ui.add_space(12.0);
                    let response = ui.add_sized(
                        [ui.available_width(), 42.0],
                        nav_button("Overview", matches!(self.page, Page::Home)),
                    );
                    if matches!(self.page, Page::Home)
                        && self.pending_page_focus == Some(FocusTarget::HomeEntry)
                    {
                        response.request_focus();
                        self.pending_page_focus = None;
                    }
                    if response.clicked() {
                        go_home = true;
                    }
                    ui.add_space(14.0);
                    for level in &self.course.meta.levels {
                        let units: Vec<(usize, &Unit)> = self
                            .course
                            .units
                            .iter()
                            .enumerate()
                            .filter(|(_, unit)| unit.level == level.n)
                            .collect();
                        let complete = units
                            .iter()
                            .filter(|(_, unit)| self.progress.is_complete(&unit.id))
                            .count();
                        let active = matches!(self.page, Page::Lesson(index) if self.course.units.get(index).is_some_and(|unit| unit.level == level.n));
                        let text = format!("Niveau {}    {} / {}", level.n, complete, units.len());
                        if ui
                            .add_sized(
                                [ui.available_width(), 38.0],
                                nav_button(&text, active),
                            )
                            .clicked()
                        {
                            scroll_level = Some(level.n);
                        }
                        ui.label(RichText::new(format!("{} · {}", level.cefr, level.title)).size(11.0).color(MUTED));
                        ui.add_space(8.0);
                    }

                    ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                        ui.add_space(2.0);
                        if let Some(index) = next {
                            let unit = &self.course.units[index];
                            Frame::new()
                                .fill(PALE_BLUE)
                                .stroke(Stroke::new(1.0, Color32::from_rgb(212, 225, 244)))
                                .corner_radius(CornerRadius::same(16))
                                .inner_margin(Margin::same(15))
                                .show(ui, |ui| {
                                    ui.label(RichText::new(if done == 0 { "START HERE" } else { "PICK UP WHERE YOU LEFT OFF" }).size(9.0).strong().color(BLUE));
                                    ui.add_space(6.0);
                                    ui.label(RichText::new(format!("Unit {}", index + 1)).size(12.0).strong().color(NAVY));
                                    ui.label(RichText::new(&unit.title).size(13.0).strong().color(INK));
                                    ui.add_space(8.0);
                                    if ui.add_sized([ui.available_width(), 42.0], action_button("Continue lesson  →", true)).clicked() {
                                        open_next = Some(index);
                                    }
                                });
                        } else {
                            Frame::new()
                                .fill(PALE_GREEN)
                                .corner_radius(CornerRadius::same(16))
                                .inner_margin(Margin::same(15))
                                .show(ui, |ui| {
                                    ui.label(RichText::new("COURSE COMPLETE").size(10.0).strong().color(GREEN));
                                    ui.label(RichText::new(format!("Bravo! All {total} units finished.")).size(14.0).strong().color(NAVY));
                                });
                        }
                    });
        });

        if go_home {
            self.return_home();
        }
        if let Some(level) = scroll_level {
            self.return_home();
            self.pending_level_scroll = Some(level);
        }
        if let Some(index) = open_next {
            self.open_unit(index);
        }
    }

    fn show_home(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let level_assets: Vec<(u32, String, PathBuf)> = self
            .course
            .meta
            .levels
            .iter()
            .map(|level| {
                (
                    level.n,
                    format!("level-{}", level.n),
                    self.resources
                        .join("images")
                        .join(format!("level{}.jpg", level.n)),
                )
            })
            .collect();
        for (_, key, path) in &level_assets {
            let _ = self.load_texture(ctx, key, path);
        }
        let level_images: HashMap<u32, Option<egui::TextureHandle>> = level_assets
            .iter()
            .map(|(number, key, _)| (*number, self.textures.get(key).cloned()))
            .collect();
        let banner = level_images.get(&1).and_then(Option::as_ref);
        let course = &self.course;
        let progress = &self.progress;
        let next_index = self.next_index();
        let done = progress.completed_count(course);
        let focus_home_entry = self.pending_page_focus == Some(FocusTarget::HomeEntry);
        let mut focused_home_entry = false;
        let mut open_unit = None;
        let mut pending_scroll = self.pending_level_scroll.take();

        ScrollArea::vertical()
            .id_salt("home-scroll")
            .show(ui, |ui| {
                ui.add_space(24.0);
                if let Some(index) = next_index {
                    let next = &course.units[index];
                    let (clicked, focused) =
                        home_hero(ui, next, index + 1, done, banner, focus_home_entry);
                    focused_home_entry = focused_home_entry || focused;
                    if clicked {
                        open_unit = Some(index);
                    }
                } else {
                    focused_home_entry = completion_hero(ui, course.units.len(), focus_home_entry);
                }
                ui.add_space(16.0);
                ui.columns(3, |columns| {
                    metric_card(
                        &mut columns[0],
                        "YOUR PROGRESS",
                        format!("{done} of {}", course.units.len()),
                        "units completed".to_owned(),
                        done as f32 / course.units.len().max(1) as f32,
                    );
                    let level = next_index.and_then(|index| {
                        course
                            .meta
                            .levels
                            .iter()
                            .find(|level| level.n == course.units[index].level)
                    });
                    let level_text = level
                        .map(|level| format!("{} · {}", level.cefr, level.title))
                        .unwrap_or_else(|| "All levels".to_owned());
                    let level_done = level
                        .map(|level| {
                            course
                                .units
                                .iter()
                                .filter(|unit| {
                                    unit.level == level.n && progress.is_complete(&unit.id)
                                })
                                .count()
                        })
                        .unwrap_or(0);
                    let level_total = level
                        .map(|level| {
                            course
                                .units
                                .iter()
                                .filter(|unit| unit.level == level.n)
                                .count()
                        })
                        .unwrap_or(0);
                    metric_card(
                        &mut columns[1],
                        "CURRENT LEVEL",
                        level_text,
                        format!("{level_done} of {level_total} lessons"),
                        if level_total == 0 {
                            1.0
                        } else {
                            level_done as f32 / level_total as f32
                        },
                    );
                    metric_card(
                        &mut columns[2],
                        "UNLOCK THE NEXT",
                        "70% quiz score".to_owned(),
                        "Take it at your own pace".to_owned(),
                        0.70,
                    );
                });

                ui.add_space(30.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Your learning path")
                            .size(23.0)
                            .strong()
                            .color(NAVY),
                    );
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(
                            RichText::new("25 small steps · one big goal")
                                .size(12.0)
                                .color(MUTED),
                        );
                    });
                });
                ui.add_space(12.0);

                for level in &course.meta.levels {
                    let start = ui.cursor().min;
                    let units: Vec<(usize, &Unit)> = course
                        .units
                        .iter()
                        .enumerate()
                        .filter(|(_, unit)| unit.level == level.n)
                        .collect();
                    if units.is_empty() {
                        continue;
                    }
                    let complete = units
                        .iter()
                        .filter(|(_, unit)| progress.is_complete(&unit.id))
                        .count();
                    let level_image = level_images.get(&level.n).and_then(Option::as_ref);
                    level_header(ui, level, complete, units.len(), level_image);
                    ui.add_space(10.0);
                    ui.columns(2, |columns| {
                        for (position, (index, unit)) in units.iter().enumerate() {
                            let next = progress.next_unit(course);
                            let can_open = progress.can_open(course, unit);
                            let unlock_title = next.map(|unit| {
                                format!(
                                    "Pass Unit {} to continue",
                                    course
                                        .units
                                        .iter()
                                        .position(|candidate| candidate.id == unit.id)
                                        .unwrap_or(0)
                                        + 1
                                )
                            });
                            let clicked = show_unit_card(
                                &mut columns[position % 2],
                                unit,
                                *index + 1,
                                progress,
                                can_open,
                                unlock_title.as_deref(),
                            );
                            if clicked {
                                open_unit = Some(*index);
                            }
                        }
                    });
                    ui.add_space(20.0);
                    if pending_scroll == Some(level.n) {
                        ui.scroll_to_rect(
                            egui::Rect::from_min_size(start, Vec2::splat(1.0)),
                            Some(Align::Min),
                        );
                        pending_scroll = None;
                    }
                }

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Cinéma français")
                            .size(22.0)
                            .strong()
                            .color(NAVY),
                    );
                    ui.label(
                        RichText::new("Watch, listen, then shadow the speaker")
                            .size(12.0)
                            .color(MUTED),
                    );
                });
                ui.add_space(12.0);
                ui.columns(2, |columns| {
                    if video_card(
                        &mut columns[0],
                        "Bienvenue !",
                        "A short introduction to your TEF Canada path.",
                    ) {
                        open_unit = Some(usize::MAX);
                    }
                    if video_card(
                        &mut columns[1],
                        "Au café",
                        "Shadow a natural café conversation, one line at a time.",
                    ) {
                        open_unit = Some(usize::MAX - 1);
                    }
                });

                ui.add_space(24.0);
                Frame::new()
                    .fill(Color32::from_rgb(239, 237, 231))
                    .corner_radius(CornerRadius::same(14))
                    .inner_margin(Margin::same(15))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("YOUR PROGRESS, YOUR MAC")
                                        .size(10.0)
                                        .strong()
                                        .color(NAVY),
                                );
                                ui.label(
                                    RichText::new(
                                        "Saved locally. No account, tracking, or internet needed.",
                                    )
                                    .size(12.0)
                                    .color(MUTED),
                                );
                            });
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    if ui
                                        .add(action_button("Open classic course", false))
                                        .clicked()
                                    {
                                        open_unit = Some(usize::MAX - 2);
                                    }
                                    if ui
                                        .add(action_button("Open in default browser", false))
                                        .clicked()
                                    {
                                        open_unit = Some(usize::MAX - 4);
                                    }
                                    if ui.add(action_button("Find course file…", false)).clicked()
                                    {
                                        open_unit = Some(usize::MAX - 5);
                                    }
                                    if ui.add(action_button("Import backup…", false)).clicked() {
                                        open_unit = Some(usize::MAX - 3);
                                    }
                                });
                            });
                        });
                    });
                ui.add_space(20.0);
            });

        if focused_home_entry {
            self.pending_page_focus = None;
        }
        match open_unit {
            Some(index) if index == usize::MAX => self.open_video("bienvenue.mp4"),
            Some(index) if index == usize::MAX - 1 => self.open_video("au-cafe.mp4"),
            Some(index) if index == usize::MAX - 2 => self.open_classic_course(),
            Some(index) if index == usize::MAX - 3 => self.import_progress(),
            Some(index) if index == usize::MAX - 4 => self.open_default_browser_course(),
            Some(index) if index == usize::MAX - 5 => self.reveal_course_file(),
            Some(index) => self.open_unit(index),
            None => {}
        }
    }

    fn show_lesson(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, index: usize) {
        let Some(unit) = self.course.units.get(index).cloned() else {
            self.return_home();
            return;
        };
        let image_path = self
            .resources
            .join("images")
            .join(format!("{}.jpg", unit.id));
        let image_key = format!("unit-{}", unit.id);
        let illustration = self.load_texture(ctx, &image_key, &image_path);
        let focus_lesson_entry = self.pending_page_focus == Some(FocusTarget::LessonEntry);

        ScrollArea::vertical()
            .id_salt(("lesson-scroll", unit.id.clone()))
            .show(ui, |ui| {
                ui.add_space(22.0);
                lesson_intro(
                    ui,
                    &unit,
                    index + 1,
                    self.progress.is_complete(&unit.id),
                    illustration.as_ref(),
                );
                ui.add_space(18.0);
                let mut selected_tab = None;
                ui.horizontal_wrapped(|ui| {
                    for tab in LessonTab::ALL {
                        let selected = self.lesson_tab == tab;
                        let button = Button::new(
                            RichText::new(tab.label())
                                .size(13.0)
                                .strong()
                                .color(if selected { WHITE } else { NAVY }),
                        )
                        .fill(if selected { NAVY } else { WHITE })
                        .min_size(Vec2::new(130.0, 44.0));
                        let response = ui.add(button);
                        if focus_lesson_entry && tab == LessonTab::Words {
                            response.request_focus();
                            self.pending_page_focus = None;
                        }
                        if response.clicked() {
                            selected_tab = Some(tab);
                        }
                    }
                });
                if let Some(tab) = selected_tab.filter(|tab| *tab != self.lesson_tab) {
                    self.lesson_tab = tab;
                    self.audio.stop();
                    if tab == LessonTab::Quiz
                        && self
                            .quiz
                            .as_ref()
                            .is_none_or(|session| session.unit_index != index)
                    {
                        self.quiz = Some(QuizSession::new(index));
                    }
                }

                ui.add_space(14.0);
                match self.lesson_tab {
                    LessonTab::Words => self.show_words(ui, &unit),
                    LessonTab::Phrases => {
                        self.show_lines(ui, "Useful phrases", &unit.phrases.clone())
                    }
                    LessonTab::Grammar => self.show_grammar(ui, &unit),
                    LessonTab::Dialogue => self.show_dialogue(ui, &unit),
                    LessonTab::Quiz => self.show_quiz(ui, ctx, index, &unit),
                }
                ui.add_space(24.0);
            });
    }

    fn show_words(&mut self, ui: &mut egui::Ui, unit: &Unit) {
        if let Some(pronunciation) = &unit.pronunciation {
            Frame::new()
                .fill(PALE_BLUE)
                .stroke(Stroke::new(1.0, Color32::from_rgb(214, 225, 243)))
                .corner_radius(CornerRadius::same(16))
                .inner_margin(Margin::same(20))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(&pronunciation.title)
                            .size(17.0)
                            .strong()
                            .color(NAVY),
                    );
                    ui.add_space(8.0);
                    for (index, point) in pronunciation.points.iter().enumerate() {
                        ui.horizontal_top(|ui| {
                            number_dot(ui, index + 1, BLUE);
                            ui.label(RichText::new(point).size(13.0).color(INK));
                        });
                        ui.add_space(5.0);
                    }
                });
            ui.add_space(18.0);
        }
        self.show_lines(ui, "Key vocabulary", &unit.vocab.clone());
    }

    fn show_lines(&mut self, ui: &mut egui::Ui, title: &str, lines: &[StudyLine]) {
        section_heading(ui, title, format!("{} examples", lines.len()));
        ui.add_space(8.0);
        for line in lines {
            let mut play = false;
            Frame::new()
                .fill(WHITE)
                .stroke(Stroke::new(1.0, BORDER))
                .corner_radius(CornerRadius::same(14))
                .inner_margin(Margin::same(15))
                .show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new(&line.fr).size(17.0).strong().color(NAVY));
                            ui.label(RichText::new(&line.en).size(13.0).color(MUTED));
                            if let Some(tip) = &line.tip {
                                ui.add_space(4.0);
                                ui.label(
                                    RichText::new(format!("TIP  {tip}")).size(11.0).color(BLUE),
                                );
                            }
                        });
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            let response = ui.add(action_button("▶  Listen", false));
                            response.widget_info(|| {
                                egui::WidgetInfo::labeled(
                                    egui::WidgetType::Button,
                                    true,
                                    format!("Listen to {}", line.fr),
                                )
                            });
                            if response.clicked() {
                                play = true;
                            }
                        });
                    });
                });
            if play {
                self.play_audio(line);
            }
            ui.add_space(7.0);
        }
        if lines.is_empty() {
            empty_state(ui, "This section is coming soon.");
        }
    }

    fn show_grammar(&mut self, ui: &mut egui::Ui, unit: &Unit) {
        let Some(grammar) = &unit.grammar else {
            empty_state(ui, "No grammar note is available for this lesson yet.");
            return;
        };
        Frame::new()
            .fill(WHITE)
            .stroke(Stroke::new(1.0, BORDER))
            .corner_radius(CornerRadius::same(16))
            .inner_margin(Margin::same(22))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(&grammar.title)
                        .size(18.0)
                        .strong()
                        .color(NAVY),
                );
                ui.add_space(9.0);
                ui.label(RichText::new(&grammar.explanation).size(14.0).color(INK));
            });
        ui.add_space(18.0);
        self.show_lines(ui, "Examples", &grammar.examples);
    }

    fn show_dialogue(&mut self, ui: &mut egui::Ui, unit: &Unit) {
        let Some(dialogue) = &unit.dialogue else {
            empty_state(ui, "No dialogue is available for this lesson yet.");
            return;
        };
        section_heading(
            ui,
            &dialogue.title,
            format!("{} lines · listen and repeat", dialogue.lines.len()),
        );
        ui.add_space(10.0);
        for (i, line) in dialogue.lines.iter().enumerate() {
            let mut play = false;
            let fill = if i % 2 == 0 {
                WHITE
            } else {
                Color32::from_rgb(243, 246, 250)
            };
            Frame::new()
                .fill(fill)
                .stroke(Stroke::new(1.0, BORDER))
                .corner_radius(CornerRadius::same(14))
                .inner_margin(Margin::same(15))
                .show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        number_dot(ui, i + 1, if line.speaker == "A" { BLUE } else { CORAL });
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!("PERSON {}", line.speaker))
                                    .size(9.0)
                                    .strong()
                                    .color(MUTED),
                            );
                            ui.label(RichText::new(&line.fr).size(16.0).strong().color(NAVY));
                            ui.label(RichText::new(&line.en).size(12.5).color(MUTED));
                        });
                        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                            let response = ui.add(action_button("▶  Listen", false));
                            response.widget_info(|| {
                                egui::WidgetInfo::labeled(
                                    egui::WidgetType::Button,
                                    true,
                                    format!("Listen to {}", line.fr),
                                )
                            });
                            if response.clicked() {
                                play = true;
                            }
                        });
                    });
                });
            if play {
                self.play_text_audio(line.audio.as_deref(), &line.fr);
            }
            ui.add_space(7.0);
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
            if focused {
                if let Some(session) = self.quiz.as_mut() {
                    session.focus_result = false;
                }
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
        if let Some(choice) = pick_answer {
            if let Some(session) = self.quiz.as_mut() {
                if session.selected.is_none() {
                    session.selected = Some(choice);
                    session.focus_next_question = true;
                    if unit.quiz[session.question_index].answer == choice {
                        session.correct += 1;
                    }
                }
            }
        }
        let mut result_to_save = None;
        if next_question {
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
                Ok(()) => {
                    if let Some(session) = self.quiz.as_mut() {
                        session.saved = true;
                        session.save_error = None;
                        if retry_save {
                            session.focus_result = true;
                        }
                    }
                    self.progress_load_error = false;
                    self.set_notice(
                        if passed {
                            format!("Unit complete — {score}%! The next lesson is unlocked.")
                        } else {
                            format!("Saved your {score}% score. Try again when you’re ready.")
                        },
                        passed,
                    );
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
        ui.style_mut().spacing.item_spacing = Vec2::new(10.0, 10.0);
        ui.style_mut().spacing.button_padding = Vec2::new(15.0, 9.0);
        let ctx = ui.ctx().clone();
        Panel::top("app_header")
            .resizable(false)
            .frame(
                Frame::new()
                    .fill(WHITE)
                    .stroke(Stroke::new(1.0, BORDER))
                    .inner_margin(Margin::same(14)),
            )
            .show(ui, |ui| self.show_header(ui));
        if ctx.content_rect().width() >= 980.0 {
            Panel::left("course_navigation")
                .exact_size(250.0)
                .resizable(false)
                .frame(
                    Frame::new()
                        .fill(WHITE)
                        .stroke(Stroke::new(1.0, BORDER))
                        .inner_margin(Margin::same(18)),
                )
                .show(ui, |ui| self.show_sidebar(ui));
        }
        egui::CentralPanel::default()
            .frame(Frame::new().fill(BACKGROUND).inner_margin(Margin::same(0)))
            .show(ui, |ui| {
                ui.add_space(1.0);
                if let Some((message, created, success)) = &self.notice {
                    if self.notice_sticky || created.elapsed() < Duration::from_secs(5) {
                        notice_bar(ui, message, *success);
                        ctx.request_repaint_after(Duration::from_millis(200));
                    }
                }
                match self.page {
                    Page::Home => self.show_home(ui, &ctx),
                    Page::Lesson(index) => self.show_lesson(ui, &ctx, index),
                }
            });
        if self.notice.as_ref().is_some_and(|(_, created, _)| {
            !self.notice_sticky && created.elapsed() >= Duration::from_secs(5)
        }) {
            self.notice = None;
        }
    }
}

fn resolve_resources() -> PathBuf {
    if let Ok(executable) = std::env::current_exe() {
        if let Some(contents) = executable.parent().and_then(Path::parent) {
            let bundled = contents.join("Resources").join("Course");
            if bundled.join("course.json").exists() {
                return bundled;
            }
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("course")
}

fn apply_theme(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
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
    visuals.faint_bg_color = Color32::from_rgb(246, 247, 250);
    visuals.extreme_bg_color = WHITE;
    visuals.hyperlink_color = BLUE;
    visuals.selection.bg_fill = Color32::from_rgb(214, 232, 240);
    visuals.widgets.noninteractive.bg_fill = WHITE;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, INK);
    visuals.widgets.inactive.bg_fill = WHITE;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, NAVY);
    visuals.widgets.hovered.bg_fill = PALE_BLUE;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, NAVY);
    visuals.widgets.active.bg_fill = TEAL;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, WHITE);
    visuals.widgets.open.bg_fill = PALE_BLUE;
    ctx.set_visuals(visuals);
    ctx.style_mut_of(egui::Theme::Light, |style| {
        style.spacing.interact_size = Vec2::new(44.0, 44.0);
        style.visuals.window_corner_radius = CornerRadius::same(18);
    });
}

fn home_hero(
    ui: &mut egui::Ui,
    next: &Unit,
    unit_number: usize,
    done: usize,
    image: Option<&egui::TextureHandle>,
    focus_entry: bool,
) -> (bool, bool) {
    let mut clicked = false;
    let mut focused = false;
    Frame::new()
        .fill(NAVY)
        .corner_radius(CornerRadius::same(22))
        .inner_margin(Margin::same(24))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("TEF CANADA · YOUR STUDY PLAN")
                            .size(10.0)
                            .strong()
                            .color(Color32::from_rgb(173, 207, 246)),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(if done == 0 {
                            "Bienvenue !"
                        } else {
                            "Bon retour !"
                        })
                        .size(30.0)
                        .strong()
                        .color(WHITE),
                    );
                    ui.label(
                        RichText::new("Build your French, one confident step at a time.")
                            .size(14.0)
                            .color(Color32::from_rgb(224, 231, 243)),
                    );
                    ui.add_space(14.0);
                    ui.horizontal(|ui| {
                        number_dot(ui, unit_number, TEAL);
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(format!(
                                    "{} · UNIT {unit_number}",
                                    if done == 0 {
                                        "START HERE"
                                    } else {
                                        "CONTINUE WITH"
                                    }
                                ))
                                .size(9.0)
                                .strong()
                                .color(Color32::from_rgb(173, 207, 246)),
                            );
                            ui.label(RichText::new(&next.title).size(16.0).strong().color(WHITE));
                            ui.label(
                                RichText::new(&next.title_fr)
                                    .size(12.0)
                                    .italics()
                                    .color(Color32::from_rgb(224, 231, 243)),
                            );
                        });
                    });
                    ui.add_space(12.0);
                    let response = ui.add(action_button(
                        if done == 0 {
                            "Start learning  →"
                        } else {
                            "Continue lesson  →"
                        },
                        true,
                    ));
                    if focus_entry {
                        response.request_focus();
                        focused = true;
                    }
                    if response.clicked() {
                        clicked = true;
                    }
                    ui.add_space(7.0);
                    ui.label(
                        RichText::new(format!(
                            "Pass {PASS_SCORE}% on the quiz to unlock your next lesson."
                        ))
                        .size(11.0)
                        .color(Color32::from_rgb(224, 231, 243)),
                    );
                });
                if let Some(image) = image {
                    ui.add_space(12.0);
                    let width = (ui.available_width() * 0.34).clamp(185.0, 280.0);
                    let size = Vec2::new(width, width * 0.5);
                    Frame::new()
                        .fill(Color32::from_rgb(255, 246, 222))
                        .corner_radius(CornerRadius::same(16))
                        .inner_margin(Margin::same(4))
                        .show(ui, |ui| {
                            ui.add(Image::new((image.id(), size)).fit_to_exact_size(size));
                        });
                }
            });
        });
    (clicked, focused)
}

fn completion_hero(ui: &mut egui::Ui, total: usize, focus_entry: bool) -> bool {
    let mut focused = false;
    Frame::new()
        .fill(Color32::from_rgb(27, 58, 75))
        .corner_radius(CornerRadius::same(22))
        .inner_margin(Margin::same(28))
        .show(ui, |ui| {
            ui.label(
                RichText::new("COURSE COMPLETE")
                    .size(10.0)
                    .strong()
                    .color(Color32::from_rgb(169, 221, 206)),
            );
            ui.add_space(7.0);
            let response = ui.add(
                Label::new(
                    RichText::new("Bravo ! You did it.")
                        .size(31.0)
                        .strong()
                        .color(WHITE),
                )
                .sense(Sense::focusable_noninteractive()),
            );
            if focus_entry {
                response.request_focus();
                focused = true;
            }
            ui.label(
                RichText::new(format!(
                    "All {total} lessons are complete. Revisit any level to keep your French sharp."
                ))
                .size(14.0)
                .color(WHITE),
            );
        });
    focused
}

fn metric_card(ui: &mut egui::Ui, kicker: &str, value: String, detail: String, progress: f32) {
    Frame::new()
        .fill(WHITE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(15))
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            ui.set_min_height(93.0);
            ui.label(RichText::new(kicker).size(9.0).strong().color(MUTED));
            ui.add_space(2.0);
            ui.label(RichText::new(value).size(16.0).strong().color(NAVY));
            ui.label(RichText::new(detail).size(11.0).color(MUTED));
            ui.add_space(4.0);
            progress_track(ui, progress, 4.0, ui.available_width(), TEAL);
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

fn level_header(
    ui: &mut egui::Ui,
    level: &Level,
    done: usize,
    total: usize,
    image: Option<&egui::TextureHandle>,
) {
    Frame::new()
        .fill(WHITE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(16))
        .inner_margin(Margin::same(14))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if let Some(image) = image {
                    let size = Vec2::new(105.0, 52.0);
                    ui.add(Image::new((image.id(), size)).fit_to_exact_size(size));
                }
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("Niveau {}", level.n))
                                .size(17.0)
                                .strong()
                                .color(NAVY),
                        );
                        ui.label(RichText::new(&level.cefr).size(10.0).strong().color(BLUE));
                    });
                    ui.label(RichText::new(&level.title).size(13.0).color(INK));
                    ui.label(RichText::new(&level.desc).size(11.0).color(MUTED));
                });
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{done} / {total} complete"))
                            .size(11.0)
                            .strong()
                            .color(if done == total { GREEN } else { MUTED }),
                    );
                });
            });
        });
}

fn show_unit_card(
    ui: &mut egui::Ui,
    unit: &Unit,
    number: usize,
    progress: &Progress,
    can_open: bool,
    unlock_title: Option<&str>,
) -> bool {
    let complete = progress.is_complete(&unit.id);
    let accent = parse_color(&unit.color);
    let mut open = false;
    Frame::new()
        .fill(if can_open {
            WHITE
        } else {
            Color32::from_rgb(249, 248, 245)
        })
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(16))
        .inner_margin(Margin::same(15))
        .show(ui, |ui| {
            ui.set_min_height(157.0);
            ui.horizontal(|ui| {
                number_dot(
                    ui,
                    number,
                    if complete {
                        GREEN
                    } else if can_open {
                        accent
                    } else {
                        Color32::from_rgb(178, 180, 185)
                    },
                );
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(format!("UNIT {number:02}"))
                            .size(9.0)
                            .strong()
                            .color(MUTED),
                    );
                    ui.label(
                        RichText::new(&unit.title)
                            .size(15.0)
                            .strong()
                            .color(if can_open { NAVY } else { MUTED }),
                    );
                    ui.label(
                        RichText::new(&unit.title_fr)
                            .size(11.5)
                            .italics()
                            .color(MUTED),
                    );
                });
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    let (badge, color) = if complete {
                        ("COMPLETED", GREEN)
                    } else if can_open {
                        ("UP NEXT", BLUE)
                    } else {
                        ("LOCKED", MUTED)
                    };
                    ui.label(RichText::new(badge).size(8.5).strong().color(color));
                });
            });
            ui.add_space(7.0);
            let objective = unit
                .objectives
                .first()
                .map(String::as_str)
                .unwrap_or("Build your French step by step.");
            ui.label(RichText::new(objective).size(11.5).color(INK));
            ui.add_space(7.0);
            ui.horizontal(|ui| {
                if can_open {
                    if ui
                        .add(action_button(
                            if complete {
                                "Review lesson"
                            } else {
                                "Start lesson"
                            },
                            !complete,
                        ))
                        .clicked()
                    {
                        open = true;
                    }
                    if let Some(score) = progress.scores.get(&unit.id) {
                        ui.label(
                            RichText::new(format!("Best score · {score}%"))
                                .size(10.0)
                                .color(MUTED),
                        );
                    }
                } else {
                    ui.label(
                        RichText::new(unlock_title.unwrap_or("Pass the previous quiz to unlock"))
                            .size(10.5)
                            .color(MUTED),
                    );
                }
            });
        });
    ui.add_space(9.0);
    open
}

fn video_card(ui: &mut egui::Ui, title: &str, detail: &str) -> bool {
    let mut open = false;
    Frame::new()
        .fill(WHITE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(16))
        .inner_margin(Margin::same(17))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(54.0, 46.0), Sense::hover());
                ui.painter()
                    .rect_filled(rect, CornerRadius::same(12), PALE_BLUE);
                ui.painter().text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "▶",
                    FontId::proportional(19.0),
                    BLUE,
                );
                ui.vertical(|ui| {
                    ui.label(RichText::new(title).size(15.0).strong().color(NAVY));
                    ui.label(RichText::new(detail).size(11.0).color(MUTED));
                });
            });
            ui.add_space(9.0);
            open = ui.add(action_button("Watch video  ↗", false)).clicked();
        });
    ui.add_space(8.0);
    open
}

fn lesson_intro(
    ui: &mut egui::Ui,
    unit: &Unit,
    number: usize,
    complete: bool,
    image: Option<&egui::TextureHandle>,
) {
    Frame::new()
        .fill(WHITE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(19))
        .inner_margin(Margin::same(22))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&unit.icon).size(18.0));
                        ui.label(
                            RichText::new(format!("NIVEAU {} · UNIT {number:02}", unit.level))
                                .size(10.0)
                                .strong()
                                .color(BLUE),
                        );
                        if complete {
                            ui.label(RichText::new("✓ COMPLETED").size(9.0).strong().color(GREEN));
                        }
                    });
                    ui.add_space(5.0);
                    ui.label(RichText::new(&unit.title).size(28.0).strong().color(NAVY));
                    ui.label(
                        RichText::new(&unit.title_fr)
                            .size(16.0)
                            .italics()
                            .color(MUTED),
                    );
                    ui.add_space(11.0);
                    for objective in &unit.objectives {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("•").size(15.0).strong().color(TEAL));
                            ui.label(RichText::new(objective).size(12.0).color(INK));
                        });
                    }
                });
                if let Some(image) = image {
                    ui.add_space(12.0);
                    let width = ui.available_width().clamp(170.0, 250.0);
                    let size = Vec2::new(width, width * 0.62);
                    Frame::new()
                        .fill(PALE_BLUE)
                        .corner_radius(CornerRadius::same(14))
                        .inner_margin(Margin::same(3))
                        .show(ui, |ui| {
                            ui.add(Image::new((image.id(), size)).fit_to_exact_size(size));
                        });
                }
            });
        });
}

fn section_heading(ui: &mut egui::Ui, title: &str, detail: String) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(title).size(20.0).strong().color(NAVY));
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.label(RichText::new(detail).size(11.0).color(MUTED));
        });
    });
}

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
        .corner_radius(CornerRadius::same(19))
        .inner_margin(Margin::same(24))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "QUESTION {:02} / {:02}",
                        session.question_index + 1,
                        total
                    ))
                    .size(10.0)
                    .strong()
                    .color(BLUE),
                );
                if question.kind == "listening" {
                    ui.label(
                        RichText::new("LISTENING PRACTICE")
                            .size(9.0)
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
                        .size(11.0)
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
                let button = Button::new(RichText::new(label).size(14.0).color(text_color))
                    .fill(fill)
                    .stroke(Stroke::new(1.0, stroke))
                    .corner_radius(CornerRadius::same(12))
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
                    .corner_radius(CornerRadius::same(13))
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
                        ui.label(RichText::new(&question.explain).size(12.0).color(INK));
                        if question.kind == "listening" {
                            if let Some(transcript) = &question.transcript {
                                ui.add_space(6.0);
                                ui.label(
                                    RichText::new(format!("You heard: “{transcript}”"))
                                        .size(12.0)
                                        .strong()
                                        .color(NAVY),
                                );
                            }
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
        .corner_radius(CornerRadius::same(21))
        .inner_margin(Margin::same(28))
        .show(ui, |ui| {
            ui.label(RichText::new(if passed && can_continue { "UNIT COMPLETE" } else { "PRACTICE COMPLETE" }).size(10.0).strong().color(if passed && can_continue { Color32::from_rgb(164, 223, 200) } else { Color32::from_rgb(213, 203, 231) }));
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
            ui.label(RichText::new(detail).size(12.0).color(Color32::from_rgb(232, 235, 243)));
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

fn action_button(label: &str, primary: bool) -> Button<'_> {
    let button = Button::new(RichText::new(label).size(12.0).strong().color(if primary {
        WHITE
    } else {
        NAVY
    }))
    .min_size(Vec2::new(124.0, 43.0))
    .corner_radius(CornerRadius::same(12));
    if primary {
        button.fill(TEAL)
    } else {
        button.fill(Color32::from_rgb(245, 247, 250))
    }
}

fn nav_button(label: &str, active: bool) -> Button<'_> {
    Button::new(
        RichText::new(label)
            .size(12.0)
            .strong()
            .color(if active { NAVY } else { MUTED }),
    )
    .fill(if active { PALE_BLUE } else { WHITE })
    .corner_radius(CornerRadius::same(10))
    .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
}

fn number_dot(ui: &mut egui::Ui, number: usize, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(33.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), 16.5, color);
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        format!("{number:02}"),
        FontId::proportional(10.0),
        WHITE,
    );
}

fn empty_state(ui: &mut egui::Ui, text: &str) {
    Frame::new()
        .fill(WHITE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(15))
        .inner_margin(Margin::same(20))
        .show(ui, |ui| {
            ui.label(RichText::new(text).size(13.0).color(MUTED));
        });
}

fn notice_bar(ui: &mut egui::Ui, message: &str, success: bool) {
    Frame::new()
        .fill(if success {
            PALE_GREEN
        } else {
            Color32::from_rgb(255, 239, 232)
        })
        .stroke(Stroke::new(
            1.0,
            if success {
                Color32::from_rgb(203, 230, 212)
            } else {
                Color32::from_rgb(246, 214, 199)
            },
        ))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.label(
                RichText::new(message)
                    .size(12.0)
                    .color(if success { GREEN } else { CORAL }),
            );
        });
}

fn parse_color(raw: &str) -> Color32 {
    let value = raw.strip_prefix('#').unwrap_or(raw);
    if let Ok(value) = u32::from_str_radix(value, 16) {
        Color32::from_rgb(
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
        )
    } else {
        BLUE
    }
}

const fn color(hex: u32) -> Color32 {
    Color32::from_rgb(
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    )
}

const BACKGROUND: Color32 = color(0xf4f2ec);
const WHITE: Color32 = color(0xffffff);
const NAVY: Color32 = color(0x1c2a4b);
const INK: Color32 = color(0x313746);
const MUTED: Color32 = color(0x697282);
const BLUE: Color32 = color(0x316fbd);
const TEAL: Color32 = color(0x188c82);
const CORAL: Color32 = color(0xc55043);
const GREEN: Color32 = color(0x287549);
const BORDER: Color32 = color(0xe2e2de);
const PALE_BLUE: Color32 = color(0xeaf1fb);
const PALE_GREEN: Color32 = color(0xe8f5ec);
