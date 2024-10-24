extern crate serde_json;

use serde_json::{json, Value};
use seshat::unicode::{Normalization, UNICODE_VERSION};

use crate::versions::SESHAT_VERSION;

pub enum NormalizationForm {
    Nfd,
    Nfc,
    Nfkd,
    Nfkc,
}

pub fn normalization_api(text: String, form: NormalizationForm) -> Value {
    let norm = match form {
        NormalizationForm::Nfd => {
            text.to_nfd()
        },
        NormalizationForm::Nfc => {
            text.to_nfc()
        },
        NormalizationForm::Nfkd => {
            text.to_nfkd()
        },
        NormalizationForm::Nfkc => {
            text.to_nfkc()
        }
    };

    let form = match form {
        NormalizationForm::Nfd => { "NFD" },
        NormalizationForm::Nfc => { "NFC" },
        NormalizationForm::Nfkd => { "NFKD" },
        NormalizationForm::Nfkc => { "NFKC" },
    };

    json!({
        "unicode_version": UNICODE_VERSION.to_string(),
        "seshat_version": SESHAT_VERSION,
        "text": text,
        "normalized": norm,
        "form": form,
    })
}
