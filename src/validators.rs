use crate::form::FormField;
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;

static DIGIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]").unwrap());
static ONLY_DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+$").unwrap());

pub fn letters(data: &mut FormField) {
    if data.field.is_empty() {
        data.valid = false;
        return;
    }

    if DIGIT.is_match(&data.field) {
        data.valid = false;
    } else {
        data.valid = true;
    }

    data.field = Arc::new(data.field.to_uppercase());
}

pub fn numbers(data: &mut FormField) {
    if ONLY_DIGITS.is_match(&data.field) {
        data.valid = true;
    } else {
        data.valid = false;
    }
}
