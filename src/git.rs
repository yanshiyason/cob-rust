use regex::Regex;
use std::process::Command;

pub fn repository_owner() -> String {
    return String::from(owner_and_repository().get(0).unwrap());
}
pub fn repository() -> String {
    return String::from(owner_and_repository().get(1).unwrap());
}

// todo: allow a configurable suffix to be applied to branch names
// todo: allow a downcase to be configurable
pub fn checkout_branch(text: String) {
    let lowercase = text.to_lowercase();
    let re = regex::Regex::new(r"[^a-zA-Z0-9#]+").unwrap();
    let branch_name = String::from(re.replace_all(&lowercase, "_"));
    let out = Command::new("git")
        .args(&["checkout", "-b", &branch_name])
        .output();

    match out {
        Ok(out) => {
            let stderr = String::from(std::str::from_utf8(&out.stderr).unwrap());

            // For some reason, output is in stderr, not stdout..
            if stderr.matches("Switched to a new branch").count() > 0 {
                println!("Switched to a new branch: '{}'", &branch_name);
                return;
            }

            if stderr.matches("fatal").count() > 0
                && stderr.matches("A branch named").count() > 0
                && stderr.matches("already exists").count() > 0
            {
                Command::new("git")
                    .args(&["checkout", &branch_name])
                    .output()
                    .unwrap();
                return;
            }

            if !stderr.trim().is_empty() {
                println!("uhh ohh: '{}'", stderr);
                return;
            }
        }
        Err(err) => println!("Failed branch checkout: {}", err),
    }
}

// priv
fn owner_and_repository() -> Vec<String> {
    match remote_origin() {
        Some(val) => {
            let re = Regex::new(r"git@github.com.*?:(.*)?/(.*)\.git$").unwrap();
            match re.captures(&val) {
                Some(captures) => {
                    let host = captures
                        .get(1)
                        .map_or(String::from(""), |v| String::from(v.as_str()));
                    let repository = captures
                        .get(2)
                        .map_or(String::from(""), |v| String::from(v.as_str()));

                    return vec![host, repository];
                }
                None => {
                    println!("remote repository is not hosted on github.com");
                    std::process::exit(1);
                }
            }
        }
        None => {
            println!("remote repository is not set, please add it using the \"git add remote origin\" command");
            std::process::exit(1);
        }
    }
}
fn remote_origin() -> Option<String> {
    match Command::new("git").args(&["config", "--list"]).output() {
        Ok(out) => {
            let git_config_output = std::str::from_utf8(&out.stdout).unwrap();
            let remote_origin_url = git_config_output
                .split("\n")
                .find(|x| x.starts_with("remote.origin.url"));
            match remote_origin_url {
                Some(value) => {
                    // todo: validate format
                    let origin = value.split("=").last().unwrap();
                    return Some(String::from(origin));
                }
                None => {
                    println!("Not remote setup!");
                    return None;
                }
            }
        }
        Err(err) => {
            println!(
                "error running following command: `git config --list`. err: {}",
                err
            );
            std::process::exit(1);
        }
    }
}
