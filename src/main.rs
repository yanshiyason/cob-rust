use std::env;
use std::fs;
use std::path::Path;
use toml;

mod async_types;
mod config;
mod git;
mod github;
mod remote_cob;
mod remote_fire_cob;

use config::Config;
use remote_cob::remote_cob;
use remote_fire_cob::remote_fire_cob;

static DEFAULT_CONFIG: &str = r#"prefix = ""

[github]
username = ""
password = ""
auth_token = ""
"#;

#[tokio::main]
async fn main() {
    app().await.unwrap()
}

async fn app() -> async_types::Result<()> {
    let home = env::var("HOME").unwrap();
    let file_name = format!("{}/.cobz", home);
    let config_file_path = Path::new(&file_name);

    if !config_file_path.exists() {
        match fs::write(config_file_path, DEFAULT_CONFIG) {
            Ok(_) => print!("created config file at: {}", config_file_path.display()),
            Err(err) => panic!(err),
        }
    }

    let contents =
        fs::read_to_string(config_file_path).expect("Something went wrong reading the file");
    let config: Config = toml::from_str(&contents).unwrap();

    let args: Vec<String> = env::args().collect();

    // validate presence of necessary config values

    if args.len() == 1 {
        remote_cob(&config).await.unwrap();
    } else if args.len() == 2 && args[1].chars().all(|x| x.is_numeric()) {
        let issue_id = String::from(args[1].as_str().trim());
        remote_fire_cob(issue_id, &config).await.unwrap();
    } else {
        let sentence = String::from(args[1..].join(" ").trim());
        git::checkout_branch(sentence);
    };

    return Ok(());
}
