use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> Result<String> {
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(input),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(input),
    };

    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<String> {
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(input)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(input)?,
    };

    Ok(String::from_utf8(decoded)?)
}
