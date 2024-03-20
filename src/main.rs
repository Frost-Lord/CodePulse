use reqwest;
use serde_json::Value;
use tokio;
use tokio::time::{self, Duration};
use crate::settings::SETTINGS;
use crate::logs::log_settings;

mod settings;
mod update;
mod logs;

fn is_git_installed() -> bool {
    std::process::Command::new("git")
        .arg("--version")
        .output()
        .is_ok()
}

#[tokio::main]
async fn main() {
    if !is_git_installed() {
        println!("Git is not installed. Please install Git to continue.");
        return;
    }

    log_settings(true, "Settings loaded successfully");

    let settings_lock = SETTINGS.lock().unwrap();
    let interval_seconds = settings_lock.settings.as_ref().unwrap().intivial;
    let colors = settings_lock.colors.clone();
    drop(settings_lock);

    let mut interval = time::interval(Duration::from_secs(interval_seconds as u64));

    loop {
        println!("{}[CodePulse]{} Sleeping for {} seconds...", colors.cyan_green, colors.endc, interval_seconds);
        interval.tick().await;

        let settings_lock = SETTINGS.lock().unwrap();
        let projects = settings_lock.projects.clone();
        drop(settings_lock);

        println!("{}[CodePulse]{} Checking for updates...", colors.bold, colors.endc);

        let client = reqwest::Client::builder()
            .user_agent("CodePulse")
            .build()
            .unwrap();

        let tasks = projects.into_iter().map(|project| {
            let client_clone = client.clone();
            let colors_clone = colors.clone();
            tokio::spawn(async move {
                const BASE_URL: &str = "https://api.github.com/repos/";
                println!("{}[CodePulse]{} {}[EVENT]{} Spawned {:?}", colors_clone.cyan_green, colors_clone.endc, colors_clone.cyan, colors_clone.endc, std::thread::current().id());
                match client_clone.get(BASE_URL.to_owned() + &project.github_url).send().await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            match resp.json::<Value>().await {
                                Ok(data) => update::check_update(&project, &data),
                                Err(e) => println!("{}[CodePulse]{} Failed to parse response for {}: {}", colors_clone.blue, colors_clone.endc, project.name, e),
                            }
                        } else {
                            let status = resp.status();
                            let body = resp.text().await.unwrap_or_else(|_| "Failed to read body".to_string());
                            println!("{}[CodePulse]{} Request to {} failed: Status {}, Body {}", colors_clone.warning, colors_clone.endc, project.github_url, status, body);
                        }
                    },
                    Err(e) => println!("{}[CodePulse]{} Failed to fetch {}: {}", colors_clone.fail, colors_clone.endc, project.github_url, e),
                }
            })
        }).collect::<Vec<_>>();
        futures::future::join_all(tasks).await;
    }
}
