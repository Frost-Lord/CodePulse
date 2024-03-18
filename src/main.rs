use reqwest;
use serde_json::Value;

mod settings;
mod update;
use crate::settings::SETTINGS;

#[tokio::main]
async fn main() {
    let colors = &SETTINGS.colors;

    let client = reqwest::Client::builder()
        .user_agent("CodePulse")
        .build()
        .unwrap();

    let projects = &SETTINGS.projects;

    println!("{}[CodePulse]{} Checking for updates...", colors.bold, colors.endc);

    for project in projects {
        const BASE_URL: &str = "https://api.github.com/repos/";
        match client.get(BASE_URL.to_owned() + &project.github_url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<Value>().await {
                        Ok(data) => update::check_update(project, &data),
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
    }
}