use super::async_types;
use super::config::Config;
use super::git;
use super::github;

pub async fn remote_fire_cob(id: String, config: &Config) -> async_types::Result<()> {
    let github = config.github.clone();
    let username = github.username;
    let token = github.auth_token;
    let issue = github::get_issue(username, token, id).await.unwrap();
    git::checkout_branch(issue.title.clone());
    return Ok(());
}
