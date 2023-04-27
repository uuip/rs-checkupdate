#![allow(dead_code, unused_variables)]

use anyhow::Error;
use mincolor::*;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue::Set;
use serde_json::json;
use tokio::task::JoinSet;

use models::ver;
use models::VerEntity;
use rule::{fetch_app, num_version};

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
    let a = VerEntity::find_by_id("fzf").one(&db).await?.unwrap();
    let aj: serde_json::Value = json!(a);
    println!("{}\n", serde_json::to_string_pretty(&aj)?);

    let apps = VerEntity::find().all(&db).await?;
    let mut set: JoinSet<Result<(ver::Model, String), Error>> = JoinSet::new();
    for app in apps {
        let msg: String = format!("{} 提取版本号失败", app.name);
        set.spawn(async move {
            let new_ver = match fetch_app(&app).await.map(num_version) {
                Ok(s) => s.unwrap(),
                Err(e) => {
                    eprintln!("{} 获取版本失败: {}", app.name, e);
                    println!("{}", "=".repeat(36));
                    return Err(e);
                }
            };
            Ok((app, new_ver))
        });
    }

    while let Some(res) = set.join_next().await {
        let (app, new_ver) = match res? {
            Ok(r) => r,
            _ => continue,
        };
        update_app(&db, app, new_ver).await;
    }

    println!("用时{:.2?}秒", now.elapsed()?.as_secs_f32());
    if cfg!(target_os = "windows") & !cfg!(debug_assertions) {
        let _ = std::process::Command::new("cmd.exe")
            .arg("/c")
            .arg("pause")
            .status();
    }
    Ok(())
}

async fn update_app(db: &DatabaseConnection, app: ver::Model, new_ver: String) {
    if new_ver != app.ver {
        println!("{} 更新为版本 {}", app.name.green(), new_ver.bright_green());
        let mut app: ver::ActiveModel = app.into();
        app.ver = Set(new_ver);
        let _ = app.update(db).await;
    } else {
        println!("{} : {} ", app.name, new_ver);
    }
    println!("{}", "=".repeat(36));
}
