use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Method,
};
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use tauri::Manager;

mod request_entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "saved_requests")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub name: String,
        pub method: String,
        pub url: String,
        pub headers: String,
        pub body: Option<String>,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

use request_entity::{
    ActiveModel as SavedRequestActive, Entity as SavedRequest, Model as SavedRequestModel,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiRequest {
    method: String,
    url: String,
    headers: Vec<HeaderEntry>,
    body: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveRequestInput {
    id: i64,
    name: String,
    request: ApiRequest,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SavedRequestDto {
    id: i64,
    name: String,
    method: String,
    url: String,
    headers: Vec<HeaderEntry>,
    body: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SavedRequestSummary {
    id: i64,
    name: String,
    method: String,
    url: String,
    updated_at: String,
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

fn database_path() -> Result<PathBuf, String> {
    if cfg!(debug_assertions) {
        std::env::current_dir()
            .map(|path| path.join("data.db"))
            .map_err(|error| format!("Unable to find working directory: {error}"))
    } else {
        let mut path = dirs::home_dir().ok_or("Unable to find the user home directory")?;
        path.push(".sensitivex");
        std::fs::create_dir_all(&path)
            .map_err(|error| format!("Unable to create database directory: {error}"))?;
        Ok(path.join("data.db"))
    }
}

async fn initialize_database() -> Result<DatabaseConnection, String> {
    let path = database_path()?;
    let path = path.to_string_lossy().replace('\\', "/");
    let url = if cfg!(windows) {
        format!("sqlite:///{path}?mode=rwc")
    } else {
        format!("sqlite://{path}?mode=rwc")
    };
    let database = Database::connect(url)
        .await
        .map_err(|error| format!("Unable to open database: {error}"))?;
    sea_orm::ConnectionTrait::execute_unprepared(
        &database,
        "CREATE TABLE IF NOT EXISTS saved_requests (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, method TEXT NOT NULL, url TEXT NOT NULL, headers TEXT NOT NULL, body TEXT, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)",
    )
    .await
    .map_err(|error| format!("Unable to initialize database: {error}"))?;
    Ok(database)
}

fn dto_from_model(model: SavedRequestModel) -> Result<SavedRequestDto, String> {
    Ok(SavedRequestDto {
        id: model.id,
        name: model.name,
        method: model.method,
        url: model.url,
        headers: serde_json::from_str(&model.headers)
            .map_err(|error| format!("Unable to decode saved headers: {error}"))?,
        body: model.body,
        created_at: model.created_at,
        updated_at: model.updated_at,
    })
}

fn now() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

#[tauri::command]
async fn list_saved_requests(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<Vec<SavedRequestSummary>, String> {
    SavedRequest::find()
        .order_by_desc(request_entity::Column::UpdatedAt)
        .all(db.inner())
        .await
        .map_err(|error| error.to_string())
        .map(|items| {
            items
                .into_iter()
                .map(|item| SavedRequestSummary {
                    id: item.id,
                    name: item.name,
                    method: item.method,
                    url: item.url,
                    updated_at: item.updated_at,
                })
                .collect()
        })
}

#[tauri::command]
async fn create_saved_request(
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<SavedRequestDto, String> {
    let timestamp = now();
    let item = SavedRequestActive {
        name: Set("未命名请求".into()),
        method: Set("GET".into()),
        url: Set("".into()),
        headers: Set("[{\"name\":\"\",\"value\":\"\"}]".into()),
        body: Set(Some("{\n  \n}".into())),
        created_at: Set(timestamp.clone()),
        updated_at: Set(timestamp),
        ..Default::default()
    }
    .insert(db.inner())
    .await
    .map_err(|error| error.to_string())?;
    dto_from_model(item)
}

#[tauri::command]
async fn get_saved_request(
    id: i64,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<SavedRequestDto, String> {
    let item = SavedRequest::find_by_id(id)
        .one(db.inner())
        .await
        .map_err(|error| error.to_string())?
        .ok_or("接口不存在")?;
    dto_from_model(item)
}

#[tauri::command]
async fn save_saved_request(
    input: SaveRequestInput,
    db: tauri::State<'_, DatabaseConnection>,
) -> Result<SavedRequestDto, String> {
    let headers =
        serde_json::to_string(&input.request.headers).map_err(|error| error.to_string())?;
    let item = SavedRequest::find_by_id(input.id)
        .one(db.inner())
        .await
        .map_err(|error| error.to_string())?
        .ok_or("接口不存在")?;
    let mut item: SavedRequestActive = item.into();
    item.name = Set(input.name.trim().to_string());
    item.method = Set(input.request.method);
    item.url = Set(input.request.url);
    item.headers = Set(headers);
    item.body = Set(input.request.body);
    item.updated_at = Set(now());
    dto_from_model(
        item.update(db.inner())
            .await
            .map_err(|error| error.to_string())?,
    )
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
        .setup(|app| {
            let database = tauri::async_runtime::block_on(initialize_database())
                .map_err(std::io::Error::other)?;
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            request_api,
            list_saved_requests,
            create_saved_request,
            get_saved_request,
            save_saved_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
