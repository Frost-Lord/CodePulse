use serde_json::Value;
use crate::settings::{SETTINGS, Project, update_project_updated_at};
use chrono::{DateTime, Utc};
use std::path::Path;
use std::fs;
use crate::logs;

pub fn check_update(project: &Project, data: &Value) {
    let settings = SETTINGS.lock().unwrap();
    let _colors = &settings.colors;
    drop(settings);
    let thread_id = std::thread::current().id();

    match DateTime::parse_from_rfc3339(&project.updated_at) {
        Ok(local_update) => {
            let local_update = local_update.with_timezone(&Utc);

            if let Some(github_update_str) = data["updated_at"].as_str() {
                match DateTime::parse_from_rfc3339(github_update_str) {
                    Ok(github_update) => {
                        let github_update = github_update.with_timezone(&Utc);

                        if github_update > local_update {
                            logs::logger(true, thread_id, &format!("{}'s Github is newer, updating...", project.name));
                            update_dir(project, true, data);
                        } else {
                            logs::logger(true, thread_id, &format!("{}'s local code is up-to-date.", project.name));
                        }
                    },
                    Err(_) => logs::logger(false, thread_id, "Failed to parse GitHub updated_at date."),
                }
            } else {
                logs::logger(false, thread_id, "Could not find or parse GitHub updated_at date.");
            }
        },
        Err(_) => {
            logs::logger(false, thread_id, "Failed to parse local updated_at date. Attempting to update based on remote data.");
            if let Some(github_update_str) = data["updated_at"].as_str() {
                update_dir(project, true, data);
                update_project_updated_at(project.name.clone(), github_update_str.to_string());
                logs::logger(true, thread_id, &format!("Successfully updated {} to the latest version based on GitHub's updated_at.", project.name));
            }
        }
    }
}

fn update_dir(project: &Project, _should_clone: bool, data: &Value) {
    let settings_lock = SETTINGS.lock().unwrap();
    let _colors = settings_lock.colors.clone();
    drop(settings_lock);
    let git_url = "https://github.com/".to_owned() + &project.github_url + ".git";
    let thread_id = std::thread::current().id();

    let dir = &project.path;

    if !Path::new(dir).exists() {
        logs::logger(true, thread_id, &format!("Directory {} does not exist, creating and cloning...", dir));
        fs::create_dir_all(dir).expect("Failed to create directory");

        let output = std::process::Command::new("git")
            .arg("clone")
            .arg(git_url)
            .arg(dir)
            .output()
            .expect("Failed to execute git clone");

        if output.status.success() {
            logs::logger(true, thread_id, &format!("Successfully cloned into {}", dir));
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            logs::logger(false, thread_id, &format!("Failed to clone: {}", stderr));
            return;
        }
    }

    let output = std::process::Command::new("git")
        .arg("pull")
        .current_dir(dir)
        .output()
        .expect("Failed to execute git pull");

    if output.status.success() {
        if let Some(github_update_str) = data["updated_at"].as_str() {
        update_project_updated_at(project.name.clone(), github_update_str.to_string());
        logs::logger(true, thread_id, &format!("Successfully updated {}", dir));
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        logs::logger(false, thread_id, &format!("Failed to update {}: {}", dir, stderr));
    }
}