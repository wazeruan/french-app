use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    pub meta: CourseMeta,
    pub units: Vec<Unit>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CourseMeta {
    pub title: String,
    pub subtitle: String,
    pub levels: Vec<Level>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Level {
    pub n: u32,
    pub title: String,
    pub cefr: String,
    pub desc: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub id: String,
    pub level: u32,
    pub title: String,
    pub title_fr: String,
    pub icon: String,
    pub color: String,
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default)]
    pub vocab: Vec<StudyLine>,
    #[serde(default)]
    pub phrases: Vec<StudyLine>,
    pub pronunciation: Option<Pronunciation>,
    pub grammar: Option<Grammar>,
    pub dialogue: Option<Dialogue>,
    #[serde(default)]
    pub quiz: Vec<QuizQuestion>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyLine {
    pub fr: String,
    pub en: String,
    pub tip: Option<String>,
    pub audio: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pronunciation {
    pub title: String,
    pub points: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Grammar {
    pub title: String,
    pub explanation: String,
    #[serde(default)]
    pub examples: Vec<StudyLine>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Dialogue {
    pub title: String,
    #[serde(default)]
    pub lines: Vec<DialogueLine>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DialogueLine {
    pub speaker: String,
    pub fr: String,
    pub en: String,
    pub audio: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct QuizQuestion {
    #[serde(rename = "type")]
    pub kind: String,
    pub q: String,
    #[serde(default)]
    pub choices: Vec<String>,
    pub answer: usize,
    pub explain: String,
    pub audio: Option<String>,
    pub transcript: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Progress {
    pub version: u32,
    pub completed: HashMap<String, bool>,
    pub scores: HashMap<String, u32>,
    pub study_days: BTreeSet<String>,
    pub completed_on: HashMap<String, String>,
    pub quiz_history: Vec<QuizAttempt>,
    pub attempt_sequence: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizAttempt {
    #[serde(default)]
    pub attempt_id: String,
    #[serde(default)]
    pub created_at_ms: u64,
    pub date: Option<String>,
    pub unit_id: String,
    pub score: u32,
    pub passed: bool,
}

#[derive(Clone, Debug)]
pub struct RecentStudyDay {
    pub weekday: &'static str,
    pub day_number: u32,
    pub active: bool,
    pub is_today: bool,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            version: 2,
            completed: HashMap::new(),
            scores: HashMap::new(),
            study_days: BTreeSet::new(),
            completed_on: HashMap::new(),
            quiz_history: Vec::new(),
            attempt_sequence: 0,
        }
    }
}

impl Progress {
    pub fn is_complete(&self, unit_id: &str) -> bool {
        self.completed.get(unit_id).copied().unwrap_or(false)
    }

    pub fn completed_count(&self, course: &Course) -> usize {
        course
            .units
            .iter()
            .filter(|unit| self.is_complete(&unit.id))
            .count()
    }

    pub fn next_unit<'a>(&self, course: &'a Course) -> Option<&'a Unit> {
        course.units.iter().find(|unit| !self.is_complete(&unit.id))
    }

    pub fn can_open(&self, course: &Course, unit: &Unit) -> bool {
        self.is_complete(&unit.id)
            || self
                .next_unit(course)
                .is_some_and(|next| next.id == unit.id)
    }

    pub fn record_study_day(&mut self, day: &str) -> bool {
        if Self::day_ordinal(day).is_none() {
            return false;
        }
        self.study_days.insert(day.to_owned())
    }

    pub fn record_quiz(&mut self, unit_id: &str, score: u32, passed: bool, day: Option<&str>) {
        self.scores
            .entry(unit_id.to_owned())
            .and_modify(|best| *best = (*best).max(score))
            .or_insert(score);
        let date = day.filter(|date| Self::day_ordinal(date).is_some());
        if let Some(date) = date {
            self.record_study_day(date);
        }
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        self.attempt_sequence = self.attempt_sequence.saturating_add(1);
        let attempt_id = format!(
            "{:032x}-{:08x}-{:016x}",
            now.as_nanos(),
            std::process::id(),
            self.attempt_sequence
        );
        self.quiz_history.push(QuizAttempt {
            attempt_id,
            created_at_ms: now.as_millis().min(u64::MAX as u128) as u64,
            date: date.map(str::to_owned),
            unit_id: unit_id.to_owned(),
            score,
            passed,
        });
        if passed {
            self.completed.insert(unit_id.to_owned(), true);
            if let Some(date) = date {
                self.completed_on
                    .entry(unit_id.to_owned())
                    .or_insert_with(|| date.to_owned());
            }
        }
        self.sort_quiz_history();
    }

    pub fn current_streak(&self, today: &str) -> u32 {
        let Some(today) = Self::day_ordinal(today) else {
            return 0;
        };
        let dates = self.study_ordinals();
        let Some(&latest) = dates.last() else {
            return 0;
        };
        if latest < today - 1 || latest > today {
            return 0;
        }

        let mut streak = 1;
        let mut expected = latest - 1;
        for date in dates.iter().rev().skip(1) {
            if *date != expected {
                break;
            }
            streak += 1;
            expected -= 1;
        }
        streak
    }

    pub fn longest_streak(&self) -> u32 {
        let mut longest = 0;
        let mut current = 0;
        let mut previous = None;
        for date in self.study_ordinals() {
            if previous.is_some_and(|previous| date == previous + 1) {
                current += 1;
            } else {
                current = 1;
            }
            longest = longest.max(current);
            previous = Some(date);
        }
        longest
    }

    pub fn recent_study_days(&self, today: &str) -> Vec<RecentStudyDay> {
        let Some(today) = Self::day_ordinal(today) else {
            return Vec::new();
        };
        let active_days: BTreeSet<i64> = self.study_ordinals().into_iter().collect();
        let weekday_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        (0..7)
            .rev()
            .filter_map(|days_ago| {
                let ordinal = today.checked_sub(days_ago)?;
                let (year, month, day_number) = Self::civil_from_days(ordinal);
                if year < 1 || !(1..=12).contains(&month) {
                    return None;
                }
                let weekday = (ordinal + 3).rem_euclid(7) as usize;
                Some(RecentStudyDay {
                    weekday: weekday_names[weekday],
                    day_number,
                    active: active_days.contains(&ordinal),
                    is_today: days_ago == 0,
                })
            })
            .collect()
    }

    pub fn normalize(&mut self) {
        self.study_days
            .retain(|day| Self::day_ordinal(day).is_some());
        self.completed_on.retain(|unit_id, day| {
            self.completed.get(unit_id).copied().unwrap_or(false)
                && Self::day_ordinal(day).is_some()
        });
        self.quiz_history.retain(|attempt| {
            attempt.score <= 100
                && !attempt.unit_id.is_empty()
                && attempt
                    .date
                    .as_deref()
                    .is_none_or(|date| Self::day_ordinal(date).is_some())
        });
        for (index, attempt) in self.quiz_history.iter_mut().enumerate() {
            if attempt.attempt_id.is_empty() {
                attempt.attempt_id = format!(
                    "legacy-{}-{}-{}-{}-{index}",
                    attempt.date.as_deref().unwrap_or("undated"),
                    attempt.unit_id,
                    attempt.score,
                    attempt.passed
                );
            }
            if attempt.created_at_ms == 0 {
                attempt.created_at_ms = attempt
                    .date
                    .as_deref()
                    .and_then(Self::day_ordinal)
                    .and_then(|day| u64::try_from(day).ok())
                    .unwrap_or(0)
                    .saturating_mul(86_400_000)
                    .saturating_add(index as u64);
            }
        }
        let mut seen = HashSet::new();
        self.quiz_history
            .retain(|attempt| seen.insert(attempt.attempt_id.clone()));
        self.sort_quiz_history();
    }

    pub fn valid_day(day: &str) -> bool {
        Self::day_ordinal(day).is_some()
    }

    fn study_ordinals(&self) -> Vec<i64> {
        self.study_days
            .iter()
            .filter_map(|day| Self::day_ordinal(day))
            .collect()
    }

    fn sort_quiz_history(&mut self) {
        self.quiz_history.sort_by(|left, right| {
            left.created_at_ms
                .cmp(&right.created_at_ms)
                .then_with(|| left.attempt_id.cmp(&right.attempt_id))
        });
    }

    fn day_ordinal(date: &str) -> Option<i64> {
        let bytes = date.as_bytes();
        if bytes.len() != 10
            || bytes[4] != b'-'
            || bytes[7] != b'-'
            || !bytes[..4].iter().all(u8::is_ascii_digit)
            || !bytes[5..7].iter().all(u8::is_ascii_digit)
            || !bytes[8..].iter().all(u8::is_ascii_digit)
        {
            return None;
        }
        let year: i64 = date[..4].parse().ok()?;
        let month: i64 = date[5..7].parse().ok()?;
        let day: i64 = date[8..].parse().ok()?;
        if !(1..=12).contains(&month) || year < 1 {
            return None;
        }
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let month_lengths = [
            31,
            if leap { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];
        if !(1..=month_lengths[(month - 1) as usize]).contains(&day) {
            return None;
        }

        let adjusted_year = year - if month <= 2 { 1 } else { 0 };
        let era = adjusted_year.div_euclid(400);
        let year_of_era = adjusted_year - era * 400;
        let adjusted_month = month + if month > 2 { -3 } else { 9 };
        let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        Some(era * 146_097 + day_of_era - 719_468)
    }

    fn civil_from_days(ordinal: i64) -> (i64, i64, u32) {
        let z = ordinal + 719_468;
        let era = z.div_euclid(146_097);
        let day_of_era = z - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let mut year = year_of_era + era * 400;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_prime = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
        let month = month_prime + if month_prime < 10 { 3 } else { -9 };
        year += if month <= 2 { 1 } else { 0 };
        (year, month, day as u32)
    }

    pub fn merge(&mut self, other: Self) {
        let other_version = other.version;
        let other_attempt_sequence = other.attempt_sequence;
        for (unit_id, completed) in other.completed {
            if completed {
                self.completed.insert(unit_id, true);
            }
        }
        for (unit_id, score) in other.scores {
            self.scores
                .entry(unit_id)
                .and_modify(|best| *best = (*best).max(score))
                .or_insert(score);
        }
        for (unit_id, date) in other.completed_on {
            self.completed_on
                .entry(unit_id)
                .and_modify(|existing| {
                    if date < *existing {
                        existing.clone_from(&date);
                    }
                })
                .or_insert(date);
        }
        self.study_days.extend(other.study_days);
        self.quiz_history.extend(other.quiz_history);
        self.attempt_sequence = self.attempt_sequence.max(other_attempt_sequence);
        self.normalize();
        self.version = self.version.max(other_version).max(2);
    }
}
