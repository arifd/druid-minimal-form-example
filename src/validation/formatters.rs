use std::sync::Arc;

use druid::text::{Formatter, Selection, Validation, ValidationError};
use once_cell::sync::Lazy;
use regex::Regex;

///////////////////////////////////////////////////////////////////////////////
// STATICS                                                                   //
///////////////////////////////////////////////////////////////////////////////
static DIGITS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[0-9]").unwrap());
static TELEPHONE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\+?([0-9]+\s?)+$").unwrap());

///////////////////////////////////////////////////////////////////////////////
// FORMATTERS                                                                //
///////////////////////////////////////////////////////////////////////////////
pub struct NameFormatter;
impl Formatter<Arc<String>> for NameFormatter {
    fn format(&self, value: &Arc<String>) -> String {
        value.to_string()
    }

    fn value(&self, input: &str) -> Result<Arc<String>, ValidationError> {
        Ok(Arc::new(String::from(input)))
    }

    fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
        // Can't be empty
        if input.len() == 0 {
            return Validation::failure(Error::Empty);
        }

        if DIGITS_RE.is_match(input) {
            return Validation::failure(Error::Invalid("A name can not contain numbers"));
        }

        Validation::success()
    }
}

pub struct TelephoneFormatter;
impl Formatter<Arc<String>> for TelephoneFormatter {
    fn format(&self, value: &Arc<String>) -> String {
        value.to_string()
    }

    fn value(&self, input: &str) -> Result<Arc<String>, ValidationError> {
        Ok(Arc::new(String::from(input)))
    }

    fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
        // Can't be empty
        if input.len() == 0 {
            return Validation::failure(Error::Empty);
        }

        if !TELEPHONE_RE.is_match(input) {
            return Validation::failure(Error::Invalid("Telephone number not valid"));
        }

        Validation::success()
    }
}

///////////////////////////////////////////////////////////////////////////////
// ERRORS                                                                    //
///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
enum Error {
    Empty,
    Invalid(&'static str),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "Field can't be empty",
                Self::Invalid(reason) => reason,
            }
        )
    }
}
