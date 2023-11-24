/*

Js - Javascript

fetch('https://jsonplaceholder.typicode.com/todos/1')
      .then(response => response.json())
      .then(json => console.log(json))
*/

use reqwest;
use serde_json;

#[tokio::main]
async fn main() {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
        .await
        .unwrap();

    if response.status().is_success() {
        let result: serde_json::Value = response.json().await.unwrap();
        println!("{result:#?}");
    } else {
        println!("Request failed with status: {}", response.status());
    }
}
