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
){
    let token: HeaderMap;
    match login(config).await {
        Ok(val) => token = val,
        Err(e) => {
            log::error!("Failed to log in :( {}", e);
            return;
        }
    }
    if app.id.is_none() {
        match upsert_app(app, token, config).await {
            Ok(()) => log::info!("Added app successfully"),
            Err(e) => log::error!("Failed to create App : {}", e)
        }
    } else {
        match update_app(app, token, config).await {
            Ok(()) => log::info!("Updated app successfully"),
            Err(e) => log::error!("Failed to update App : {}", e)
        }
    }
}

pub async fn login(config: &Config) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let auth_url = format!(
        "{}/api/auth/password",
        config.shield_server
    );
    let mut params = HashMap::new();
    params.insert("identity", config.shield_user.clone());
    params.insert("password", config.shield_pass.clone());
    let response = reqwest::Client::new()
        .post(auth_url)
        .json(&params)
        .send()
        .await?;
    let auth: Value = response.json().await?;
    let token = auth.get("token").unwrap().as_str().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    Ok(headers)
}

pub async fn upsert_app(
    app: &Application,
    headers: HeaderMap,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/applications", config.shield_server);
    log::debug!("Creating App: {}", serde_json::to_string_pretty(app)?);
    let response = reqwest::Client::new()
        .put(url)
        .json(&app)
        .headers(headers)
        .send()
        .await?;
    let response_text = response.text().await?;
    log::debug!("Server Response: {}", &response_text);
    let result_app: Application = serde_json::from_str(&response_text)?;
    let path_name = format!("{}/app.json", RESULT_DIR);
    write_json_file(path::Path::new(&path_name), &result_app);
    Ok(())
}

pub async fn update_app(
    app: &Application,
    headers: HeaderMap,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/collections/applications/records/{}", config.shield_server, app.id.clone().unwrap());
    let response = reqwest::Client::new()
        .patch(url)
        .json(&app)
        .headers(headers)
        .send()
        .await?;
    let status = response.status();
    if status.is_success(){
        Ok(())
    }else {
        Err(Box::from(format!("Shield Update App call failed!! : {}", status.as_str())))
    }
}