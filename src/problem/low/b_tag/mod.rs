use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::problem::Problem;

lazy_static! {
    static ref BTAG_RE: Regex = Regex::new(
        r"
        <b>(.*)<\/b>
    "
    )
    .unwrap();
}

pub struct ID26 {}

impl Problem for ID26 {
    const PROBLEM_DESCRIPTION: &'static str = "ID 26. HTML-тег выделения жирным <b>";

    fn detect(text: &str) -> bool {
        BTAG_RE.find(text).is_some()
    }

    fn replace(text: &str) -> String {
        BTAG_RE
            .replace_all(text, |caps: &Captures| format!("'''{}'''", &caps[0]))
            .to_string()
    }
}
