use crate::*;

mod yandex;
pub use yandex::*;

/// Lists the differents implemented translators API.
#[derive(Debug)]
pub enum Translator<'a> {
    Yandex { key: &'a str },
}

impl<'a> Translator<'a> {
    pub fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error> {
        match self {
            Translator::Yandex { key } => {
                Yandex::with_key(key.clone()).translate(text, source_language, target_language)
            }
        }
    }
}

pub trait Api {
    fn new() -> Self;

    fn translate(
        &self,
        text: String,
        source_language: InputLanguage,
        target_language: Language,
    ) -> Result<String, Error>;
}

pub trait ApiKey<'a>: Api + Sized {
    fn with_key(key: &'a str) -> Self;

    fn set_set(&mut self, key: &'a str);

    fn get_key(&self) -> Option<&'a str>;
}

trait ApiResponse {
    fn get_text(&self) -> String;
}

trait ApiError {
    fn from_error_code(code: u16) -> Self;
}
