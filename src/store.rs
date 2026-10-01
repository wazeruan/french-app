use crate::{
    learning::LearningState,
    model::{Course, Progress},
};
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn data_dir() -> PathBuf {
    if let Some(path) = env::var_os("FRENCH_APP_DATA_DIR").filter(|path| !path.is_empty()) {
        return PathBuf::from(path);
    }
    env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
        .join("Library")
        .join("Application Support")
        .join("FrenchApp")
}

pub fn progress_path() -> PathBuf {
    data_dir().join("progress.json")
}

pub fn local_day_key() -> Option<String> {
    #[cfg(target_os = "macos")]
    let output = Command::new("/bin/date").arg("+%F").output();
    #[cfg(not(target_os = "macos"))]
    let output = Command::new("date").arg("+%F").output();

    let output = output.ok()?;
    if !output.status.success() {
        return None;
    }
    let day = String::from_utf8(output.stdout).ok()?;
    let day = day.trim();
    Progress::valid_day(day).then(|| day.to_owned())
}

pub fn load_progress() -> (Progress, Option<String>) {
    let path = progress_path();
    match fs::read(&path) {
        Ok(bytes) => match decode_saved_progress(&bytes) {
            Ok(progress) => (progress, None),
            Err(error) => (
                Progress::default(),
                Some(format!(
                    "Saved progress is unreadable and has been preserved. Import a valid backup to recover it. Details: {error}"
                )),
            ),
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => (Progress::default(), None),
        Err(error) => (
            Progress::default(),
            Some(format!(
                "Saved progress could not be opened and has not been changed. Import a valid backup to recover it. Details: {error}"
            )),
        ),
    }
}

pub fn save_progress(progress: &Progress) -> Result<(), String> {
    let path = progress_path();
    match fs::read(&path) {
        Ok(existing) if decode_saved_progress(&existing).is_err() => {
            return Err(
                "Saved progress is unreadable, so it was left untouched. Import a valid backup to recover it.".to_owned(),
            );
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(format!(
                "Could not read saved progress before saving: {error}"
            ));
        }
    }
    let parent = path
        .parent()
        .ok_or_else(|| "Could not find the application data folder".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| format!("Could not save progress: {error}"))?;

    let mut progress = progress.clone();
    progress.version = 2;
    write_json_atomic(&path, &progress)
}

pub fn write_json_atomic(path: &Path, value: &impl serde::Serialize) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .ok_or_else(|| "A complete save path is required.".to_owned())?;
    let filename = path
        .file_name()
        .ok_or_else(|| "A filename is required.".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("Could not create the save folder: {error}"))?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let temporary = parent.join(format!(
        ".{}.{}.{nonce}.tmp",
        filename.to_string_lossy(),
        std::process::id()
    ));
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("Could not encode the backup: {error}"))?;
    let result = (|| {
        let mut file = File::options()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| format!("Could not write the save file: {error}"))?;
        file.write_all(&bytes)
            .and_then(|_| file.sync_all())
            .map_err(|error| format!("Could not finish saving: {error}"))?;
        fs::rename(&temporary, path)
            .map_err(|error| format!("Could not replace the save file: {error}"))
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}

fn decode_learning(bytes: &[u8]) -> Result<LearningState, String> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    if !value
        .as_object()
        .is_some_and(|object| object.contains_key("version") && object.contains_key("cards"))
    {
        return Err("The learning file is missing its version or review deck.".to_owned());
    }
    let mut state: LearningState =
        serde_json::from_value(value).map_err(|error| error.to_string())?;
    if state.version != 1 {
        return Err("This learning file uses an unsupported version.".to_owned());
    }
    state.normalize();
    Ok(state)
}

pub fn load_learning() -> (LearningState, Option<String>) {
    match fs::read(data_dir().join("learning.json")) {
        Ok(bytes) => match decode_learning(&bytes) {
            Ok(state) => (state, None),
            Err(error) => (
                LearningState::default(),
                Some(format!(
                    "Your saved review deck and preferences are unreadable and have been preserved. Import a full app backup to recover them. {error}"
                )),
            ),
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            (LearningState::default(), None)
        }
        Err(error) => (
            LearningState::default(),
            Some(format!(
                "Could not read your review deck and preferences: {error}"
            )),
        ),
    }
}

pub fn save_learning(state: &LearningState) -> Result<(), String> {
    let path = data_dir().join("learning.json");
    match fs::read(&path) {
        Ok(bytes) => {
            decode_learning(&bytes).map_err(|_| "Unreadable learning data was left untouched. Import a full app backup to recover it.".to_owned())?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(format!(
                "Could not read learning data before saving: {error}"
            ));
        }
    }
    write_json_atomic(&path, state)
}

pub fn recover_learning(state: &LearningState) -> Result<(), String> {
    let path = data_dir().join("learning.json");
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let backup = path.with_extension(format!("{nonce}.recovery.json"));
    let existed = path.exists();
    if existed {
        fs::rename(&path, &backup)
            .map_err(|error| format!("Could not preserve previous learning data: {error}"))?;
    }
    if let Err(error) = save_learning(state) {
        if existed {
            let _ = fs::rename(&backup, &path);
        }
        return Err(error);
    }
    Ok(())
}

pub fn export_backup(
    path: &Path,
    progress: &Progress,
    learning: &LearningState,
) -> Result<(), String> {
    let mut value = serde_json::to_value(progress).map_err(|error| error.to_string())?;
    value["learning"] = serde_json::to_value(learning).map_err(|error| error.to_string())?;
    write_json_atomic(path, &value)
}

pub fn import_learning(path: &Path) -> Result<Option<LearningState>, String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    value
        .get("learning")
        .map(|learning| {
            decode_learning(&serde_json::to_vec(learning).map_err(|error| error.to_string())?)
        })
        .transpose()
}

fn decode_saved_progress(bytes: &[u8]) -> Result<Progress, String> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    let Some(object) = value.as_object() else {
        return Err("Saved progress must be a JSON object.".to_owned());
    };
    if !object.contains_key("completed") || !object.contains_key("scores") {
        return Err("Saved progress is missing its completion or score data.".to_owned());
    }
    let mut progress: Progress =
        serde_json::from_value(value).map_err(|error| error.to_string())?;
    if progress.version > 2 {
        return Err(format!(
            "Saved progress uses a newer version ({}) that this app cannot read safely (supported through version 2).",
            progress.version
        ));
    }
    progress.normalize();
    progress.version = 2;
    Ok(progress)
}

pub fn recover_progress(progress: &Progress) -> Result<Option<PathBuf>, String> {
    let path = progress_path();
    let backup = if path.exists() {
        let mut backup = path.with_extension("recovery.json");
        let mut suffix = 2;
        while backup.exists() {
            backup = path.with_extension(format!("recovery-{suffix}.json"));
            suffix += 1;
        }
        fs::rename(&path, &backup).map_err(|error| {
            format!("Could not preserve the previous progress file before recovery: {error}")
        })?;
        Some(backup)
    } else {
        None
    };

    if let Err(error) = save_progress(progress) {
        if let Some(backup) = &backup {
            let _ = fs::rename(backup, &path);
        }
        return Err(error);
    }
    Ok(backup)
}

pub fn import_progress(path: &PathBuf, course: &Course) -> Result<Progress, String> {
    let bytes = fs::read(path).map_err(|error| format!("Could not read that backup: {error}"))?;
    let mut imported = decode_saved_progress(&bytes)
        .map_err(|error| format!("This file is not a supported French course backup: {error}"))?;
    let known: HashSet<&str> = course.units.iter().map(|unit| unit.id.as_str()).collect();
    imported
        .completed
        .retain(|id, completed| known.contains(id.as_str()) && *completed);
    imported
        .scores
        .retain(|id, score| known.contains(id.as_str()) && *score <= 100);
    imported
        .completed_on
        .retain(|id, _| known.contains(id.as_str()) && imported.completed.contains_key(id));
    imported
        .quiz_history
        .retain(|attempt| known.contains(attempt.unit_id.as_str()));
    imported.normalize();
    imported.version = 2;
    Ok(imported)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning::ReviewCard;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct TestFolder(PathBuf);
    impl TestFolder {
        fn new() -> Self {
            static SEQUENCE: AtomicU64 = AtomicU64::new(0);
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = env::temp_dir().join(format!(
                "french-app-store-test-{}-{nonce}-{}",
                std::process::id(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for TestFolder {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn course() -> Course {
        serde_json::from_str(include_str!("../course/course.json")).unwrap()
    }

    #[test]
    fn full_backup_round_trips_both_progress_and_learning() {
        let folder = TestFolder::new();
        let path = folder.0.join("backup.json");
        let course = course();
        let unit = &course.units[0];
        let mut progress = Progress::default();
        progress.record_quiz(&unit.id, 90, true, Some("2026-09-30"));
        let mut learning = LearningState {
            last_unit: Some(unit.id.clone()),
            last_section: 3,
            text_scale: 1.2,
            slow_audio: true,
            daily_goal: 15,
            ..Default::default()
        };
        learning.cards.insert(
            crate::learning::word_key(&unit.id, &unit.vocab[0].fr),
            ReviewCard {
                level: 3,
                due_day: 21000,
                last_reviewed: 20993,
                reviews: 4,
            },
        );
        learning.reviews_by_day.insert("2026-09-30".into(), 4);
        export_backup(&path, &progress, &learning).unwrap();
        let imported_progress = import_progress(&path, &course).unwrap();
        let imported_learning = import_learning(&path).unwrap().unwrap();
        assert_eq!(
            serde_json::to_value(imported_progress).unwrap(),
            serde_json::to_value(progress).unwrap()
        );
        assert_eq!(
            serde_json::to_value(imported_learning).unwrap(),
            serde_json::to_value(learning).unwrap()
        );
    }

    #[test]
    fn legacy_progress_backup_imports_without_learning_and_filters_unknown_units() {
        let folder = TestFolder::new();
        let path = folder.0.join("legacy.json");
        let course = course();
        let id = &course.units[0].id;
        let legacy = serde_json::json!({"completed": {id: true, "unknown": true}, "scores": {id: 95, "unknown": 90}});
        write_json_atomic(&path, &legacy).unwrap();
        let imported = import_progress(&path, &course).unwrap();
        assert_eq!(imported.version, 2);
        assert!(imported.is_complete(id));
        assert_eq!(imported.scores[id], 95);
        assert!(!imported.completed.contains_key("unknown"));
        assert!(!imported.scores.contains_key("unknown"));
        assert!(import_learning(&path).unwrap().is_none());
    }

    #[test]
    fn atomic_replacement_cleans_temporary_files_on_success_and_failure() {
        let folder = TestFolder::new();
        let path = folder.0.join("save.json");
        write_json_atomic(&path, &serde_json::json!({"old": true})).unwrap();
        write_json_atomic(&path, &serde_json::json!({"new": true})).unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&fs::read(&path).unwrap()).unwrap(),
            serde_json::json!({"new": true})
        );
        assert_eq!(fs::read_dir(&folder.0).unwrap().count(), 1);
        let blocked = folder.0.join("directory.json");
        fs::create_dir(&blocked).unwrap();
        assert!(write_json_atomic(&blocked, &serde_json::json!({"new": true})).is_err());
        assert!(blocked.is_dir());
        assert_eq!(fs::read_dir(&folder.0).unwrap().count(), 2);
        assert!(write_json_atomic(Path::new(""), &0).is_err());
        assert!(write_json_atomic(Path::new("relative.json"), &0).is_err());
    }

    #[test]
    fn learning_decoder_rejects_future_invalid_and_incomplete_data() {
        for bytes in [
            br#"{"version":2,"cards":{}}"#.as_slice(),
            br#"{"version":1,"cards":[]}"#,
            br#"{"version":1}"#,
            b"not json",
        ] {
            assert!(decode_learning(bytes).is_err());
        }
    }

    #[test]
    fn unreadable_learning_is_preserved_and_recovery_keeps_previous_bytes() {
        // Isolate environment overrides in a child process, avoiding global mutation
        // while other Rust tests execute concurrently.
        if env::var_os("FRENCH_APP_TEST_LEARNING_CHILD").is_some() {
            let root = data_dir();
            assert_eq!(
                root,
                PathBuf::from(env::var_os("FRENCH_APP_DATA_DIR").unwrap())
            );
            let path = root.join("learning.json");
            for bytes in [
                br#"{"version":999,"cards":{}}"#.as_slice(),
                br#"{"version":1,"cards":[]}"#,
                b"not json",
            ] {
                fs::write(&path, bytes).unwrap();
                let (loaded, warning) = load_learning();
                assert!(warning.is_some());
                assert!(loaded.cards.is_empty());
                assert!(save_learning(&LearningState::default()).is_err());
                assert_eq!(fs::read(&path).unwrap(), bytes);
            }
            let original = fs::read(&path).unwrap();
            recover_learning(&LearningState::default()).unwrap();
            let (_, warning) = load_learning();
            assert!(warning.is_none());
            let backups: Vec<_> = fs::read_dir(&root)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path != &root.join("learning.json"))
                .collect();
            assert_eq!(backups.len(), 1);
            assert_eq!(fs::read(&backups[0]).unwrap(), original);
            return;
        }
        let folder = TestFolder::new();
        let output = Command::new(env::current_exe().unwrap())
            .args([
                "--exact",
                "store::tests::unreadable_learning_is_preserved_and_recovery_keeps_previous_bytes",
                "--nocapture",
            ])
            .env("FRENCH_APP_TEST_LEARNING_CHILD", "1")
            .env("FRENCH_APP_DATA_DIR", &folder.0)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "Child test failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
