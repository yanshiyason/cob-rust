/// Generated by https://quicktype.io
///
/// To change quicktype's target language, run command:
///
///   "Set quicktype target language"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Issue {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "repository_url")]
    pub repository_url: String,

    #[serde(rename = "labels_url")]
    pub labels_url: String,

    #[serde(rename = "comments_url")]
    pub comments_url: String,

    #[serde(rename = "events_url")]
    pub events_url: String,

    #[serde(rename = "html_url")]
    pub html_url: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "node_id")]
    pub node_id: String,

    #[serde(rename = "number")]
    pub number: i64,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "labels")]
    pub labels: Vec<Option<serde_json::Value>>,

    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "locked")]
    pub locked: bool,

    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,

    #[serde(rename = "assignees")]
    pub assignees: Vec<Option<serde_json::Value>>,

    #[serde(rename = "milestone")]
    pub milestone: Option<serde_json::Value>,

    #[serde(rename = "comments")]
    pub comments: i64,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "closed_at")]
    pub closed_at: Option<serde_json::Value>,

    #[serde(rename = "author_association")]
    pub author_association: String,

    #[serde(rename = "active_lock_reason")]
    pub active_lock_reason: Option<serde_json::Value>,

    #[serde(rename = "body")]
    pub body: String,

    #[serde(rename = "performed_via_github_app")]
    pub performed_via_github_app: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "login")]
    pub login: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "node_id")]
    pub node_id: String,

    #[serde(rename = "avatar_url")]
    pub avatar_url: String,

    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "html_url")]
    pub html_url: String,

    #[serde(rename = "followers_url")]
    pub followers_url: String,

    #[serde(rename = "following_url")]
    pub following_url: String,

    #[serde(rename = "gists_url")]
    pub gists_url: String,

    #[serde(rename = "starred_url")]
    pub starred_url: String,

    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,

    #[serde(rename = "organizations_url")]
    pub organizations_url: String,

    #[serde(rename = "repos_url")]
    pub repos_url: String,

    #[serde(rename = "events_url")]
    pub events_url: String,

    #[serde(rename = "received_events_url")]
    pub received_events_url: String,

    #[serde(rename = "type")]
    pub user_type: String,

    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}
