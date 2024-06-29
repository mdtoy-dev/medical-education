use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub author: String,
    pub created_at: String,
}

#[wasm_bindgen]
pub fn create_content(title: &str, body: &str, author: &str, tags: JsValue) -> JsValue {
    let content = Content {
        title: title.to_string(),
        body: body.to_string(),
        tags: from_value(tags).unwrap(),
        metadata: Metadata {
            author: author.to_string(),
            created_at: "2024-06-29".to_string(), // Placeholder date
        },
    };
    to_value(&content).unwrap()
}

#[wasm_bindgen]
pub fn update_content(content: JsValue, title: &str, body: &str, tags: JsValue) -> JsValue {
    let mut content: Content = from_value(content).unwrap();
    content.title = title.to_string();
    content.body = body.to_string();
    content.tags = from_value(tags).unwrap();
    to_value(&content).unwrap()
}
