extern crate serde_json;

use rocket_contrib::json::{JsonValue};
use seshat::unicode::{CodePoint, Ucd};
use seshat::unicode::props::UnicodeProperty;

pub fn properties_api(cp: CodePoint) -> JsonValue {
    json!({"codepoint": format!("{}", cp),
        "properties": {
            "numeric": {
                // TODO
            },
            "string": {
                // TODO
            },
            "misc": {
                "name": cp.na(),
            },
            "catalog": {
                // "age": cp.age(),
                // "block": cp.blk(),
            },
            "enumerated": {
                "canonical_combining_class": cp.ccc() as u8,
                "decomposition_type": {
                    "full": cp.dt().property_value_name().full,
                    "abbr": cp.dt().property_value_name().abbr,
                },
                "general_category": {
                    "full": cp.gc().property_value_name().full,
                    "abbr": cp.gc().property_value_name().abbr,
                },
                "grapheme_cluster_break": {
                    "full": cp.gcb().property_value_name().full,
                    "abbr": cp.gcb().property_value_name().abbr,
                },

            },
            "binary": {
                // TODO
            },
        },
        "seshat_version": "0.0.11"})
}