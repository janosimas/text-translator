use std::env;
use text_translator::*;

const REFERENCE_TEXT: &str =
    "There is no real ending. It's just the place where you stop the story.";
const REFERENCE_TRANSLATION: &str =
    "Es gibt kein wirkliches Ende. Es ist nur der Ort, an dem Sie die Geschichte stoppen.";

#[tokio::test]
async fn google_api_single_translate() {
    dotenv::dotenv().ok();
    let mut google_key = String::default();
    for (key, value) in env::vars() {
        // if key == "YANDEX_KEY" {
        if key == "GOOGLE_TRANSLATE_API_KEY_GITHUB_ACTIONS" {
            google_key = value;
        }
    }
    let translator = GoogleV2::with_key(&google_key);
    let res = translator.translate(
        REFERENCE_TEXT.to_string(),
        InputLanguage::Defined(Language::English),
        Language::German,
    );

    assert_eq!(res.await.unwrap(), REFERENCE_TRANSLATION);
}
