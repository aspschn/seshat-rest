extern crate serde_json;

use rocket::serde::json::{json, Value};
use seshat::unicode::{Segmentation, ToCodePoint};

pub fn segmentation_graphemes_api(text: String) -> Value {
    let graphemes = text.break_graphemes();

    let mut breaks: Vec<Value> = vec![];

    for seg in graphemes {
        let mut characters: Vec<Value> = vec![];
        for character in seg.chars() {
            characters.push(json!({
                "character": character,
                "code_point": format!("{}", character.to_code_point()),
            }));
        }
        breaks.push(json!(characters));
    }

    json!({
        "breaks": breaks,
        "text": text,
        "type": "grapheme",
        "seshat_version": "0.0.15",
    })
}
