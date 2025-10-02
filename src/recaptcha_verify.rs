use std::collections::HashMap;

use serde_json::Value;
use serde_json::json;

use crate::config::CONFIG;

/// Send an API request to google to validate a recaptcha token
pub async fn recaptcha_verify(token: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if token == "" {
        return Ok(false);
    }

    let body = json!({
        "event": {
            "token": token,
            "expectedAction": "invite",
            "siteKey": CONFIG.google.recaptcha_site_key,
        }
    });

    // TODO: Is this the best way to add a query parameter?
    let mut query: HashMap<String, String> = HashMap::new();
    query.insert("key".to_string(), CONFIG.google.google_api_key.clone());

    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "https://recaptchaenterprise.googleapis.com/v1/projects/{}/assessments",
            CONFIG.google.google_cloud_project_name
        ))
        .body(serde_json::to_string(&body)?)
        .query(&query)
        .send()
        .await?;

    // This consumes the response for some reason
    let response_text = response.text().await?;

    println!("Verification response text: {}", response_text);

    // Parse the json response
    let data: Value = serde_json::from_str(&response_text)?;

    Ok(data["tokenProperties"]["valid"].as_bool().unwrap())
}
