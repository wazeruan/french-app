use serde::{Deserialize, Serialize};

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
    pub completed: std::collections::HashMap<String, bool>,
    pub scores: std::collections::HashMap<String, u32>,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            version: 1,
            completed: std::collections::HashMap::new(),
            scores: std::collections::HashMap::new(),
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

    pub fn record_quiz(&mut self, unit_id: &str, score: u32, passed: bool) {
        self.scores
            .entry(unit_id.to_owned())
            .and_modify(|best| *best = (*best).max(score))
            .or_insert(score);
        if passed {
            self.completed.insert(unit_id.to_owned(), true);
        }
    }

    pub fn merge(&mut self, other: Self) {
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
        self.version = self.version.max(other.version).max(1);
    }
}
