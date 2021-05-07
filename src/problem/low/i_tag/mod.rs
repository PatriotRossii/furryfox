use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::problem::Problem;

lazy_static! {
    static ref ITAG_RE: Regex = Regex::new(
        r"
        <i>(.*)<\/i>
    "
    )
    .unwrap();
}

pub struct ID38 {}

impl Problem for ID38 {
    const PROBLEM_DESCRIPTION: &'static str = "ID 38. HTML-тег курсивного начертания <i>";

    fn detect(text: &str) -> bool {
        ITAG_RE.find(text).is_some()
    }

    fn replace(text: &str) -> String {
        ITAG_RE
            .replace_all(text, |caps: &Captures| format!("''{}''", &caps[0]))
            .to_string()
    }
}
