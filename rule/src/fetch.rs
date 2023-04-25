use std::env;
use std::ops::Deref;

use anyhow::{anyhow, Error};
use once_cell::sync::Lazy;
use reqwest::{Client, Response};

use models::ver;

use crate::rule_index::RULES;
use crate::{FnSignature, UA};

static TOKEN: Lazy<String> = Lazy::new(|| env::var("GITHUB_TOKEN").unwrap());

pub async fn fetch_app(app: &ver::Model, client: Client) -> Result<String, Error> {
    if app.name == *"Fences" {
        let resp: Response = client.head(&app.url).send().await?;
        let head: &str = resp.headers()["Content-Length"].to_str()?;
        Ok(head.to_owned())
    } else if app.name == *"EmEditor" {
        let resp: Response = Client::builder()
            .user_agent(UA)
            .redirect(reqwest::redirect::Policy::none())
            .build()?
            .get(&app.url)
            .send()
            .await?;
        let func: &FnSignature = RULES.get(app.name.as_str()).unwrap();
        let arg: &str = resp.headers()["location"].to_str()?;
        func(arg).ok_or(anyhow!("解析版本错误"))
    } else if app.json == 1 {
        let resp: Response = {
            if app.url.starts_with("https://api.github.com") {
                client
                    .get(&app.url)
                    .header("Authorization", format!("token {}", TOKEN.deref()))
                    .send()
                    .await?
            } else {
                client.get(&app.url).send().await?
            }
        };
        let j: serde_json::Value = resp.json::<serde_json::Value>().await?;
        let v: String = match app.name.as_str() {
            "PyCharm" => j["PCP"][0]["version"].to_string(),
            "Clash" => j["name"].to_string(),
            _ => j["tag_name"].to_string(),
        };
        Ok(v)
    } else {
        let resp: Response = client.get(&app.url).send().await?;
        let func: &FnSignature = RULES.get(app.name.as_str()).unwrap();
        let arg: String = resp.text().await?;
        func(&arg).ok_or(anyhow!("解析版本错误"))
    }
}
