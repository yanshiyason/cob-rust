use dialoguer::{theme::ColorfulTheme, Select};

use super::async_types;
use super::config::Config;
use super::git;
use super::github;

pub async fn remote_cob(config: &Config) -> async_types::Result<()> {
    let github = config.github.clone();
    let username = github.username;
    let token = github.auth_token;
    let issues = github::list_issues(username, token).await.unwrap();

    let selections: Vec<String> = issues
        .iter()
        .map(|i| format!("{}: {}", i.number, i.title.clone()))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select issue to create branch from")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        let issue = issues.get(selection).unwrap();
        git::checkout_branch(issue.title.clone());
    } else {
        std::process::exit(0);
    }

    return Ok(());
}
