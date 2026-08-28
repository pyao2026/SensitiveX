use std::time::{Duration, Instant};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Method,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiRequest {
    method: String,
    url: String,
    headers: Vec<HeaderEntry>,
    body: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct HeaderEntry {
    name: String,
    value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    status: u16,
    status_text: String,
    elapsed_ms: u128,
    headers: Vec<HeaderEntry>,
    mime: String,
    body: String,
}

#[tauri::command]
fn request_api(request: ApiRequest) -> Result<ApiResponse, String> {
    let method = match request.method.as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        _ => return Err("Only GET and POST requests are supported.".into()),
    };

    let url = reqwest::Url::parse(&request.url)
        .map_err(|error| format!("Invalid request URL: {error}"))?;
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err("Only HTTP and HTTPS URLs are supported.".into());
    }

    let mut headers = HeaderMap::new();
    for header in request.headers {
        let name = HeaderName::from_bytes(header.name.trim().as_bytes())
            .map_err(|error| format!("Invalid header name '{}': {error}", header.name))?;
        let value = HeaderValue::from_str(&header.value)
            .map_err(|error| format!("Invalid value for header '{}': {error}", header.name))?;
        headers.append(name, value);
    }

    let body = if method == Method::POST {
        let body = request.body.unwrap_or_default();
        if !body.trim().is_empty() {
            serde_json::from_str::<serde_json::Value>(&body)
                .map_err(|error| format!("Request body must be valid JSON: {error}"))?;
        }
        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        Some(body)
    } else {
        None
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|error| format!("Unable to create HTTP client: {error}"))?;
    let started_at = Instant::now();
    let mut builder = client.request(method, url).headers(headers);
    if let Some(body) = body {
        builder = builder.body(body);
    }

    let response = builder
        .send()
        .map_err(|error| format!("Request failed: {error}"))?;
    let status = response.status();
    let mime = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let response_headers = response
        .headers()
        .iter()
        .map(|(name, value)| HeaderEntry {
            name: name.to_string(),
            value: value
                .to_str()
                .unwrap_or("<non-UTF-8 header value>")
                .to_string(),
        })
        .collect();
    let body = response
        .text()
        .map_err(|error| format!("Unable to read response body: {error}"))?;

    Ok(ApiResponse {
        status: status.as_u16(),
        status_text: status.canonical_reason().unwrap_or("Unknown").to_string(),
        elapsed_ms: started_at.elapsed().as_millis(),
        headers: response_headers,
        mime,
        body,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![request_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
