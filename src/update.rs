use serde_json::Value;
use crate::settings::{SETTINGS, Project};
use chrono::{DateTime, Utc};

pub fn check_update(project: &Project, data: &Value) {
    let colors = &SETTINGS.colors;

    let local_update_str = &project.updated_at;
    let local_update = DateTime::parse_from_rfc3339(local_update_str)
        .expect("Failed to parse local updated_at date")
        .with_timezone(&Utc);

    //println!("{}Project Data: {}{}", colors.blue, data, colors.endc);

    if let Some(github_update_str) = data["updated_at"].as_str() {
        let github_update = DateTime::parse_from_rfc3339(github_update_str)
            .expect("Failed to parse GitHub updated_at date")
            .with_timezone(&Utc);

        if github_update > local_update {
            println!("{}[EVENT]{} {}'s Github is newer, updating...", colors.cyan, colors.endc, project.name);
            update_dir();
        } else {
            println!("{}[EVENT]{} {}'s local code is up-to-date.", colors.cyan, colors.endc, project.name);
        }
    } else {
        println!("{}Could not find or parse GitHub updated_at date.{}", colors.fail, colors.endc);
    }
}

fn update_dir() {
    let colors = &SETTINGS.colors;
    println!("{}UpdateDir{}", colors.bold, colors.endc);
}
