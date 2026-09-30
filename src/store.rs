use crate::model::{Course, Progress};
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

pub fn data_dir() -> PathBuf {
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

    let temporary = path.with_extension("json.tmp");
    let mut progress = progress.clone();
    progress.version = 2;
    let bytes = serde_json::to_vec_pretty(&progress)
        .map_err(|error| format!("Could not encode progress: {error}"))?;
    let mut file =
        File::create(&temporary).map_err(|error| format!("Could not write progress: {error}"))?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|error| format!("Could not finish saving progress: {error}"))?;
    fs::rename(&temporary, &path).map_err(|error| {
        let _ = fs::remove_file(&temporary);
        format!("Could not replace saved progress: {error}")
    })
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
