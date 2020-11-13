use reqwest;
use serde_json;

use std::collections::HashMap;

use super::async_types;
use super::git;

mod issue;
use issue::Issue;

pub async fn list_issues(username: String, token: String) -> async_types::Result<Vec<Issue>> {
    let repository_owner = git::repository_owner();
    let repository = git::repository();
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues?page=#{}",
        repository_owner, repository, 1
    );

    assert!(
        username.trim().is_empty() == false,
        "username can't be empty"
    );
    assert!(token.trim().is_empty() == false, "token can't be empty");

    let req = reqwest::Client::new()
        .get(&url)
        .basic_auth(username, Some(token))
        .header("Content-Type", "application/json")
        .header("User-Agent", "cob")
        .build()
        .unwrap();

    let client = reqwest::Client::new();
    let response = client.execute(req).await?;
    let body = response.text().await.unwrap();

    // TODO error handling.. if the github api returns a {"message": "some msg"} payload..

    let json: Vec<Issue> = serde_json::from_str(&body).unwrap();
    return Ok(json);
}

pub async fn get_issue(username: String, token: String, id: String) -> async_types::Result<Issue> {
    let repository_owner = git::repository_owner();
    let repository = git::repository();
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}",
        repository_owner, repository, id
    );

    assert!(
        username.trim().is_empty() == false,
        "username can't be empty"
    );
    assert!(token.trim().is_empty() == false, "token can't be empty");

    let req = reqwest::Client::new()
        .get(&url)
        .basic_auth(username, Some(token))
        .header("Content-Type", "application/json")
        .header("User-Agent", "cob")
        .build()
        .unwrap();

    let client = reqwest::Client::new();
    let response = client.execute(req).await?;
    let body = response.text().await.unwrap();
    let map: HashMap<String, serde_json::Value> = serde_json::from_str(&body).unwrap();
    if map["message"].as_str().unwrap().contains("Not Found") {
        println!(
            "Message from github: {} for issue number {}",
            map["message"], id
        );
        std::process::exit(1);
    }
    let json: Issue = serde_json::from_str(&body).unwrap();
    return Ok(json);
}
