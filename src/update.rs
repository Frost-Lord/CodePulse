use serde_json::Value;
use crate::settings::{SETTINGS, Project};
use chrono::{DateTime, Utc};
use std::path::Path;
use std::fs;

pub fn logger(event: bool, thread_id: std::thread::ThreadId, message: &str) {
    let _colors = &SETTINGS.colors;
    if event {
        println!("{}[CodePulse]{} {}[EVENT]{} {}[{:?}]{} {}", _colors.cyan_green, _colors.endc, _colors.cyan, _colors.endc, _colors.blue, thread_id, _colors.endc, message);
    } else {
        println!("{}[CodePulse]{} {}[ERROR]{} {}[{:?}]{} {}", _colors.fail, _colors.endc, _colors.cyan, _colors.endc, _colors.blue, thread_id, _colors.endc, message);
    }
}

pub fn check_update(project: &Project, data: &Value) {
    let _colors = &SETTINGS.colors;
    let thread_id = std::thread::current().id();

    let local_update_str = &project.updated_at;
    let local_update = DateTime::parse_from_rfc3339(local_update_str)
        .expect("Failed to parse local updated_at date")
        .with_timezone(&Utc);

    if let Some(github_update_str) = data["updated_at"].as_str() {
        let github_update = DateTime::parse_from_rfc3339(github_update_str)
            .expect("Failed to parse GitHub updated_at date")
            .with_timezone(&Utc);

        if github_update > local_update {
            logger(true, thread_id, &format!("{}'s Github is newer, updating...", project.name));
            update_dir(project, true);
        } else {
            logger(true, thread_id, &format!("{}'s local code is up-to-date.", project.name));
        }
    } else {
        logger(false, thread_id, "Could not find or parse GitHub updated_at date.");
    }
}

fn update_dir(project: &Project, _should_clone: bool) {
    let _colors = &SETTINGS.colors;
    let git_url = "https://github.com/".to_owned() + &project.github_url + ".git";
    let thread_id = std::thread::current().id();

    let dir = &project.path;

    if !Path::new(dir).exists() {
        logger(true, thread_id, &format!("Directory {} does not exist, creating and cloning...", dir));
        fs::create_dir_all(dir).expect("Failed to create directory");

        let output = std::process::Command::new("git")
            .arg("clone")
            .arg(git_url)
            .arg(dir)
            .output()
            .expect("Failed to execute git clone");

        if output.status.success() {
            logger(true, thread_id, &format!("Successfully cloned into {}", dir));
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            logger(false, thread_id, &format!("Failed to clone: {}", stderr));
            return;
        }
    }

    let output = std::process::Command::new("git")
        .arg("pull")
        .current_dir(dir)
        .output()
        .expect("Failed to execute git pull");

    if output.status.success() {
        logger(true, thread_id, &format!("Successfully updated {}", dir));
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        logger(false, thread_id, &format!("Failed to update {}: {}", dir, stderr));
    }
}
