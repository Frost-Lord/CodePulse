use serde_json::Value;
use crate::settings::{SETTINGS, Project};
use chrono::{DateTime, Utc};
use std::path::Path;
use std::fs;

pub fn check_update(project: &Project, data: &Value) {
    let colors = &SETTINGS.colors;

    let local_update_str = &project.updated_at;
    let local_update = DateTime::parse_from_rfc3339(local_update_str)
        .expect("Failed to parse local updated_at date")
        .with_timezone(&Utc);

    if let Some(github_update_str) = data["updated_at"].as_str() {
        let github_update = DateTime::parse_from_rfc3339(github_update_str)
            .expect("Failed to parse GitHub updated_at date")
            .with_timezone(&Utc);

        if github_update > local_update {
            println!("{}[EVENT]{} {}'s Github is newer, updating...", colors.cyan, colors.endc, project.name);
            update_dir(project, true);
        } else {
            println!("{}[EVENT]{} {}'s local code is up-to-date.", colors.cyan, colors.endc, project.name);
        }
    } else {
        println!("{}Could not find or parse GitHub updated_at date.{}", colors.fail, colors.endc);
    }
}

fn update_dir(project: &Project, _should_clone: bool) {
    let colors = &SETTINGS.colors;
    println!("{}UpdateDir{}", colors.bold, colors.endc);

    let git_url = "https://github.com/".to_owned() + &project.github_url + ".git";

    let dir = &project.path;

    if !Path::new(dir).exists() {
        println!("{}[EVENT]{} Directory does not exist, creating and cloning...", colors.cyan, colors.endc);
        fs::create_dir_all(dir).expect("Failed to create directory");

        let output = std::process::Command::new("git")
            .arg("clone")
            .arg(git_url)
            .arg(dir)
            .output()
            .expect("Failed to execute git clone");

        if output.status.success() {
            println!("{}[EVENT]{} Successfully cloned into {}", colors.cyan, colors.endc, dir);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{}[EVENT]{} Failed to clone: {}", colors.fail, colors.endc, stderr);
            return;
        }
    }

    println!("{}[EVENT]{} Updating {}...", colors.cyan, colors.endc, dir);

    let output = std::process::Command::new("git")
        .arg("pull")
        .current_dir(dir)
        .output()
        .expect("Failed to execute git pull");

    if output.status.success() {
        println!("{}[EVENT]{} Successfully updated {}", colors.cyan, colors.endc, dir);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("{}[EVENT]{} Failed to update {}: {}", colors.fail, colors.endc, dir, stderr);
    }
}
