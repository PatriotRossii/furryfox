use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::problem::Problem;

lazy_static! {
    static ref SERIAL_SUP_RE: Regex = Regex::new(r"(?i)(\d+)<sup>(th|rd|nd)</sup>").unwrap();
}

pub struct ID101  {}

impl Problem for ID101 {
    const PROBLEM_DESCRIPTION: &'static str = "ID 101. Порядковый номер с надстрочным текстом";

    fn detect(text: &str) -> bool {
        SERIAL_SUP_RE.find(text).is_some()
    }

    fn replace(text: &str) -> String {
        SERIAL_SUP_RE
            .replace_all(text, |caps: &Captures| format!("{}{}", &caps[0], &caps[1]))
            .to_string()
    }
}
