use super::*;

fn surface<R>(ui: &mut egui::Ui, margin: i8, body: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let width = (ui.available_width() - f32::from(margin) * 2.0 - 2.0).max(1.0);
    Frame::new()
        .fill(WHITE)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(CornerRadius::same(5))
        .inner_margin(Margin::same(margin))
        .show(ui, |ui| {
            ui.set_width(width);
            body(ui)
        })
        .inner
}

fn title(ui: &mut egui::Ui, text: &str, detail: &str) -> egui::Response {
    let response = ui.add(
        Label::new(RichText::new(text).size(30.0).strong().color(NAVY))
            .sense(Sense::focusable_noninteractive()),
    );
    if !detail.is_empty() {
        ui.label(RichText::new(detail).size(15.0).color(MUTED));
    }
    ui.add_space(18.0);
    response
}

fn caption(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).size(12.0).strong().color(MUTED));
}

fn segment(ui: &mut egui::Ui, text: &str, selected: bool) -> egui::Response {
    ui.add(
        Button::new(
            RichText::new(text)
                .size(14.0)
                .color(if selected { TEAL } else { INK }),
        )
        .fill(if selected {
            PALE_GREEN
        } else {
            Color32::TRANSPARENT
        })
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(4))
        .min_size(Vec2::new(0.0, 38.0)),
    )
}

impl FrenchApp {
    fn resume_index(&self) -> Option<usize> {
        self.learning
            .last_unit
            .as_deref()
            .and_then(|id| {
                self.course
                    .units
                    .iter()
                    .position(|unit| unit.id == id && self.progress.can_open(&self.course, unit))
            })
            .or_else(|| self.next_index())
    }

    fn navigate(&mut self, page: Page) {
        self.audio.stop();
        self.audio_active = false;
        self.page = page;
        self.pending_page_focus = Some(match page {
            Page::Home => FocusTarget::Home,
            Page::Journey => FocusTarget::Journey,
            Page::Review => FocusTarget::Review,
            Page::Preferences => FocusTarget::Preferences,
            Page::Lesson(_) => FocusTarget::Lesson,
        });
    }

    pub(super) fn shortcuts(&mut self, ctx: &egui::Context) {
        let (home, review, record, search, stop) = ctx.input(|i| {
            (
                i.modifiers.command && i.key_pressed(egui::Key::Num1),
                i.modifiers.command && i.key_pressed(egui::Key::Num2),
                i.modifiers.command && i.key_pressed(egui::Key::Num3),
                i.modifiers.command && i.key_pressed(egui::Key::F),
                i.key_pressed(egui::Key::Escape),
            )
        });
        if home {
            self.navigate(Page::Home);
        }
        if review {
            self.navigate(Page::Review);
        }
        if record {
            self.navigate(Page::Journey);
        }
        if search {
            self.navigate(Page::Home);
            self.search_focus = true;
        }
        if stop {
            self.audio.stop();
            self.audio_active = false;
        }
    }

    pub(super) fn show_header(&mut self, ui: &mut egui::Ui, compact: bool) {
        let label = match self.page {
            Page::Home => "Learning workspace".to_owned(),
            Page::Review => "Vocabulary review".to_owned(),
            Page::Journey => "Learning record".to_owned(),
            Page::Preferences => "Preferences".to_owned(),
            Page::Lesson(index) => format!(
                "Niveau {}  /  Lesson {:02}",
                self.course.units[index].level,
                index + 1
            ),
        };
        let mut home = false;
        let mut preferences = false;
        let mut slow = false;
        ui.horizontal(|ui| {
            let controls_width = if self.audio_active { 248.0 } else { 172.0 };
            let heading_width = (ui.available_width() - controls_width - 12.0).max(100.0);
            ui.allocate_ui_with_layout(
                Vec2::new(heading_width, 40.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    if matches!(self.page, Page::Lesson(_))
                        && ui
                            .add(icon_action_button("‹", "Back to course library", false))
                            .clicked()
                    {
                        home = true;
                    }
                    ui.label(RichText::new(label).size(14.0).strong().color(INK));
                },
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(icon_action_button("Settings", "Open preferences", false))
                    .clicked()
                {
                    preferences = true;
                }
                let response = ui.add(icon_action_button(
                    if self.slow_audio { "0.72×" } else { "1×" },
                    "Toggle slower audio",
                    self.slow_audio,
                ));
                if response
                    .on_hover_text("Audio speed · click to toggle")
                    .clicked()
                {
                    slow = true;
                }
                if self.audio_active
                    && ui
                        .add(icon_action_button("■ Stop", "Stop audio", false))
                        .clicked()
                {
                    self.audio.stop();
                    self.audio_active = false;
                }
            });
        });
        if compact {
            ui.horizontal_wrapped(|ui| {
                for (page, text) in [
                    (Page::Home, "Learn"),
                    (Page::Review, "Review"),
                    (Page::Journey, "Record"),
                    (Page::Preferences, "Preferences"),
                ] {
                    let selected = self.page == page
                        || (page == Page::Home && matches!(self.page, Page::Lesson(_)));
                    if segment(ui, text, selected).clicked() {
                        self.navigate(page);
                    }
                }
            });
        }
        if home {
            self.return_home();
        }
        if preferences {
            self.navigate(Page::Preferences);
        }
        if slow {
            let mut state = self.learning.clone();
            state.slow_audio = !self.slow_audio;
            if self.save_learning(state) {
                self.slow_audio = self.learning.slow_audio;
            }
        }
    }

    pub(super) fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        let due = self
            .today
            .as_deref()
            .map(|day| {
                self.learning
                    .due_words(&self.course, &self.progress, day)
                    .len()
            })
            .unwrap_or(0);
        ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                Frame::new()
                    .fill(TEAL)
                    .corner_radius(CornerRadius::same(5))
                    .inner_margin(Margin::symmetric(12, 5))
                    .show(ui, |ui| {
                        ui.label(RichText::new("F").size(23.0).strong().color(WHITE));
                    });
                ui.vertical(|ui| {
                    ui.label(RichText::new("Français").size(19.0).strong().color(NAVY));
                    caption(ui, "PAS À PAS");
                });
            });
            ui.add_space(30.0);
            let mut picked = None;
            for (page, icon, text) in [
                (Page::Home, "⌂", "Learn".to_owned()),
                (
                    Page::Review,
                    "↻",
                    if due > 0 {
                        format!("Review  ·  {due}")
                    } else {
                        "Review".to_owned()
                    },
                ),
                (Page::Journey, "↗", "Learning record".to_owned()),
            ] {
                let selected = self.page == page
                    || (page == Page::Home && matches!(self.page, Page::Lesson(_)));
                let response = sidebar_nav_item(ui, icon, &text, selected);
                if self.pending_page_focus == Some(FocusTarget::Home)
                    && page == Page::Home
                    && self.page == Page::Home
                {
                    response.request_focus();
                    self.pending_page_focus = None;
                }
                if response.clicked() {
                    picked = Some(page);
                }
                ui.add_space(4.0);
            }
            if let Some(page) = picked {
                self.navigate(page);
            }
            ui.add_space(26.0);
            caption(ui, "YOUR COURSE");
            ui.add_space(8.0);
            let mut level_pick = None;
            for level in &self.course.meta.levels {
                let count = self
                    .course
                    .units
                    .iter()
                    .filter(|u| u.level == level.n && self.progress.is_complete(&u.id))
                    .count();
                let total = self
                    .course
                    .units
                    .iter()
                    .filter(|u| u.level == level.n)
                    .count();
                if segment(
                    ui,
                    &format!("{}   Niveau {}     {count}/{total}", level.cefr, level.n),
                    self.level_filter == Some(level.n) && self.page == Page::Home,
                )
                .clicked()
                {
                    level_pick = Some(level.n);
                }
            }
            if let Some(level) = level_pick {
                self.navigate(Page::Home);
                self.level_filter = Some(level);
                self.search_query.clear();
                self.status_filter = 0;
                self.pending_level_scroll = Some(level);
            }
            ui.add_space(30.0);
            ui.separator();
            ui.add_space(12.0);
            ui.label(
                RichText::new(format!(
                    "{} of {} lessons complete",
                    self.done_count(),
                    self.course.units.len()
                ))
                .size(13.0)
                .color(MUTED),
            );
            progress_track(
                ui,
                self.done_count() as f32 / self.course.units.len().max(1) as f32,
                4.0,
                ui.available_width(),
                TEAL,
            );
            ui.add_space(14.0);
            if sidebar_nav_item(ui, "⚙", "Preferences", self.page == Page::Preferences).clicked()
            {
                self.navigate(Page::Preferences);
            }
            ui.add_space(18.0);
            ui.label(
                RichText::new("TEF Canada · A1 to B1")
                    .size(12.0)
                    .color(MUTED),
            );
            ui.label(
                RichText::new("Your learning stays on this Mac.")
                    .size(12.0)
                    .color(MUTED),
            );
        });
    }

    pub(super) fn show_home(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let resume = self.resume_index();
        let today = self.today.clone();
        let due = today
            .as_deref()
            .map(|day| {
                self.learning
                    .due_words(&self.course, &self.progress, day)
                    .len()
            })
            .unwrap_or(0);
        let reviewed = today
            .as_deref()
            .and_then(|day| self.learning.reviews_by_day.get(day))
            .copied()
            .unwrap_or(0);
        let streak = today
            .as_deref()
            .map(|day| self.progress.current_streak(day))
            .unwrap_or(0);
        let banner_path = self.resources.join("images/level1.jpg");
        let banner = self.load_texture(ctx, "dashboard-art", &banner_path);
        let mut picked = None;
        let mut review = false;
        let mut review_from_card = false;
        let daily_goal = self.learning.daily_goal;
        let next = self.next_index();
        let mut video = None;
        let mut pending_scroll = self.pending_level_scroll.take();
        ScrollArea::vertical()
            .id_salt("home-scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                title(
                    ui,
                    "A little French, every day.",
                    "Build confidence, one lesson and one conversation at a time.",
                );
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} / {} lessons complete",
                            self.done_count(),
                            self.course.units.len()
                        ))
                        .size(14.0)
                        .strong()
                        .color(INK),
                    );
                    ui.label(
                        RichText::new(format!("   ·   {streak}-day streak"))
                            .size(14.0)
                            .color(MUTED),
                    );
                });
                ui.add_space(18.0);
                let mut continue_card = |ui: &mut egui::Ui| {
                    surface(ui, 22, |ui| {
                        caption(
                            ui,
                            if self.learning.last_unit.is_some() {
                                "PICK UP WHERE YOU LEFT OFF"
                            } else {
                                "YOUR FIRST STEP"
                            },
                        );
                        ui.add_space(10.0);
                        if let Some(index) = resume {
                            let unit = &self.course.units[index];
                            ui.label(RichText::new(&unit.title).size(24.0).strong().color(NAVY));
                            ui.label(
                                RichText::new(format!(
                                    "Lesson {:02}  ·  {}  ·  {}",
                                    index + 1,
                                    self.course
                                        .meta
                                        .levels
                                        .iter()
                                        .find(|l| l.n == unit.level)
                                        .map(|l| l.cefr.as_str())
                                        .unwrap_or(""),
                                    unit.title_fr
                                ))
                                .size(14.0)
                                .color(MUTED),
                            );
                            ui.add_space(16.0);
                            let response = ui.add(action_button(
                                if self.learning.last_unit.is_some() {
                                    "Continue lesson  →"
                                } else {
                                    "Start learning  →"
                                },
                                true,
                            ));
                            if self.pending_page_focus == Some(FocusTarget::Home) {
                                response.request_focus();
                                self.pending_page_focus = None;
                            }
                            if response.clicked() {
                                picked = Some(index);
                            }
                            if let Some(next) = next.filter(|next| *next != index)
                                && ui.link("Go to your next lesson").clicked()
                            {
                                picked = Some(next);
                            }
                        } else {
                            ui.label(
                                RichText::new("Félicitations !")
                                    .size(24.0)
                                    .strong()
                                    .color(NAVY),
                            );
                            ui.label(
                                "You've finished the course. Keep your French fresh with review.",
                            );
                            if ui
                                .add(action_button("Review vocabulary  →", true))
                                .clicked()
                            {
                                review_from_card = true;
                            }
                        }
                    });
                };
                // A balanced pair on larger windows; a single column when reading space is tight.
                if ui.available_width() >= 780.0 {
                    let width = ui.available_width();
                    ui.horizontal_top(|ui| {
                        ui.allocate_ui_with_layout(
                            Vec2::new(width * 0.64 - 8.0, 0.0),
                            Layout::top_down(Align::Min),
                            |ui| continue_card(ui),
                        );
                        ui.allocate_ui_with_layout(
                            Vec2::new(width * 0.36 - 8.0, 0.0),
                            Layout::top_down(Align::Min),
                            |ui| {
                                review_summary(ui, due, reviewed, daily_goal, &mut review);
                            },
                        );
                    });
                } else {
                    continue_card(ui);
                    ui.add_space(12.0);
                    review_summary(ui, due, reviewed, daily_goal, &mut review);
                }
                ui.add_space(28.0);
                ui.label(
                    RichText::new("Your learning path")
                        .size(24.0)
                        .strong()
                        .color(NAVY),
                );
                ui.label(
                    RichText::new(
                        "Explore the course. Pass each quiz with 70% to unlock the next lesson.",
                    )
                    .size(14.0)
                    .color(MUTED),
                );
                ui.add_space(14.0);
                ui.horizontal_wrapped(|ui| {
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text("Search lessons…")
                            .desired_width(280.0)
                            .font(FontId::proportional(15.0)),
                    );
                    if self.search_focus {
                        response.request_focus();
                        self.search_focus = false;
                    }
                    if !self.search_query.is_empty() && segment(ui, "Clear", false).clicked() {
                        self.search_query.clear();
                    }
                    egui::ComboBox::from_id_salt("lesson_status")
                        .selected_text(
                            ["All lessons", "Available", "Completed"][self.status_filter as usize],
                        )
                        .show_ui(ui, |ui| {
                            for (index, label) in
                                ["All lessons", "Available", "Completed"].iter().enumerate()
                            {
                                ui.selectable_value(&mut self.status_filter, index as u8, *label);
                            }
                        });
                });
                ui.horizontal_wrapped(|ui| {
                    if segment(ui, "All levels", self.level_filter.is_none()).clicked() {
                        self.level_filter = None;
                    }
                    for level in &self.course.meta.levels {
                        if segment(
                            ui,
                            &format!("{} · Niveau {}", level.cefr, level.n),
                            self.level_filter == Some(level.n),
                        )
                        .clicked()
                        {
                            self.level_filter = Some(level.n);
                        }
                    }
                });
                ui.add_space(12.0);
                let query = self.search_query.trim().to_lowercase();
                let mut matching = 0;
                for level in &self.course.meta.levels {
                    if self.level_filter.is_some_and(|filter| filter != level.n) {
                        continue;
                    }
                    let units: Vec<_> = self
                        .course
                        .units
                        .iter()
                        .enumerate()
                        .filter(|(_, unit)| unit.level == level.n)
                        .filter(|(_, unit)| {
                            query.is_empty()
                                || format!(
                                    "{} {} {}",
                                    unit.title,
                                    unit.title_fr,
                                    unit.objectives.join(" ")
                                )
                                .to_lowercase()
                                .contains(&query)
                        })
                        .filter(|(_, unit)| match self.status_filter {
                            1 => self.progress.can_open(&self.course, unit),
                            2 => self.progress.is_complete(&unit.id),
                            _ => true,
                        })
                        .collect();
                    if units.is_empty() {
                        continue;
                    }
                    matching += units.len();
                    let start = ui.cursor().min;
                    surface(ui, 16, |ui| {
                        let heading = format!(
                            "{}  ·  {}     {} lessons",
                            level.cefr,
                            level.title,
                            units.len()
                        );
                        let force_open = !query.is_empty()
                            || self.level_filter.is_some()
                            || self.status_filter != 0;
                        let mut header = egui::CollapsingHeader::new(
                            RichText::new(heading).size(16.0).strong().color(NAVY),
                        )
                        .id_salt(("course-level", level.n))
                        .default_open(
                            level.n == 1
                                || resume
                                    .is_some_and(|index| self.course.units[index].level == level.n),
                        );
                        if force_open {
                            header = header.open(Some(true));
                        }
                        header.show(ui, |ui| {
                            ui.label(RichText::new(&level.desc).size(14.0).color(MUTED));
                            ui.add_space(10.0);
                            for (index, unit) in units {
                                let available = self.progress.can_open(&self.course, unit);
                                let done = self.progress.is_complete(&unit.id);
                                let score = self.progress.scores.get(&unit.id).copied();
                                if lesson_row(
                                    ui,
                                    unit,
                                    index + 1,
                                    available,
                                    done,
                                    score,
                                    self.learning.last_unit.as_deref() == Some(&unit.id),
                                ) {
                                    picked = Some(index);
                                }
                                ui.separator();
                            }
                        });
                    });
                    if pending_scroll == Some(level.n) {
                        ui.scroll_to_rect(
                            egui::Rect::from_min_size(start, Vec2::splat(1.0)),
                            Some(Align::Min),
                        );
                        pending_scroll = None;
                    }
                    ui.add_space(12.0);
                }
                if matching == 0 {
                    empty_state(
                        ui,
                        "No lessons match these filters. Clear the search or choose another level.",
                    );
                }
                ui.add_space(14.0);
                surface(ui, 18, |ui| {
                    egui::CollapsingHeader::new(
                        RichText::new("Listening library").size(18.0).strong(),
                    )
                    .id_salt("listening-library")
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(
                                "Watch, listen, and shadow a natural French conversation.",
                            )
                            .size(14.0)
                            .color(MUTED),
                        );
                        ui.horizontal_wrapped(|ui| {
                            if ui.add(action_button("Bienvenue  ↗", false)).clicked() {
                                video = Some("bienvenue.mp4");
                            }
                            if ui.add(action_button("Au café  ↗", false)).clicked() {
                                video = Some("au-cafe.mp4");
                            }
                            if let Some(image) = &banner {
                                ui.add(Image::new((image.id(), Vec2::new(130.0, 65.0))));
                            }
                        });
                    });
                });
                ui.add_space(28.0);
            });
        if let Some(index) = picked {
            self.open_unit(index);
        }
        if review || review_from_card {
            self.navigate(Page::Review);
        }
        if let Some(video) = video {
            self.open_video(video);
        }
    }

    pub(super) fn show_journey(&mut self, ui: &mut egui::Ui) {
        let today = self.today.as_deref();
        let streak = today
            .map(|day| self.progress.current_streak(day))
            .unwrap_or(0);
        let days = today
            .map(|day| self.progress.recent_study_days(day))
            .unwrap_or_default();
        let milestones = milestone_cards(&self.course, &self.progress);
        let mut achievements = achievement_cards(&self.course, &self.progress);
        achievements.extend(learning_achievements(&self.learning));
        let earned = achievements.iter().filter(|award| award.unlocked).count();
        let reviews: u32 = self
            .learning
            .reviews_by_day
            .values()
            .fold(0, |sum, count| sum.saturating_add(*count));
        ScrollArea::vertical().id_salt("learning-record").auto_shrink([false, false]).show(ui, |ui| {
            let heading = title(ui, "Every step counts.", "Your practice, progress, and the milestones you've earned.");
            if self.pending_page_focus == Some(FocusTarget::Journey) { heading.request_focus(); self.pending_page_focus = None; }
            surface(ui, 22, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(format!("{streak}-day streak")).size(22.0).strong().color(TEAL));
                    ui.label(RichText::new(format!("   ·   {} study days   ·   {} lessons complete", self.progress.study_days.len(), self.done_count())).size(15.0).color(INK));
                });
                ui.label(RichText::new(format!("Personal best: {} days in a row  ·  {} quiz attempts  ·  {reviews} vocabulary reviews", self.progress.longest_streak(), self.progress.quiz_history.len())).size(14.0).color(MUTED));
                ui.add_space(18.0);
                ui.columns(7, |columns| {
                    for (column, day) in columns.iter_mut().zip(&days) {
                        let width = (column.available_width() - 10.0).max(1.0);
                        Frame::new().fill(if day.active { PALE_GREEN } else { PALE_SURFACE })
                            .stroke(Stroke::new(1.0, if day.is_today { TEAL } else { Color32::TRANSPARENT }))
                            .corner_radius(CornerRadius::same(4)).inner_margin(Margin::symmetric(4, 10)).show(column, |ui| {
                                ui.set_width(width);
                                ui.vertical_centered(|ui| {
                                    caption(ui, day.weekday);
                                    ui.label(RichText::new(day.day_number.to_string()).size(19.0).strong().color(NAVY));
                                    ui.label(RichText::new(if day.active { "✓" } else { "·" }).size(14.0).color(TEAL));
                                });
                            });
                    }
                });
            });
            ui.add_space(24.0);
            ui.label(RichText::new("Course milestones").size(23.0).strong().color(NAVY));
            ui.label(RichText::new("Small steps toward a bigger goal.").size(14.0).color(MUTED));
            ui.add_space(12.0);
            for milestone in &milestones { award_row(ui, milestone, "lessons"); ui.add_space(8.0); }
            ui.add_space(20.0);
            ui.label(RichText::new(format!("Achievements  ·  {earned} / {}", achievements.len())).size(23.0).strong().color(NAVY));
            ui.add_space(12.0);
            let columns = if ui.available_width() >= 740.0 { 2 } else { 1 };
            ui.columns(columns, |columns| {
                for (index, achievement) in achievements.iter().enumerate() {
                    award_row(&mut columns[index % columns.len()], achievement, "");
                    columns[index % columns.len()].add_space(10.0);
                }
            });
            ui.add_space(20.0);
            ui.label(RichText::new("Recent practice").size(23.0).strong().color(NAVY));
            ui.add_space(12.0);
            surface(ui, 20, |ui| {
                if self.progress.quiz_history.is_empty() && self.learning.reviews_by_day.is_empty() {
                    ui.label(RichText::new("Your finished quizzes and vocabulary reviews will appear here.").size(15.0).color(MUTED));
                }
                for attempt in self.progress.quiz_history.iter().rev().take(10) {
                    let name = self.course.units.iter().find(|unit| unit.id == attempt.unit_id).map(|unit| unit.title.as_str()).unwrap_or("Lesson");
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(format!("{}  ·  {name}", attempt.date.as_deref().unwrap_or("Undated"))).size(15.0).color(INK));
                        ui.label(RichText::new(format!("{}%  ·  {}", attempt.score, if attempt.passed { "Passed" } else { "Keep practising" })).size(14.0).strong().color(if attempt.passed { GREEN } else { MUTED }));
                    });
                    ui.separator();
                }
                for (day, count) in self.learning.reviews_by_day.iter().rev().take(7) {
                    ui.label(RichText::new(format!("{day}  ·  {count} vocabulary reviews")).size(15.0).color(INK));
                    ui.separator();
                }
            });
            ui.add_space(16.0);
            ui.label(RichText::new("A lesson visit or word review records a study day. Imported course completions count toward milestones; older browser activity has no dated history.").size(13.0).color(MUTED));
            ui.add_space(28.0);
        });
    }

    pub(super) fn show_lesson(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, index: usize) {
        let Some(unit) = self.course.units.get(index).cloned() else {
            self.return_home();
            return;
        };
        let mut tab_pick = None;
        ScrollArea::vertical()
            .id_salt(("lesson", &unit.id, self.lesson_tab))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                caption(
                    ui,
                    &format!(
                        "NIVEAU {}  /  LESSON {:02}{}",
                        unit.level,
                        index + 1,
                        if self.progress.is_complete(&unit.id) {
                            "  /  COMPLETED"
                        } else {
                            ""
                        }
                    ),
                );
                ui.add_space(6.0);
                title(ui, &unit.title, &unit.title_fr);
                egui::CollapsingHeader::new(RichText::new("What you'll learn").size(14.0).strong())
                    .default_open(true)
                    .id_salt(("objectives", &unit.id))
                    .show(ui, |ui| {
                        for objective in &unit.objectives {
                            ui.label(
                                RichText::new(format!("•  {objective}"))
                                    .size(14.0)
                                    .color(INK),
                            );
                        }
                    });
                ui.add_space(18.0);
                ui.horizontal_wrapped(|ui| {
                    for (position, tab) in LessonTab::ALL.into_iter().enumerate() {
                        let response = segment(
                            ui,
                            &format!("{}  {}", position + 1, tab.label()),
                            self.lesson_tab == tab,
                        );
                        if self.pending_page_focus == Some(FocusTarget::Lesson)
                            && tab == self.lesson_tab
                        {
                            response.request_focus();
                            self.pending_page_focus = None;
                        }
                        if response.clicked() {
                            tab_pick = Some(tab);
                        }
                    }
                });
                ui.separator();
                ui.add_space(18.0);
                match self.lesson_tab {
                    LessonTab::Words => self.show_words(ui, &unit),
                    LessonTab::Phrases => {
                        self.show_lines(ui, "Useful phrases", &unit.phrases, None)
                    }
                    LessonTab::Grammar => self.show_grammar(ui, &unit),
                    LessonTab::Dialogue => self.show_dialogue(ui, &unit),
                    LessonTab::Quiz => self.show_quiz(ui, ctx, index, &unit),
                }
                ui.add_space(22.0);
                let current = LessonTab::ALL
                    .iter()
                    .position(|tab| *tab == self.lesson_tab)
                    .unwrap_or(0);
                ui.horizontal_wrapped(|ui| {
                    if current > 0 && ui.add(action_button("← Previous section", false)).clicked()
                    {
                        tab_pick = Some(LessonTab::ALL[current - 1]);
                    }
                    if current < 4 {
                        let label = format!("Next: {}  →", LessonTab::ALL[current + 1].label());
                        if ui.add(action_button(&label, true)).clicked() {
                            tab_pick = Some(LessonTab::ALL[current + 1]);
                        }
                    }
                    ui.label(
                        RichText::new(format!("Section {} of 5", current + 1))
                            .size(13.0)
                            .color(MUTED),
                    );
                });
                ui.add_space(28.0);
            });
        if let Some(tab) = tab_pick {
            self.lesson_tab = tab;
            self.audio.stop();
            self.audio_active = false;
            self.pending_page_focus = Some(FocusTarget::Lesson);
            let mut state = self.learning.clone();
            state.last_section = LessonTab::ALL
                .iter()
                .position(|candidate| *candidate == tab)
                .unwrap_or(0);
            self.save_learning(state);
            ctx.request_repaint();
        }
    }

    fn show_words(&mut self, ui: &mut egui::Ui, unit: &Unit) {
        if let Some(pronunciation) = &unit.pronunciation {
            surface(ui, 20, |ui| {
                ui.label(
                    RichText::new(&pronunciation.title)
                        .size(18.0)
                        .strong()
                        .color(NAVY),
                );
                ui.add_space(10.0);
                for (index, point) in pronunciation.points.iter().enumerate() {
                    ui.label(
                        RichText::new(format!("{:02}   {point}", index + 1))
                            .size(15.0)
                            .color(INK),
                    );
                    ui.add_space(5.0);
                }
            });
            ui.add_space(22.0);
        }
        self.show_lines(ui, "Key vocabulary", &unit.vocab, Some(&unit.id));
    }

    fn show_lines(
        &mut self,
        ui: &mut egui::Ui,
        heading: &str,
        lines: &[StudyLine],
        save_unit: Option<&str>,
    ) {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(heading).size(22.0).strong().color(NAVY));
            ui.label(
                RichText::new(format!("{} examples", lines.len()))
                    .size(13.0)
                    .color(MUTED),
            );
            if let Some(unit) = save_unit
                && ui
                    .add(action_button("Add lesson to review", false))
                    .on_hover_text(
                        "Add every vocabulary word from this lesson to your personal review deck.",
                    )
                    .clicked()
            {
                let mut state = self.learning.clone();
                let before = state.cards.len();
                for line in lines {
                    state.cards.entry(word_key(unit, &line.fr)).or_default();
                }
                let added = state.cards.len() - before;
                if self.save_learning(state) {
                    self.set_notice(
                        format!("Added {added} new words to your review deck."),
                        true,
                    );
                }
            }
        });
        ui.add_space(12.0);
        if lines.is_empty() {
            empty_state(ui, "This section is coming soon.");
            return;
        }
        let mut audio_pick = None;
        let mut saved_pick = None;
        let mut stop = false;
        surface(ui, 18, |ui| {
            let count = lines.len();
            for (position, line) in lines.iter().enumerate() {
                let key = save_unit.map(|unit| word_key(unit, &line.fr));
                let saved = key
                    .as_ref()
                    .is_some_and(|key| self.learning.cards.contains_key(key));
                let playing = self.audio_active && self.playing_word.as_deref() == Some(&line.fr);
                let width = ui.available_width();
                let controls = if key.is_some() { 178.0 } else { 102.0 };
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new((width - controls - 14.0).max(120.0), 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            ui.label(RichText::new(&line.fr).size(20.0).strong().color(NAVY));
                            ui.label(RichText::new(&line.en).size(15.0).color(MUTED));
                            if let Some(tip) = &line.tip {
                                ui.add_space(4.0);
                                ui.label(RichText::new(tip).size(13.0).color(BLUE));
                            }
                        },
                    );
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        if let Some(key) = key
                            && ui
                                .add(icon_action_button(
                                    if saved { "✓ Saved" } else { "+ Save" },
                                    &format!(
                                        "{} {} to review",
                                        if saved { "Remove" } else { "Save" },
                                        line.fr
                                    ),
                                    false,
                                ))
                                .clicked()
                        {
                            saved_pick = Some(key);
                        }
                        if ui
                            .add(icon_action_button(
                                if playing { "■ Stop" } else { "▶ Listen" },
                                &format!("Listen to {}", line.fr),
                                playing,
                            ))
                            .clicked()
                        {
                            if playing {
                                stop = true;
                            } else {
                                audio_pick = Some(line.clone());
                            }
                        }
                    });
                });
                if position + 1 < count {
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);
                }
            }
        });
        if let Some(key) = saved_pick {
            let mut state = self.learning.clone();
            state.toggle_word(key);
            self.save_learning(state);
        }
        if stop {
            self.audio.stop();
            self.audio_active = false;
        }
        if let Some(line) = audio_pick {
            self.play_audio(&line);
        }
    }

    fn show_grammar(&mut self, ui: &mut egui::Ui, unit: &Unit) {
        let Some(grammar) = &unit.grammar else {
            empty_state(ui, "No grammar note is available for this lesson.");
            return;
        };
        surface(ui, 22, |ui| {
            ui.label(
                RichText::new(&grammar.title)
                    .size(22.0)
                    .strong()
                    .color(NAVY),
            );
            ui.add_space(12.0);
            ui.label(RichText::new(&grammar.explanation).size(16.0).color(INK));
        });
        ui.add_space(24.0);
        self.show_lines(ui, "Examples", &grammar.examples, None);
    }

    fn show_dialogue(&mut self, ui: &mut egui::Ui, unit: &Unit) {
        let Some(dialogue) = &unit.dialogue else {
            empty_state(ui, "No dialogue is available for this lesson.");
            return;
        };
        title(
            ui,
            &dialogue.title,
            "Listen to each line, then say it aloud.",
        );
        let mut audio = None;
        surface(ui, 20, |ui| {
            for (index, line) in dialogue.lines.iter().enumerate() {
                let width = ui.available_width();
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        Vec2::new((width - 108.0).max(150.0), 0.0),
                        Layout::top_down(Align::Min),
                        |ui| {
                            caption(ui, &format!("PERSON {}", line.speaker));
                            ui.label(RichText::new(&line.fr).size(18.0).strong().color(NAVY));
                            ui.label(RichText::new(&line.en).size(15.0).color(MUTED));
                        },
                    );
                    if ui
                        .add(icon_action_button(
                            "▶ Listen",
                            &format!("Listen to {}", line.fr),
                            false,
                        ))
                        .clicked()
                    {
                        audio = Some((line.audio.clone(), line.fr.clone()));
                    }
                });
                if index + 1 < dialogue.lines.len() {
                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);
                }
            }
        });
        if let Some((id, text)) = audio {
            self.play_text_audio(id.as_deref(), &text);
        }
    }

    pub(super) fn show_review(&mut self, ui: &mut egui::Ui) {
        let available: HashSet<_> = self
            .course
            .units
            .iter()
            .filter(|unit| self.progress.can_open(&self.course, unit))
            .flat_map(|unit| {
                unit.vocab
                    .iter()
                    .map(move |line| word_key(&unit.id, &line.fr))
            })
            .filter(|key| self.learning.cards.contains_key(key))
            .collect();
        if let Some(session) = &mut self.review_session {
            session.reconcile(&available);
        }
        let today = self.today.clone();
        let due = today
            .as_deref()
            .map(|day| self.learning.due_words(&self.course, &self.progress, day))
            .unwrap_or_default();
        let reviewed = today
            .as_deref()
            .and_then(|day| self.learning.reviews_by_day.get(day))
            .copied()
            .unwrap_or(0);
        let mut start = false;
        let mut reveal = false;
        let mut rating = None;
        let mut audio = None;
        let mut remove = None;
        let mut open_lesson = false;
        let mut end = false;
        let keyboard = !ui.ctx().text_edit_focused();
        let session = self.review_session.clone();
        ScrollArea::vertical().id_salt("review-page").auto_shrink([false, false]).show(ui, |ui| {
            let heading = title(ui, "Make it stick.", "A few minutes of recall turn new words into familiar ones.");
            if self.pending_page_focus == Some(FocusTarget::Review) { heading.request_focus(); self.pending_page_focus = None; }
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("{} words due", due.len())).size(15.0).strong().color(TEAL));
                ui.label(RichText::new(format!("   ·   {reviewed} / {} reviews today", self.learning.daily_goal)).size(14.0).color(MUTED));
            });
            ui.add_space(18.0);
            if let Some(session) = &session {
                if let Some(key) = session.keys.get(session.position) {
                    let card = self.course.units.iter().flat_map(|unit| unit.vocab.iter().map(move |line| (unit, line)))
                        .find(|(unit, line)| word_key(&unit.id, &line.fr) == *key);
                    if let Some((unit, line)) = card {
                        surface(ui, 28, |ui| {
                            caption(ui, &format!("CARD {} OF {}  ·  {}", session.position + 1, session.keys.len(), unit.title));
                            ui.add_space(26.0);
                            ui.label(RichText::new(&line.fr).size(32.0).strong().color(NAVY));
                            ui.add_space(12.0);
                            if ui.add(action_button("▶ Listen", false)).clicked() { audio = Some(line.clone()); }
                            ui.add_space(26.0);
                            if session.revealed {
                                ui.separator();
                                ui.add_space(18.0);
                                ui.label(RichText::new(&line.en).size(24.0).color(INK));
                                if let Some(tip) = &line.tip { ui.label(RichText::new(tip).size(15.0).color(MUTED)); }
                                ui.add_space(24.0);
                                ui.horizontal_wrapped(|ui| {
                                    if ui.add(action_button("Practice again  ·  J", false)).clicked() { rating = Some(false); }
                                    if ui.add(action_button("Remembered  ·  K", true)).clicked() { rating = Some(true); }
                                });
                                ui.label(RichText::new("Remembered words return in 1, 3, 7, 14, then 30 days. Practice-again words stay due.").size(13.0).color(MUTED));
                            } else {
                                ui.label(RichText::new("Try to recall the meaning before you reveal it.").size(15.0).color(MUTED));
                                if ui.add(action_button("Reveal meaning  ·  Space", true)).clicked() { reveal = true; }
                            }
                        });
                        if keyboard {
                            ui.ctx().input(|i| {
                                if !i.modifiers.command && !i.modifiers.ctrl && !i.modifiers.alt {
                                    if !session.revealed && i.key_pressed(egui::Key::Space) { reveal = true; }
                                    if session.revealed && i.key_pressed(egui::Key::J) { rating = Some(false); }
                                    if session.revealed && i.key_pressed(egui::Key::K) { rating = Some(true); }
                                }
                            });
                        }
                    }
                } else {
                    surface(ui, 26, |ui| {
                        ui.label(RichText::new("One more step forward.").size(25.0).strong().color(NAVY));
                        ui.label(RichText::new(format!("{} words remembered · {} to practice again", session.remembered, session.keys.len() - session.remembered)).size(16.0).color(INK));
                        ui.add_space(18.0);
                        if !due.is_empty() && ui.add(action_button("Review remaining words", true)).clicked() { start = true; }
                        if ui.add(action_button("Continue a lesson  →", false)).clicked() { open_lesson = true; }
                    });
                }
            } else if due.is_empty() {
                surface(ui, 24, |ui| {
                    ui.label(RichText::new(if self.learning.cards.is_empty() { "Build your personal vocabulary deck." } else { "You're up to date." }).size(23.0).strong().color(NAVY));
                    ui.label(RichText::new("Save words from a lesson to review them here. Words you've remembered return when it's time to practise.").size(16.0).color(MUTED));
                    ui.add_space(18.0);
                    if ui.add(action_button("Open a lesson  →", true)).clicked() { open_lesson = true; }
                });
            } else {
                surface(ui, 24, |ui| {
                    ui.label(RichText::new("Your next small win.").size(24.0).strong().color(NAVY));
                    ui.label(RichText::new(format!("Review up to {} saved words. Reveal each meaning, then tell us whether you remembered it.", self.learning.daily_goal)).size(16.0).color(MUTED));
                    ui.add_space(18.0);
                    let response = ui.add(action_button("Start review  →", true));
                    if self.pending_page_focus == Some(FocusTarget::Review) { response.request_focus(); self.pending_page_focus = None; }
                    if response.clicked() { start = true; }
                });
            }
            let active = session.as_ref().is_some_and(|s| s.position < s.keys.len());
            if active {
                ui.add_space(16.0);
                if ui.add(action_button("End review", false)).clicked() { end = true; }
            }
            if !active {
                ui.add_space(24.0);
                surface(ui, 18, |ui| {
                    egui::CollapsingHeader::new(RichText::new("Your saved vocabulary").size(18.0).strong()).id_salt("saved-vocabulary").show(ui, |ui| {
                        for unit in &self.course.units {
                            if !self.progress.can_open(&self.course, unit) { continue; }
                            for line in &unit.vocab {
                                let key = word_key(&unit.id, &line.fr);
                                if !self.learning.cards.contains_key(&key) { continue; }
                                ui.horizontal_wrapped(|ui| {
                                    ui.label(RichText::new(format!("{}  ·  {}", line.fr, line.en)).size(15.0).color(INK));
                                    if ui.small_button("Remove").clicked() { remove = Some(key); }
                                });
                                ui.separator();
                            }
                        }
                    });
                });
            }
            ui.add_space(28.0);
        });
        if start {
            self.review_session = Some(ReviewSession {
                keys: due
                    .into_iter()
                    .take(self.learning.daily_goal as usize)
                    .collect(),
                position: 0,
                revealed: false,
                remembered: 0,
            });
            self.audio.stop();
            self.audio_active = false;
        }
        if end {
            self.review_session = None;
            self.audio.stop();
            self.audio_active = false;
        }
        if reveal && let Some(session) = &mut self.review_session {
            session.revealed = true;
        }
        if let Some(remembered) = rating {
            self.refresh_today_now();
            let rate_day = self.today.clone();
            if let (Some(session), Some(today)) =
                (self.review_session.as_ref(), rate_day.as_deref())
                && let Some(key) = session.keys.get(session.position)
            {
                let mut learning = self.learning.clone();
                let previous_awards = self.recognition_snapshot();
                if learning.rate_word(key, remembered, today) && self.save_learning(learning) {
                    if let Some(session) = &mut self.review_session {
                        session.position += 1;
                        session.revealed = false;
                        session.remembered += usize::from(remembered);
                    }
                    self.record_study_day();
                    self.audio.stop();
                    self.audio_active = false;
                    if let Some(message) = self.new_recognition_message(&previous_awards) {
                        self.set_notice(message, true);
                    }
                }
            }
        }
        if let Some(key) = remove {
            let mut state = self.learning.clone();
            state.cards.remove(&key);
            self.save_learning(state);
        }
        if let Some(line) = audio {
            self.play_audio(&line);
        }
        if open_lesson {
            if let Some(index) = self.resume_index() {
                self.open_unit(index);
            } else {
                self.return_home();
            }
        }
    }

    pub(super) fn show_preferences(&mut self, ui: &mut egui::Ui) {
        let mut state = self.learning.clone();
        let mut import = false;
        let mut export = false;
        let mut classic = 0;
        ScrollArea::vertical().id_salt("preferences").auto_shrink([false, false]).show(ui, |ui| {
            let heading = title(ui, "Make yourself comfortable.", "Reading, practice, and local backups — all in one place.");
            if self.pending_page_focus == Some(FocusTarget::Preferences) { heading.request_focus(); self.pending_page_focus = None; }
            surface(ui, 22, |ui| {
                ui.label(RichText::new("Reading size").size(19.0).strong().color(NAVY));
                ui.label(RichText::new("Choose the size that feels comfortable for longer lessons.").size(15.0).color(MUTED));
                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(&mut state.text_scale, 1.0, "Standard");
                    ui.radio_value(&mut state.text_scale, 1.1, "Comfortable");
                    ui.radio_value(&mut state.text_scale, 1.2, "Large");
                });
                ui.add_space(20.0); ui.separator(); ui.add_space(16.0);
                ui.label(RichText::new("Audio pace").size(19.0).strong().color(NAVY));
                ui.checkbox(&mut state.slow_audio, "Use slower playback");
                ui.label(RichText::new("Recorded clips play at 0.72×. Voice fallback uses a slower speaking rate.").size(14.0).color(MUTED));
                ui.add_space(20.0); ui.separator(); ui.add_space(16.0);
                ui.label(RichText::new("Daily review goal").size(19.0).strong().color(NAVY));
                ui.horizontal_wrapped(|ui| { for goal in [5, 10, 15, 20] { ui.radio_value(&mut state.daily_goal, goal, format!("{goal} words")); } });
                ui.label(RichText::new("A session includes up to this many due words. You can keep practising after your goal.").size(14.0).color(MUTED));
            });
            ui.add_space(22.0);
            surface(ui, 22, |ui| {
                ui.label(RichText::new("Your learning, safely saved").size(19.0).strong().color(NAVY));
                ui.label(RichText::new("Back up your course progress, quiz history, saved words, review schedule, and preferences. Import merges learning records and restores backed-up preferences.").size(15.0).color(MUTED));
                ui.add_space(14.0);
                ui.horizontal_wrapped(|ui| {
                    if ui.add_enabled(!self.progress_load_error && !self.learning_load_error, action_button("Export backup", true)).clicked() { export = true; }
                    if ui.add(action_button("Import backup", false)).clicked() { import = true; }
                });
                ui.add_space(12.0);
                ui.label(RichText::new("Saved on this Mac. No account or internet connection required.").size(13.0).color(MUTED));
                egui::CollapsingHeader::new("Move progress from the classic course").id_salt("classic-settings").show(ui, |ui| {
                    ui.label("Open the classic course in the browser profile where you studied, export its JSON backup, then import it here.");
                    ui.horizontal_wrapped(|ui| {
                        if ui.add(action_button("Open classic course", false)).clicked() { classic = 1; }
                        if ui.add(action_button("Default browser", false)).clicked() { classic = 2; }
                        if ui.add(action_button("Find course file…", false)).clicked() { classic = 3; }
                    });
                });
            });
            ui.add_space(22.0);
            surface(ui, 22, |ui| {
                ui.label(RichText::new("Keyboard shortcuts").size(19.0).strong().color(NAVY));
                for text in ["⌘1  Learn   ·   ⌘2  Review   ·   ⌘3  Learning record", "⌘F  Find a lesson   ·   Escape  Stop audio", "Review: Space  Reveal   ·   J  Practice again   ·   K  Remembered"] {
                    ui.label(RichText::new(text).size(14.0).color(INK));
                }
                ui.add_space(20.0);
                ui.separator();
                ui.label(RichText::new(&self.course.meta.title).size(15.0).strong().color(NAVY));
                ui.label(RichText::new(&self.course.meta.subtitle).size(14.0).color(MUTED));
            });
            ui.add_space(28.0);
        });
        if (state.text_scale != self.learning.text_scale
            || state.slow_audio != self.learning.slow_audio
            || state.daily_goal != self.learning.daily_goal)
            && self.save_learning(state)
        {
            self.slow_audio = self.learning.slow_audio;
        }
        if import {
            self.import_progress();
        }
        if export {
            self.export_progress();
        }
        match classic {
            1 => self.open_classic_course(),
            2 => self.open_default_browser_course(),
            3 => self.reveal_course_file(),
            _ => {}
        }
    }
}

fn review_summary(ui: &mut egui::Ui, due: usize, reviewed: u32, goal: u32, open: &mut bool) {
    surface(ui, 22, |ui| {
        caption(ui, "DAILY PRACTICE");
        ui.add_space(10.0);
        ui.label(
            RichText::new(if due == 0 {
                "Keep your words fresh.".to_owned()
            } else {
                format!("{due} words ready to review")
            })
            .size(20.0)
            .strong()
            .color(NAVY),
        );
        ui.label(
            RichText::new(format!("{reviewed} of {goal} reviews today"))
                .size(14.0)
                .color(MUTED),
        );
        ui.add_space(10.0);
        progress_track(
            ui,
            reviewed as f32 / goal.max(1) as f32,
            4.0,
            ui.available_width(),
            TEAL,
        );
        ui.add_space(14.0);
        if ui
            .add(action_button(
                if due > 0 {
                    "Review words  →"
                } else {
                    "Your review deck  →"
                },
                false,
            ))
            .clicked()
        {
            *open = true;
        }
    });
}

fn lesson_row(
    ui: &mut egui::Ui,
    unit: &Unit,
    number: usize,
    available: bool,
    done: bool,
    score: Option<u32>,
    resume: bool,
) -> bool {
    let mut open = false;
    let width = ui.available_width();
    ui.add_space(10.0);
    ui.horizontal_top(|ui| {
        ui.allocate_ui_with_layout(
            Vec2::new((width - 142.0).max(180.0), 0.0),
            Layout::top_down(Align::Min),
            |ui| {
                ui.label(
                    RichText::new(format!("{number:02}   {}", unit.title))
                        .size(17.0)
                        .strong()
                        .color(if available { NAVY } else { MUTED }),
                );
                ui.label(RichText::new(&unit.title_fr).size(14.0).color(MUTED));
                if !available {
                    ui.label(
                        RichText::new("Complete the previous lesson to unlock this one.")
                            .size(13.0)
                            .color(MUTED),
                    );
                } else if let Some(score) = score {
                    ui.label(
                        RichText::new(format!("Best quiz score · {score}%"))
                            .size(13.0)
                            .color(if done { GREEN } else { MUTED }),
                    );
                }
            },
        );
        if available {
            open = ui
                .add(action_button(
                    if done {
                        "✓ Review"
                    } else if resume {
                        "Resume  →"
                    } else {
                        "Start  →"
                    },
                    !done,
                ))
                .clicked();
        } else {
            ui.label(RichText::new("Locked").size(14.0).color(MUTED));
        }
    });
    ui.add_space(10.0);
    open
}

fn award_row(ui: &mut egui::Ui, award: &JourneyCardInfo, unit: &str) {
    surface(ui, 16, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(award.icon)
                    .size(15.0)
                    .color(if award.unlocked { GREEN } else { MUTED }),
            );
            ui.label(RichText::new(&award.title).size(17.0).strong().color(NAVY));
            if award.unlocked {
                ui.label(RichText::new("✓ Earned").size(13.0).strong().color(GREEN));
            }
        });
        ui.label(RichText::new(&award.detail).size(14.0).color(MUTED));
        ui.add_space(8.0);
        progress_track(
            ui,
            award.current as f32 / award.target.max(1) as f32,
            4.0,
            ui.available_width(),
            if award.unlocked { GREEN } else { TEAL },
        );
        ui.label(
            RichText::new(format!("{} / {} {unit}", award.current, award.target))
                .size(13.0)
                .color(MUTED),
        );
    });
}
