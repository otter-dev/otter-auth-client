use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::{Path, PathBuf};

use crate::error::Error;

pub mod error;

pub type Result<T> = std::result::Result<T, Error>;

pub async fn get_github_auth_code(client_id: &str) -> Result<GithubAuthConfig> {
    reqwest::Client::new()
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .json(&json!({
            "client_id": client_id,
            "scope": "user:email"
        }))
        .send()
        .await?
        .json::<GithubAuthConfig>()
        .await
        .map_err(Error::ReqwestError)
}

fn get_config_dir() -> Result<PathBuf> {
    Ok(PathBuf::from_iter(&[
        dirs::home_dir()
            .ok_or(Error::MissingHomeDirectory)?
            .as_path(),
        Path::new(".config/osec"),
    ]))
}

pub fn save_config(auth_config: &GithubAuthConfig) -> Result<()> {
    let config_dir = get_config_dir()?;
    std::fs::create_dir_all(&config_dir)?;
    let auth_json = serde_json::to_string(auth_config)?;
    let file_path = config_dir.join("auth.json");
    std::fs::write(file_path, auth_json)?;
    println!("Auth config saved to {:?}", &config_dir);
    Ok(())
}

pub fn get_config() -> Result<GithubAuthConfig> {
    let config_path = get_config_dir()?.join("auth.json");
    let config = std::fs::read(config_path)?;
    serde_json::from_slice(&config).map_err(Error::JsonError)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubAuthConfig {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}
