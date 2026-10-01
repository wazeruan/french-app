use crate::model::{Course, Progress};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ReviewCard {
    pub level: u8,
    pub due_day: i64,
    pub last_reviewed: i64,
    pub reviews: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct LearningState {
    pub version: u32,
    pub last_unit: Option<String>,
    pub last_section: usize,
    pub text_scale: f32,
    pub slow_audio: bool,
    pub daily_goal: u32,
    pub cards: BTreeMap<String, ReviewCard>,
    pub reviews_by_day: BTreeMap<String, u32>,
}

impl Default for LearningState {
    fn default() -> Self {
        Self {
            version: 1,
            last_unit: None,
            last_section: 0,
            text_scale: 1.0,
            slow_audio: false,
            daily_goal: 5,
            cards: BTreeMap::new(),
            reviews_by_day: BTreeMap::new(),
        }
    }
}

pub fn word_key(unit_id: &str, french: &str) -> String {
    format!("{unit_id}:{french}")
}

impl LearningState {
    pub fn normalize(&mut self) {
        self.text_scale = if self.text_scale.is_finite() {
            self.text_scale.clamp(1.0, 1.25)
        } else {
            1.0
        };
        self.last_section = self.last_section.min(4);
        self.daily_goal = self.daily_goal.clamp(5, 30);
        self.cards.retain(|key, _| !key.is_empty());
        for card in self.cards.values_mut() {
            card.level = card.level.min(5);
        }
        self.reviews_by_day
            .retain(|day, _| Progress::valid_day(day));
    }

    pub fn toggle_word(&mut self, key: String) -> bool {
        if self.cards.remove(&key).is_some() {
            false
        } else {
            self.cards.insert(key, ReviewCard::default());
            true
        }
    }

    pub fn due_words(&self, course: &Course, progress: &Progress, today: &str) -> Vec<String> {
        let Some(day) = Progress::day_ordinal(today) else {
            return Vec::new();
        };
        let mut due: Vec<_> = course
            .units
            .iter()
            .filter(|unit| progress.can_open(course, unit))
            .flat_map(|unit| {
                unit.vocab
                    .iter()
                    .map(move |line| word_key(&unit.id, &line.fr))
            })
            .filter(|key| self.cards.get(key).is_some_and(|card| card.due_day <= day))
            .collect();
        due.sort_by_key(|key| (self.cards[key].due_day, key.clone()));
        due.dedup();
        due
    }

    pub fn rate_word(&mut self, key: &str, remembered: bool, today: &str) -> bool {
        let Some(day) = Progress::day_ordinal(today) else {
            return false;
        };
        let Some(card) = self.cards.get_mut(key) else {
            return false;
        };
        card.level = if remembered {
            card.level.saturating_add(1).min(5)
        } else {
            0
        };
        let interval = [0, 1, 3, 7, 14, 30][card.level as usize];
        card.due_day = day + interval;
        card.last_reviewed = day;
        card.reviews = card.reviews.saturating_add(1);
        let count = self.reviews_by_day.entry(today.to_owned()).or_default();
        *count = count.saturating_add(1);
        true
    }

    pub fn merge(&mut self, other: Self) {
        self.text_scale = other.text_scale;
        self.slow_audio = other.slow_audio;
        self.daily_goal = other.daily_goal;
        if other.last_unit.is_some() {
            self.last_unit = other.last_unit.clone();
            self.last_section = other.last_section;
        }
        for (key, card) in other.cards {
            let replace = self.cards.get(&key).is_none_or(|existing| {
                (card.last_reviewed, card.reviews) > (existing.last_reviewed, existing.reviews)
            });
            if replace {
                self.cards.insert(key, card);
            }
        }
        for (day, count) in other.reviews_by_day {
            self.reviews_by_day
                .entry(day)
                .and_modify(|existing| *existing = (*existing).max(count))
                .or_insert(count);
        }
        if self.last_unit.is_none() {
            self.last_unit = other.last_unit;
            self.last_section = other.last_section;
        }
        self.normalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(state: &LearningState) -> serde_json::Value {
        serde_json::to_value(state).unwrap()
    }

    #[test]
    fn due_deck_uses_real_vocabulary_and_unlocks_in_course_order() {
        let course: Course = serde_json::from_str(include_str!("../course/course.json")).unwrap();
        assert!(course.units.len() >= 3);
        let keys: Vec<_> = course.units[..3]
            .iter()
            .map(|unit| word_key(&unit.id, &unit.vocab[0].fr))
            .collect();
        let day = Progress::day_ordinal("2026-09-30").unwrap();
        let mut state = LearningState::default();
        for key in &keys {
            state.cards.insert(
                key.clone(),
                ReviewCard {
                    due_day: day,
                    ..Default::default()
                },
            );
        }
        state
            .cards
            .insert("unknown:bonjour".into(), ReviewCard::default());
        let mut progress = Progress::default();
        assert_eq!(
            state.due_words(&course, &progress, "2026-09-30"),
            vec![keys[0].clone()]
        );
        progress.completed.insert(course.units[0].id.clone(), true);
        let mut expected = keys[..2].to_vec();
        expected.sort();
        assert_eq!(state.due_words(&course, &progress, "2026-09-30"), expected);
        state.cards.get_mut(&keys[0]).unwrap().due_day = day + 1;
        assert_eq!(
            state.due_words(&course, &progress, "2026-09-30"),
            vec![keys[1].clone()]
        );
        assert!(state.due_words(&course, &progress, "2026-02-30").is_empty());
    }

    #[test]
    fn remembered_intervals_cap_and_forgetting_resets_to_today() {
        let mut state = LearningState::default();
        let key = word_key("u1", "bonjour");
        assert!(state.toggle_word(key.clone()));
        let day = Progress::day_ordinal("2026-09-30").unwrap();
        for (index, interval) in [1, 3, 7, 14, 30, 30].iter().enumerate() {
            assert!(state.rate_word(&key, true, "2026-09-30"));
            let card = &state.cards[&key];
            assert_eq!(card.level, ((index + 1) as u8).min(5));
            assert_eq!(card.due_day, day + interval);
            assert_eq!(card.last_reviewed, day);
            assert_eq!(card.reviews, (index + 1) as u32);
        }
        assert!(state.rate_word(&key, false, "2026-10-01"));
        let card = &state.cards[&key];
        assert_eq!(card.level, 0);
        assert_eq!(card.due_day, day + 1);
        assert_eq!(card.reviews, 7);
        assert_eq!(state.reviews_by_day["2026-09-30"], 6);
        assert_eq!(state.reviews_by_day["2026-10-01"], 1);
        assert!(!state.toggle_word(key.clone()));
        assert!(!state.cards.contains_key(&key));
    }

    #[test]
    fn invalid_dates_and_unknown_words_do_not_mutate_reviews() {
        let mut state = LearningState::default();
        state.toggle_word("u1:bonjour".into());
        let before = snapshot(&state);
        for day in ["2026-02-30", "2026-13-01", "", "September 30"] {
            assert!(!state.rate_word("u1:bonjour", true, day));
            assert_eq!(snapshot(&state), before);
        }
        assert!(!state.rate_word("missing", false, "2026-09-30"));
        assert_eq!(snapshot(&state), before);
    }

    #[test]
    fn normalization_bounds_preferences_and_removes_invalid_entries() {
        let mut state = LearningState {
            text_scale: f32::NAN,
            daily_goal: 0,
            last_section: usize::MAX,
            ..Default::default()
        };
        state.cards.insert(String::new(), ReviewCard::default());
        state.cards.insert(
            "u1:bonjour".into(),
            ReviewCard {
                level: 255,
                ..Default::default()
            },
        );
        state.reviews_by_day.insert("2026-02-30".into(), 1);
        state.reviews_by_day.insert("2026-09-30".into(), 3);
        state.normalize();
        assert_eq!(
            (state.text_scale, state.daily_goal, state.last_section),
            (1.0, 5, 4)
        );
        assert_eq!(state.cards.len(), 1);
        assert_eq!(state.cards["u1:bonjour"].level, 5);
        assert_eq!(state.reviews_by_day.len(), 1);
        for (scale, expected) in [(0.5, 1.0), (2.0, 1.25), (f32::INFINITY, 1.0), (1.1, 1.1)] {
            state.text_scale = scale;
            state.daily_goal = 100;
            state.normalize();
            assert_eq!(state.text_scale, expected);
            assert_eq!(state.daily_goal, 30);
        }
    }

    #[test]
    fn merging_uses_latest_reviews_and_is_idempotent() {
        let mut state = LearningState {
            last_unit: Some("local".into()),
            text_scale: 1.1,
            ..Default::default()
        };
        state.cards.insert(
            "same".into(),
            ReviewCard {
                last_reviewed: 10,
                reviews: 3,
                level: 1,
                ..Default::default()
            },
        );
        state.cards.insert(
            "local-newer".into(),
            ReviewCard {
                last_reviewed: 20,
                reviews: 1,
                ..Default::default()
            },
        );
        state.reviews_by_day.insert("2026-09-30".into(), 4);
        let mut other = LearningState {
            last_unit: Some("imported".into()),
            last_section: 3,
            text_scale: 1.2,
            slow_audio: true,
            daily_goal: 20,
            ..Default::default()
        };
        other.cards.insert(
            "same".into(),
            ReviewCard {
                last_reviewed: 10,
                reviews: 4,
                level: 2,
                ..Default::default()
            },
        );
        other.cards.insert(
            "local-newer".into(),
            ReviewCard {
                last_reviewed: 19,
                reviews: 9,
                ..Default::default()
            },
        );
        other.cards.insert("new".into(), ReviewCard::default());
        other.reviews_by_day.insert("2026-09-30".into(), 2);
        other.reviews_by_day.insert("2026-10-01".into(), 5);
        state.merge(other.clone());
        assert_eq!(state.cards["same"].level, 2);
        assert_eq!(state.cards["local-newer"].last_reviewed, 20);
        assert_eq!(state.reviews_by_day["2026-09-30"], 4);
        assert_eq!(state.last_unit.as_deref(), Some("imported"));
        assert_eq!(state.last_section, 3);
        assert_eq!(state.text_scale, 1.2);
        assert!(state.slow_audio);
        assert_eq!(state.daily_goal, 20);
        let once = snapshot(&state);
        state.merge(other.clone());
        assert_eq!(snapshot(&state), once);
        let mut empty = LearningState::default();
        empty.merge(other);
        assert_eq!(empty.last_unit.as_deref(), Some("imported"));
        assert_eq!(empty.last_section, 3);
    }
}
