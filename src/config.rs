use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub prefix: String,
    pub github: GithubConfig,
}

#[derive(Deserialize, Clone)]
pub struct GithubConfig {
    pub username: String,
    pub password: String,
    pub auth_token: String,
}
