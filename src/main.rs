use reqwest;
use serde_json::Value;
use tokio;
use tokio::time::{self, Duration};

mod settings;
mod update;
mod logs;
use crate::settings::SETTINGS;

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

    let interval_seconds = SETTINGS.settings.as_ref().unwrap().intivial;
    let mut interval = time::interval(Duration::from_secs(interval_seconds as u64));

    loop {
        interval.tick().await;

        let colors = &SETTINGS.colors.clone();
        let client = reqwest::Client::builder()
            .user_agent("CodePulse")
            .build()
            .unwrap();
        let projects = SETTINGS.projects.clone();

        println!("{}[CodePulse]{} Checking for updates...", colors.bold, colors.endc);

        let tasks = projects.into_iter().map(|project| {
            let client = client.clone();
            let colors = colors.clone();
            tokio::spawn(async move {
                const BASE_URL: &str = "https://api.github.com/repos/";
                println!("{}[CodePulse]{} {}[EVENT]{} Spawned {:?}", colors.cyan_green, colors.endc, colors.cyan, colors.endc, std::thread::current().id());
                match client.get(BASE_URL.to_owned() + &project.github_url).send().await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            match resp.json::<Value>().await {
                                Ok(data) => update::check_update(&project, &data),
                                Err(e) => println!("{}[CodePulse]{} Failed to parse response for {}: {}", colors.blue, colors.endc, project.name, e),
                            }
                        } else {
                            let status = resp.status();
                            let body = resp.text().await.unwrap_or_else(|_| "Failed to read body".to_string());
                            println!("{}[CodePulse]{} Request to {} failed: Status {}, Body {}", colors.warning, colors.endc, project.github_url, status, body);
                        }
                    },
                    Err(e) => println!("{}[CodePulse]{} Failed to fetch {}: {}", colors.fail, colors.endc, project.github_url, e),
                }
            })
        }).collect::<Vec<_>>();
        futures::future::join_all(tasks).await;
    }
}
