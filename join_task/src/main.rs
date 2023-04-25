#![allow(dead_code, unused_variables)]

use mincolor::*;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::Client;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait};
use serde_json::json;
use tokio::task;

use models::ver;
use models::Ver;
use rule::{fetch_app, UA};

type VerModel = ver::Model;
static VER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[.\d]*\d").unwrap());

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(target_os = "windows") {
        let _ = enable_ansi_support::enable_ansi_support();
    }
    let now = std::time::SystemTime::now();

    let db: DatabaseConnection =
        // Database::connect("postgres://postgres:postgres@127.0.0.1/postgres").await?;
        // Database::connect("sqlite:///Users/sharp/Downloads/ver_tab.db").await?;
        Database::connect("sqlite:///C:/Users/sharp/AppData/Local/Programs/checkupdate/ver_tab.db").await?;
    let a = Ver::find_by_id("fzf").one(&db).await?.unwrap();
    let aj: serde_json::Value = json!(a);
    println!("{}\n", serde_json::to_string_pretty(&aj)?);

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(UA));
    let client: Client = Client::builder().default_headers(headers).build()?;

    let apps: Vec<VerModel> = Ver::find().all(&db).await?;
    let mut tasks = Vec::new();
    for app in apps {
        let client: Client = client.clone();
        let db = db.clone();
        let t = task::spawn(async move { update_app(app, client, db).await });
        tasks.push(t);
    }
    futures::future::join_all(tasks).await;
    println!("用时{:.2?}秒", now.elapsed()?.as_secs_f32());
    if cfg!(target_os = "windows") & !cfg!(debug_assertions) {
        let _ = std::process::Command::new("cmd.exe")
            .arg("/c")
            .arg("pause")
            .status();
    }
    Ok(())
}

async fn update_app(app: ver::Model, client: Client, db: DatabaseConnection) {
    let new_ver: String = match fetch_app(&app, client).await.map(num_version) {
        Ok(s) => s.unwrap(),
        Err(e) => {
            eprintln!("{} 获取版本失败: {}", app.name, e);
            println!("{}", "=".repeat(36));
            return;
        }
    };
    if new_ver != app.ver {
        println!("{} 更新为版本 {}", app.name.green(), new_ver.bright_green());
        let mut app: ver::ActiveModel = app.into();
        app.ver = Set(new_ver.to_owned());
        let _ = app.update(&db).await;
    } else {
        println!("{} : {} ", app.name, new_ver);
    }
    println!("{}", "=".repeat(36));
}

fn num_version(ver_info: String) -> Option<String> {
    VER_RE
        .captures(ver_info.as_str())?
        .get(0)
        .map(|x| x.as_str().to_string())
}
