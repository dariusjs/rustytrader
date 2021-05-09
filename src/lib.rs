// use reqwest::Client;
// use std::env;
// use std::string::String;

// fn get_env_var(key: &str) -> String {
//     match env::var(key) {
//         Ok(val) => val.to_string(),
//         Err(error) => error.to_string(),
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let token = get_env_var("TOKEN");
//     let args: Vec<String> = env::args().collect();
//     let user = &args[1];

//     let response = Client::new()
//         .get(format!("https://api.spacetraders.io/users/{}", user))
//         .bearer_auth(token)
//         .send()
//         .await?
//         .text()
//         .await?;

//     println!("{:?}", response);

//     Ok(())
// }

#![recursion_limit = "256"]
mod app;
mod components;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
