mod app;
pub use app::BeanCypher;
use std::collections::HashMap;

#[path = "decode.rs"]
mod decode;
#[path = "encode.rs"]
mod encode;
mod hash_convert;

#[derive(Debug, Clone)]
pub enum ErrorState {
    Error(String),
    Warning(String),
    None,
}

impl ErrorState {
    fn into_string(self) -> String {
        match self {
            Self::Warning(s) | Self::Error(s) => s,
            Self::None => String::new(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Language {
    English,
    Latvian,
}

// TODO: i would LOVE to have it not be a function and just a definition of the enum (so i'd only need to do Language::English.app_name), but for the life of me i couldn't figure out what concoction i needed to amass to do it.
// please, future me, you are my only hope of solving this. please make it easier avoid mistyping. this shitpost depends on it.
impl Language {
    fn get_string(self, key: &str) -> String {
        match self {
            Self::English => (*HashMap::from([
                ("app_name", "Bean Cypher"),
                ("menu_file", "File"),
                ("menu_file_encode", "Encode from file..."),
                ("menu_file_decode", "Decode from file..."),
                ("menu_file_quit", "Quit"),
                ("menu_settings", "Settings"),
                (
                    "settings_flavour",
                    "(Only theme selection does not get saved.)",
                ),
                ("set_theme", "Theme"),
                ("set_lowercase", "Lowercase output"),
                ("set_enable_cypher", "Enable custom cypher"),
                ("set_cypher", "Custom cypher"),
                ("set_cypher_import", "Import from text file..."),
                ("set_silly", "a silly :3"),
                ("silly_true", "OH GOD PLEASE NO"),
                ("silly_false", "Begin!"),
                ("set_language", "Language"),
                ("menu_credits", "Credits"),
                ("cred_version", "Version"),
                ("cred_creator", "Creator"),
                ("cred_cbo", "Chief Bean Officer"),
                ("cred_fav_station", "Kortimu's favourite radio station"),
                ("cred_sanity", "Current sanity levels:  Not that much"),
                ("cred_source", "Source code"),
                ("cred_website", "Website"),
                ("cred_funny_img", "funny image"),
                ("hint_input", "Input text here..."),
                ("btn_encode", "Encode text"),
                ("btn_decode", "Decode text"),
                ("btn_copy", "Copy to clipboard"),
                ("btn_save", "Save output..."),
                ("text", "Text"),
                // TODO: credits
            ])
            .get(key)
            .unwrap_or(&key))
            .to_string(),
            Self::Latvian => (*HashMap::from([
                ("app_name", "Pupiņu Šifrs"),
                ("menu_file", "Fails"),
                ("menu_file_encode", "Šifrēt no faila..."),
                ("menu_file_decode", "Atšifrēt no faila..."),
                ("menu_file_quit", "Aizvērt"),
                ("menu_settings", "Iestatījumi"),
                // ??
                (
                    "settings_flavour",
                    "(Tikai dizaina izvēle netiek saglabāta.)",
                ),
                // ??
                ("set_theme", "Dizains"),
                ("set_lowercase", "Mazie burti rezultātā"),
                ("set_enable_cypher", "Ieslēgt pielāgotu šifru"),
                ("set_cypher", "Pielāgotais šifrs"),
                ("set_cypher_import", "Importēt no teksta faila..."),
                ("set_silly", "kkas muļķīgs :3"),
                ("silly_true", "GRIBU ATPAKAĻ"),
                ("silly_false", "Sākt!"),
                ("set_language", "Valoda"),
                ("menu_credits", "Atzinības"),
                ("cred_version", "Versija"),
                ("cred_creator", "Autors"),
                ("cred_cbo", "Galvenais Pupiņu Virsnieks"),
                ("cred_fav_station", "Radio stacija, kuru Kortimu dievina"),
                ("cred_sanity", "Pašreizējais prāta līmenis:  Ne pārāk liels"),
                // ??
                ("cred_source", "Koda avots"),
                ("cred_website", "Mājaslapa"),
                ("cred_funny_img", "smieklīga bildīte"),
                ("hint_input", "Raksti šeit..."),
                ("btn_encode", "Šifrēt tekstu"),
                ("btn_decode", "Atšifrēt tekstu"),
                ("btn_copy", "Kopēt starpliktuvē"),
                ("btn_save", "Saglabāt rezultātu..."),
                ("text", "Teksts"),
            ])
            .get(key)
            .unwrap_or(&key))
            .to_string(),
        }
    }
}
