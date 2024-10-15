extern crate serde_json;

use rocket::serde::json::{json, Value};
use seshat::unicode::{Segmentation, ToCodePoint, UNICODE_VERSION};

use crate::versions::SESHAT_VERSION;

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

pub fn segmentation_grapheme_api_v3(text: String) -> Value {
    let graphemes = text.break_graphemes();

    let mut breaks: Vec<Value> = vec![];

    for seg in graphemes {
        let mut code_points = vec![];
        for character in seg.chars() {
            code_points.push(character.to_code_point().to_string());
        }
        breaks.push(json!({
            "string": seg,
            "code_points": code_points,
        }));
    }

    json!({
        "unicode_version": UNICODE_VERSION.to_string(),
        "seshat_version": SESHAT_VERSION,
        "text": text,
        "breaks": breaks,
    })
}
