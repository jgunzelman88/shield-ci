use std::process::Command;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static!{
  static ref REMOVE_REMOTE: Regex = Regex::new(r"^.+?[/]").unwrap();
}

pub fn get_branch() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git").arg("branch").arg("--show-current").output()?;
    let stdout = String::from(String::from_utf8(output.stdout)?.trim());
    Ok(stdout)
}

pub fn get_branches() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut branches: Vec<String> = Vec::new();
    let output = Command::new("git").arg("branch").arg("-r").output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let tokens: Vec<&str> = stdout.split("\n").into_iter().collect();
    for token in tokens {
      let norm = REMOVE_REMOTE.replace(token, "");
      if norm != "" {
        branches.push(String::from(norm));
      }
    }
    Ok(branches)
}
