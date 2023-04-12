use crate::models::application::Application;
use crate::models::config::{Config, RESULT_DIR};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use std::collections::HashMap;
use std::path;

use super::shared::write_json_file;

pub async fn submit_results(
    app: &Application,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let token = login(config).await?;
    if app.id.is_none() {
        create_app(app, token, config).await?;
    } else {
        update_app(app, token, config).await?;
    }
    Ok(())
}

pub async fn login(config: &Config) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let auth_url = format!(
        "{}/api/collections/users/auth-with-password",
        config.pb_server
    );
    let mut params = HashMap::new();
    params.insert("identity", config.pb_user.clone());
    params.insert("password", config.pb_pass.clone());
    let response = reqwest::Client::new()
        .post(auth_url)
        .form(&params)
        .send()
        .await?;
    let auth: Value = response.json().await?;
    let token = auth.get("token").unwrap().as_str().unwrap();
    let bearer = format!("bearer {}", token);
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&bearer).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    Ok(headers)
}

pub async fn create_app(
    app: &Application,
    headers: HeaderMap,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/collections/applications/records", config.pb_server);
    let response = reqwest::Client::new()
        .post(url)
        .json(&app)
        .headers(headers)
        .send()
        .await?;
    let result_app: Application = response.json().await?;
    let path_name = format!("{}/app.json", RESULT_DIR);
    write_json_file(path::Path::new(&path_name), &result_app);
    Ok(())
}

pub async fn update_app(
    app: &Application,
    headers: HeaderMap,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/collections/applications/records/{}", config.pb_server, app.id.clone().unwrap());
    reqwest::Client::new()
        .patch(url)
        .json(&app)
        .headers(headers)
        .send()
        .await?;
    Ok(())
}