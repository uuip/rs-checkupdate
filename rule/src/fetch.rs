use std::env;
use std::ops::Deref;

use anyhow::{anyhow, Error};
use once_cell::sync::Lazy;
use reqwest::{Client, header, Response};

use models::ver;

use crate::rule_index::{CSSRULES, FNRULES};
use crate::rules::parse_css;

static TOKEN: Lazy<String> = Lazy::new(|| env::var("GITHUB_TOKEN").unwrap_or_default());
pub const UA: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:112.0) Gecko/20100101 Firefox/112.0";

pub async fn parse_app(app: &ver::Model) -> Result<String, Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(UA));
    let client = Client::builder().default_headers(headers).build()?;

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
        let arg: &str = resp.headers()["location"].to_str()?;
        find_version(app, arg).ok_or(anyhow!("解析版本错误"))
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
        let arg: String = resp.text().await?;
        find_version(app, &arg).ok_or(anyhow!("解析版本错误"))
    }
}

fn find_version(app: &ver::Model, resp: &str) -> Option<String> {
    let app_name = app.name.as_str();
    let func = FNRULES.get(app_name);
    if let Some(f) = func {
        f(resp)
    } else {
        CSSRULES.get(app_name).map(|css| parse_css(resp, css))?
    }
}
