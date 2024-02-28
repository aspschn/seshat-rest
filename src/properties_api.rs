extern crate serde_json;

use rocket::serde::json::{json, Value};
use serde_derive::{Serialize, Deserialize};
use seshat::unicode::{CodePoint, Ucd};
use seshat::unicode::props::UnicodeProperty;
use seshat::unicode::props::PropertyName;
use seshat::unicode::props::BinaryProperty;


/// Property name struct with a helper class.
#[derive(Serialize, Deserialize)]
struct PropName {
    full: &'static str,
    abbr: &'static str,
}

impl PropName {
    fn new(full: &'static str, abbr: &'static str) -> PropName {
        PropName {
            full,
            abbr,
        }
    }
}


/// Serializable version of `PropertyName`.
#[derive(Serialize, Deserialize)]
struct PropValue {
    full: &'static str,
    abbr: &'static str,
}

impl From<PropertyName> for PropValue {
    fn from(value: PropertyName) -> Self {
        PropValue {
            full: value.full,
            abbr: value.abbr,
        }
    }
}

/// Binary property struct.
#[derive(Serialize)]
struct BinaryProp {
    property_name: PropName,
    value: PropValue,
    boolean_value: bool,
}

impl BinaryProp {
    fn build(full: &'static str, abbr: &'static str, value: bool) -> BinaryProp {
        BinaryProp {
            property_name: PropName::new(full, abbr),
            value: PropValue::from(BinaryProperty::from(value).property_value_name()),
            boolean_value: value,
        }
    }
}


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

pub fn properties_api_v3(cp: CodePoint) -> Value {
    json!({
        "code_point": format!("{}", cp),
        "seshat_version": "0.2.1",
        "properties": {
            "numeric": [
                {
                    "property_name": {
                        "full": "Canonical_Combining_Class",
                        "abbr": "ccc",
                    },
                    "value": PropValue::from(cp.ccc().property_value_name()),
                    "numeric_value": cp.ccc() as u8,
                }
            ],
            "miscellaneous": [
                {
                    "property_name": PropName::new("Name", "na"),
                    "value": {
                        "full": "",
                        "abbr": "",
                    },
                    "string_value": cp.na(),
                },
                /*{
                    "property_name": {
                        "full": "Jamo_Short_Name",
                        "abbr": "JSN",
                    },
                    "value": {
                        "full": "",
                        "abbr": "",
                    },
                    "string_value": "",
                },*/
            ],
            "catalog": [
                {
                    "property_name": PropName::new("Age", "age"),
                    "value": PropValue::from(cp.age().property_value_name()),
                },
                {
                    "property_name": PropName::new("Block", "blk"),
                    "value": PropValue::from(cp.blk().property_value_name()),
                },
                {
                    "property_name": PropName::new("Script", "sc"),
                    "value": PropValue::from(cp.sc().property_value_name()),
                },
            ],
            "enumeration": [
                {
                    "property_name": PropName::new("General_Category", "gc"),
                    "value": PropValue::from(cp.gc().property_value_name()),
                },
                {
                    "property_name": PropName::new("Hangul_Syllable_Type", "hst"),
                    "value": PropValue::from(cp.hst().property_value_name()),
                },
                {
                    "property_name": PropName::new("Decomposition_Type", "dt"),
                    "value": PropValue::from(cp.dt().property_value_name()),
                },
                {
                    "property_name": PropName::new("Bidi_Class", "bc"),
                    "value": PropValue::from(cp.bc().property_value_name()),
                },
                {
                    "property_name": PropName::new("Grapheme_Cluster_Break", "GCB"),
                    "value": PropValue::from(cp.gcb().property_value_name()),
                },
                {
                    "property_name": PropName::new("Word_Break", "WB"),
                    "value": PropValue::from(cp.wb().property_value_name()),
                },
                {
                    "property_name": PropName::new("Indic_Syllabic_Category", "InSC"),
                    "value": PropValue::from(cp.insc().property_value_name()),
                },
                {
                    "property_name": PropName::new("Indic_Conjunct_Break", "InCB"),
                    "value": PropValue::from(cp.incb().property_value_name()),
                },
            ],
            "binary": [
                {
                    "property_name": PropName::new("White_Space", "WSpace"),
                    "value": PropValue::from(BinaryProperty::from(cp.wspace()).property_value_name()),
                    "boolean_value": cp.wspace(),
                },
                BinaryProp::build("Alphabetic", "Alpha", cp.alpha()),
                BinaryProp::build("Noncharacter_Code_Point", "NChar", cp.nchar()),
                BinaryProp::build("Default_Ignorable_Code_Point", "DI", cp.di()),
            ],
        }
    })
}
