use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::problem::Problem;

lazy_static! {
    static ref URL_WITH_DOUBLE_BRACKETS_RE: Regex = Regex::new(r"^\[\[((?:(?:(?:https?|ftp):)?\/\/)(?:\S+(?::\S*)?@)?(?:(?!(?:10|127)(?:\.\d{1,3}){3})(?!(?:169\.254|192\.168)(?:\.\d{1,3}){2})(?!172\.(?:1[6-9]|2\d|3[0-1])(?:\.\d{1,3}){2})(?:[1-9]\d?|1\d\d|2[01]\d|22[0-3])(?:\.(?:1?\d{1,2}|2[0-4]\d|25[0-5])){2}(?:\.(?:[1-9]\d?|1\d\d|2[0-4]\d|25[0-4]))|(?:(?:[a-z0-9\x{00a1}-\x{ffff}][a-z0-9\x{00a1}-\x{ffff}_-]{0,62})?[a-z0-9\x{00a1}-\x{ffff}]\.)+(?:[a-z\x{00a1}-\x{ffff}]{2,}\.?))(?::\d{2,5})?(?:[/?#]\S*)?)\]\]$").unwrap();
}

pub struct ID86 {}

impl Problem for ID86 {
    const PROBLEM_DESCRIPTION: &'static str = "ID 86. Внешняя ссылка с двумя квадратными скобками";

    fn detect(text: &str) -> bool {
        URL_WITH_DOUBLE_BRACKETS_RE.find(text).is_some()
    }

    fn replace(text: &str) -> String {
        URL_WITH_DOUBLE_BRACKETS_RE
            .replace_all(text, |caps: &Captures| format!("[{}]", &caps[0]))
            .to_string()
    }
}
