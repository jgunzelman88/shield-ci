use crate::models::application::Application;
use crate::models::config::{Config, RESULT_DIR};
use crate::models::dependecy_report::DependencyReport;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use std::collections::HashMap;
use std::path;

use super::shared::write_json_file;

/**
 * Submit Results both update applicaiton and add a new report.
 * # Arguments
 *    * app: application to submit
 *    * report: report to submit
 *    * config: configuration
 */
pub async fn submit_results(app: &Application, report: &DependencyReport, config: &Config) {
    let token: HeaderMap;
    match login(config).await {
        Ok(val) => token = val,
        Err(e) => {
            log::error!("Failed to log in :( {}", e);
            return;
        }
    }
    let app_id: String;
    match upsert_app(app, &token, config).await {
        Ok(val) => {
            log::info!("Added app successfully");
            app_id = val;
        }
        Err(e) => {
            log::error!("Failed to upsert App : {}", e);
            return;
        }
    }
    let mut appended_report = report.clone();
    appended_report.application_id = Some(app_id);
    appended_report.vulnerabilities = report.vulnerabilities.clone();
    match push_report(&appended_report, &token, config).await {
        Ok(_) => {
            log::info!("Added app successfully");
        }
        Err(e) => {
            log::error!("Failed to Push Report : {}", e);
        }
    }
}

/**
 * Login to shield hub
 */
pub async fn login(config: &Config) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let auth_url = format!("{}/api/auth/password", config.shield_server);
    let mut params = HashMap::new();
    params.insert("identity", config.shield_user.clone());
    params.insert("password", config.shield_pass.clone());
    let response = reqwest::Client::new()
        .post(auth_url)
        .json(&params)
        .send()
        .await?;
    let auth_text = response.text().await?;
    let auth: Value = serde_json::from_str(&auth_text)?;
    let valid_token = auth.get("token");
    if valid_token.is_none() {
        let msg = String::from(&auth_text);
        return Err(Box::from(msg));
    }
    let token = valid_token.unwrap().as_str().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );
    Ok(headers)
}

/**
 * Upsert application to shield hub
 * */
pub async fn upsert_app(
    app: &Application,
    headers: &HeaderMap,
    config: &Config,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("{}/api/applications", config.shield_server);
    log::debug!("Upserting App: {}", serde_json::to_string_pretty(app)?);
    let response = reqwest::Client::new()
        .put(url)
        .json(&app)
        .headers(headers.clone())
        .send()
        .await?;
    let response_text = response.text().await?;
    log::debug!("Server Response: {}", &response_text);
    let result_app: Application = serde_json::from_str(&response_text)?;
    let path_name = format!("{}/{}/app.json", config.base_dir, RESULT_DIR);
    write_json_file(path::Path::new(&path_name), &result_app);
    Ok(result_app.id.clone().unwrap_or_default())
}

pub async fn push_report(
    report: &DependencyReport,
    headers: &HeaderMap,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/api/applications/reports", config.shield_server);
    log::debug!("Pushing Report: {}", serde_json::to_string_pretty(report)?);
    let response = reqwest::Client::new()
        .put(url)
        .json(&report)
        .headers(headers.clone())
        .send()
        .await?;
    let status = response.status();
    log::debug!("Server Response: {}", &status);
    if status.is_success() {
        Ok(())
    } else {
        let text = format!("Failed to Push Report: {}", status.as_str());
        Err(Box::from(text))
    }
}
