use phf::phf_map;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serenity::json;
use serenity::json::json;
use std::env;

use crate::{Context, Error};
use poise::command;

const EVAL_LANGS: phf::Map<&'static str, &'static str> = phf_map! {
    "js" => "javascript",
    "py" => "python",
    "ts" => "typescript",
    "node.js" => "nodejs",
    "c#" => "csharp",
    "c++" => "c_cpp",
    "cpp" => "c_cpp",
};
const ALL_LANGS: [&str; 16] = [
    "php",
    "python",
    "c",
    "c_cpp",
    "csharp",
    "kotlin",
    "golang",
    "r",
    "java",
    "typescript",
    "nodejs",
    "ruby",
    "perl",
    "swift",
    "fortran",
    "bash",
];

#[command(prefix_command)]
pub async fn eval(ctx: Context<'_>, lang: String, #[rest] code: String) -> Result<(), Error> {
    const URL: &str = "https://code-compiler10.p.rapidapi.com/";

    // Parse the code if it is a codeblock
    let code: &str = &Regex::new(r"(^```.*\n)|(^`)|`{1,3}$")
        .unwrap()
        .replace_all(&code, "");

    // check for aliases for the language
    let lang = if EVAL_LANGS.contains_key(lang.as_str()) {
        EVAL_LANGS[lang.as_str()]
    } else {
        lang.as_str()
    };

    // If language is invalid, return
    if !ALL_LANGS.contains(&lang) {
        ctx.say("Invalid language").await?;
        return Ok(());
    }

    let payload = json!({
        "langEnum": ALL_LANGS,
        "lang": lang,
        "code": code,
        "input": ""
    });

    let mut header = HeaderMap::new();
    header.insert(
        "content-type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    header.insert("x-compile", HeaderValue::from_str("rapidapi").unwrap());
    header.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    header.insert(
        "X-RapidAPI-Key",
        HeaderValue::from_str(&env::var("API_TOKEN").unwrap()).unwrap(),
    );
    header.insert(
        "X-RapidAPI-Host",
        HeaderValue::from_str("code-compiler10.p.rapidapi.com").unwrap(),
    );

    // Create an async web request
    let response = Client::new()
        .post(URL)
        .json(&payload)
        .headers(header)
        .send()
        .await?;

    // Check if the request was successful
    if !response.status().is_success() {
        let _ = ctx
            .say(format!(
                "The following error occured: {}",
                response.text().await?
            ))
            .await;
        return Ok(());
    }

    // Parse the response
    let data: json::JsonMap = response.json().await?;

    // if the second to last character of data["output"] is a newline, remove it
    let mut output = data["output"].as_str().unwrap().to_string();
    if output.chars().rev().nth(1) == Some('\n') {
        output.pop();
    }

    // Send the response
    ctx.say(format!("```{}\n{}```", lang, output)).await?;

    Ok(())
}
