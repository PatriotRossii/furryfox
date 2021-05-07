use async_trait::async_trait;
use mediawiki::{api::Api, page::Page};

pub mod low;
pub mod middle;

pub enum FurryfoxError {
    FailedToEdit,
    FailedToRetrievePageText,
}

#[async_trait]
pub trait Problem {
    const PROBLEM_DESCRIPTION: &'static str;

    fn detect(text: &str) -> bool;
    fn replace(text: &str) -> String;
    async fn fix(api: &mut Api, page: &Page) -> Result<bool, FurryfoxError> {
        let text = page
            .text(api)
            .await
            .map_err(|_| FurryfoxError::FailedToRetrievePageText)?;
        if Self::detect(&text) {
            let replacement = Self::replace(&text);
            page.edit_text(api, &replacement, Self::PROBLEM_DESCRIPTION)
                .await
                .map_err(|_| FurryfoxError::FailedToEdit)?;
            return Ok(true);
        }
        Ok(false)
    }
}
