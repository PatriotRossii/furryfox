use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::problem::Problem;

lazy_static! {
    static ref BOLD_HEADER: Regex = Regex::new(r"=== '''(.*)''' ===").unwrap();
}

pub struct ID44 {}

impl Problem for ID44 {
    const PROBLEM_DESCRIPTION: &'static str = "ID 44. Заголовок содержит выделение «жирным»";

    fn detect(text: &str) -> bool {
        BOLD_HEADER.find(text).is_some()
    }

    fn replace(text: &str) -> String {
        BOLD_HEADER
            .replace_all(text, |caps: &Captures| format!("=== {} ===", &caps[0]))
            .to_string()
    }
}
