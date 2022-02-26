extern crate serde_json;

use rocket::serde::json::{json, Value};
use seshat::unicode::{CodePoint, Ucd};
use seshat::unicode::props::UnicodeProperty;


pub fn properties_api(cp: CodePoint) -> Value {
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
                "age": cp.age().property_value_name().abbr,
                "block": {
                    "full": cp.blk().property_value_name().full,
                    "abbr": cp.blk().property_value_name().abbr,
                },
                "script": {
                    "full": cp.sc().property_value_name().full,
                    "abbr": cp.sc().property_value_name().abbr,
                }
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
                "ascii_hex_digit": cp.ahex(),
                "bidi_control": cp.bidi_c(),
                "case_ignorable": cp.ci(),
                "cased": cp.cased(),
                "dash": cp.dash(),
                "default_ignorable_code_point": cp.di(),
                "deprecated": cp.dep(),
                "diacritic": cp.dia(),
                "extender": cp.ext(),
                "full_composition_exclusion": cp.comp_ex(),
                "grapheme_extend": cp.gr_ext(),
                "hex_digit": cp.hex(),
                "hyphen": cp.hyphen(),
                "ideographic": cp.ideo(),
                "ids_binary_operator": cp.idsb(),
                "ids_trinary_operator": cp.idst(),
                "join_control": cp.join_c(),
                "logical_order_exception": cp.loe(),
                "lowercase": cp.lower(),
                "noncharacter_code_point": cp.nchar(),
                "other_alphabetic": cp.oalpha(),
                "other_default_ignorable_code_point": cp.odi(),
                "other_grapheme_extend": cp.ogr_ext(),
                "other_id_continue": cp.oidc(),
                "other_id_start": cp.oids(),
                "other_lowercase": cp.olower(),
                "other_math": cp.omath(),
                "other_uppercase": cp.oupper(),
                "pattern_syntax": cp.pat_syn(),
                "pattern_white_space": cp.pat_ws(),
                "prepended_concatenation_mark": cp.pcm(),
                "quotation_mark": cp.qmark(),
                "radical": cp.radical(),
                "regional_indicator": cp.ri(),
                "sentence_terminal": cp.sterm(),
                "soft_dotted": cp.sd(),
                "terminal_punctuation": cp.term(),
                "unified_ideograph": cp.uideo(),
                "uppercase": cp.upper(),
                "variation_selector": cp.vs(),
                "white_space": cp.wspace(),
            },
        },
        "seshat_version": "0.0.15"})
}