#![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::sync::Arc;

use futures::{stream, StreamExt};
use mincolor::*;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::{header, Client};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait};
use serde_json::json;
use tokio::sync::Mutex;

use models::ver;
use models::Ver;
use rule::{fetch_app, UA};

type SharedStatus<'a> = Arc<Mutex<HashMap<&'a str, Vec<&'a str>>>>;
static VER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[.\d]*\d").unwrap());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(target_os = "windows") {
        let _ = enable_ansi_support::enable_ansi_support();
    }
    let now = std::time::SystemTime::now();
    let status: SharedStatus = Arc::new(Mutex::new(HashMap::from([
        ("success", Vec::new()),
        ("failed", Vec::new()),
    ])));

    let db: DatabaseConnection =
        // Database::connect("postgres://postgres:postgres@127.0.0.1/postgres").await?;
        Database::connect("sqlite:///C:/Users/sharp/AppData/Local/Programs/checkupdate/ver_tab.db").await?;
    let a = Ver::find_by_id("fzf").one(&db).await?.unwrap();
    let aj: serde_json::Value = json!(a);
    println!("{}\n", serde_json::to_string_pretty(&aj)?);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(UA));
    let client = Client::builder().default_headers(headers).build()?;

    let apps = Ver::find().all(&db).await?;
    let tasks = stream::iter(apps)
        .map(|app| {
            let client = client.clone();
            let db = db.clone();
            let status = status.clone();
            async move { update_app(app, client, db, status).await }
        })
        .buffer_unordered(64)
        .collect::<Vec<_>>()
        .await;

    println!("用时{:.2?}秒", now.elapsed()?.as_secs_f32());
    println!("{:?}", status);
    if cfg!(target_os = "windows") & !cfg!(debug_assertions) {
        let _ = std::process::Command::new("cmd.exe")
            .arg("/c")
            .arg("pause")
            .status();
    }
    Ok(())
}

async fn update_app(
    app: ver::Model,
    client: Client,
    db: DatabaseConnection,
    status: SharedStatus<'_>,
) {
    let new_ver = fetch_app(&app, client).await.map_or(None, num_version);
    let new_ver = if let Some(s) = new_ver {
        s
    } else {
        eprintln!("{} 获取版本失败\n{}", app.name, "=".repeat(36));
        let mut status = status.lock().await;
        status
            .get_mut("failed")
            .unwrap()
            .push(Box::leak(app.name.into_boxed_str()));
        return;
    };
    if new_ver != app.ver {
        let mut app: ver::ActiveModel = app.into();
        app.ver = Set(new_ver.to_owned());
        let app = app.update(&db).await.unwrap();
        println!("{} 更新为版本 {}", app.name.green(), new_ver.bright_green());
        let mut status = status.lock().await;
        status
            .get_mut("success")
            .unwrap()
            .push(Box::leak(app.name.into_boxed_str()));
    } else {
        println!("{} : {}", app.name.bright_cyan(), new_ver.bright_cyan());
    }
    println!("{}", "=".repeat(36));
}

fn num_version(ver_info: String) -> Option<String> {
    VER_RE
        .find(ver_info.as_str())
        .map(|x| x.as_str().to_string())
}