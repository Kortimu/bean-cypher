#![allow(dead_code)]
// mod app;
// pub use app::BeanCypher;
mod app_test;
pub use app_test::TestApp;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum Tab {
    Cypher,
    Settings,
    Credits,
    Manual,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum ManualSection {
    Intro,
    Basics,
    Explanation,
    CustomCyphers,
    Edge,
    Faq,
    Thanks,
    Sillies,
}

impl ManualSection {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Intro => "introduction",
            Self::Basics => "cypher basics",
            Self::Explanation => "how the cypher works",
            Self::CustomCyphers => "custom cyphers",
            Self::Edge => "E.D.G.E.",
            Self::Faq => "FAQ",
            Self::Thanks => "thanks",
            Self::Sillies => "stupid bean images",
        }
    }

    pub const fn file_path(self) -> &'static str {
        match self {
            Self::Intro => include_str!("../assets/manual/01-intro.md"),
            Self::Basics => include_str!("../assets/manual/02-basics.md"),
            Self::Explanation => include_str!("../assets/manual/03-explanation.md"),
            Self::CustomCyphers => include_str!("../assets/manual/04-custom_cyphers.md"),
            Self::Edge => include_str!("../assets/manual/05-edge.md"),
            Self::Faq => include_str!("../assets/manual/06-faq.md"),
            Self::Thanks => include_str!("../assets/manual/07-thanks.md"),
            Self::Sillies => include_str!("../assets/manual/08-sillies.md"),
        }
    }
}

enum ErrorType {
    EmptyInput,
    _DecodingInputLacksInfo,
    _FailedFile,
    Wip,
}

#[path = "widgets/toggle_button.rs"]
mod toggle_button;
use crate::toggle_button::toggle;

fn get_app_version() -> Option<(u32, u32)> {
    let program_major = match env!("CARGO_PKG_VERSION_MAJOR").parse() {
        Ok(result) => result,
        Err(error) => {
            println!("{error}: Flawed version major value in Cargo.toml.");
            return None;
        }
    };
    let program_minor = match env!("CARGO_PKG_VERSION_MINOR").parse() {
        Ok(result) => result,
        Err(error) => {
            println!("{error}: Flawed version minor value in Cargo.toml.");
            return None;
        }
    };

    Some((program_major, program_minor))
}

fn load_flag(ctx: &egui::Context, lang: &str) -> egui::TextureHandle {
    let path = format!("assets/{lang}.png");

    let bytes = std::fs::read(path).expect("reading failed for flag :<");
    let image = image::load_from_memory(&bytes)
        .expect("loading from memory failed for flag :<")
        .to_rgba8();

    let size = [image.width() as usize, image.height() as usize];

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image);

    ctx.load_texture(
        format!("flag_{lang}"),
        color_image,
        egui::TextureOptions::LINEAR,
    )
}
// ----------------- old stuff begins here -----------------

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

#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
struct Language {
    lang_name: &'static str,
    app_name: &'static str,
    menu_file: &'static str,
    menu_file_encode: &'static str,
    menu_file_decode: &'static str,
    menu_file_quit: &'static str,
    menu_settings: &'static str,
    settings_flavour: &'static str,
    set_theme: &'static str,
    set_lowercase: &'static str,
    set_enable_cypher: &'static str,
    set_cypher: &'static str,
    set_cypher_import: &'static str,
    set_silly: &'static str,
    silly_true: &'static str,
    silly_false: &'static str,
    set_lang: &'static str,
    menu_credits: &'static str,
    cred_version: &'static str,
    cred_creator: &'static str,
    cred_cbo: &'static str,
    cred_fav_station: &'static str,
    cred_sanity: &'static str,
    cred_source: &'static str,
    cred_website: &'static str,
    cred_funny_img: &'static str,
    hint_input: &'static str,
    btn_encode: &'static str,
    btn_decode: &'static str,
    btn_copy: &'static str,
    btn_save: &'static str,
    text: &'static str,
    err_cypher_phrase: &'static str,
    err_file_parse: &'static str,
    err_file_select: &'static str,
    err_file_location: &'static str,
    err_decode_info: &'static str,
    err_decode_v_major: &'static str,
    err_decode_v_minor: &'static str,
    warn_version_line_1: &'static str,
    warn_version_line_2: &'static str,
    warn_version_line_3: &'static str,
}

const ENGLISH: Language = Language {
    lang_name: "English",
    app_name: "Bean Cypher",
    menu_file: "File",
    menu_file_encode: "Encode from file...",
    menu_file_decode: "Decode from file...",
    menu_file_quit: "Quit",
    menu_settings: "Settings",
    settings_flavour: "(Only langauge selection does not get saved.)",
    set_theme: "Theme",
    set_lowercase: "Lowercase output",
    set_enable_cypher: "Enable custom cypher",
    set_cypher: "Custom cypher",
    set_cypher_import: "Import from text file...",
    set_silly: "a silly :3",
    silly_true: "OH GOD PLEASE NO",
    silly_false: "Begin!",
    set_lang: "Language",
    menu_credits: "Credits",
    cred_version: "Version",
    cred_creator: "Creator",
    cred_cbo: "Chief Bean Officer",
    cred_fav_station: "My favourite radio station",
    cred_sanity: "Current sanity levels:  Not that much",
    cred_source: "Source code",
    cred_website: "Website",
    cred_funny_img: "funny image",
    hint_input: "Input text here...",
    btn_encode: "Encode text",
    btn_decode: "Decode text",
    btn_copy: "Copy to clipboard",
    btn_save: "Save output...",
    text: "Text",
    err_cypher_phrase: "Cypher error: Cypher doesn't contain every character as a separate ID.",
    err_file_parse: "File error: Failed to parse selected file.",
    err_file_select: "File error: Failed to select a text file.",
    err_file_location: "File error: Faulty file location.",
    // the following are unused for now because decode::run() doesn't have access to self/app/whatever
    err_decode_info: "Decoding error: Input lacks info.",
    err_decode_v_major: "Decoding error: Flawed major version value in Cargo.toml.",
    err_decode_v_minor: "Decoding error: Flawed minor version value in Cargo.toml.",
    warn_version_line_1: "Warning: The text might get decoded wrong due to mismatched versions.",
    warn_version_line_2: "Encoded in",
    warn_version_line_3: "Decoded in",
};

const LATVIAN: Language = Language {
    lang_name: "Latviešu",
    app_name: "Pupiņu Šifrs",
    menu_file: "Fails",
    menu_file_encode: "Šifrēt no faila...",
    menu_file_decode: "Atšifrēt no faila...",
    menu_file_quit: "Aizvērt",
    menu_settings: "Iestatījumi",
    settings_flavour: "(Tikai valodas izvēle netiek saglabāta.)",
    set_theme: "Motīvs",
    set_lowercase: "Mazie burti rezultātā",
    set_enable_cypher: "Ieslēgt pielāgotu šifru",
    set_cypher: "Pielāgotais šifrs",
    set_cypher_import: "Importēt no teksta faila...",
    set_silly: "kkas muļķīgs :3",
    silly_true: "GRIBU ATPAKAĻ",
    silly_false: "Sākt!",
    set_lang: "Valoda",
    menu_credits: "Atzinības",
    cred_version: "Versija",
    cred_creator: "Autors",
    cred_cbo: "Galvenais Pupiņu Virsnieks",
    cred_fav_station: "Mana mīļākā radio stacija",
    cred_sanity: "Pašreizējais prāta līmenis:  Neliels",
    // ??
    cred_source: "Koda avots",
    cred_website: "Mājaslapa",
    cred_funny_img: "smieklīga bildīte",
    hint_input: "Raksti šeit...",
    btn_encode: "Šifrēt tekstu",
    btn_decode: "Atšifrēt tekstu",
    btn_copy: "Kopēt starpliktuvē",
    btn_save: "Saglabāt rezultātu...",
    text: "Teksts",
    err_cypher_phrase: "Šifra kļūda: Katra šifrā esošā rakstzīme nav apzīmēta ar atsevišķu ID.",
    err_file_parse: "Faila kļūda: Neizdevās parsēt doto failu.",
    err_file_select: "Faila kļūda: Neizdevās izvēlēties teksta failu.",
    err_file_location: "Faila kļūda: Kļūdaina faila atrašanās vieta.",
    // the following are unused for now because decode::run() doesn't have access to self/app/whatever
    err_decode_info: "Atšifrēšanas kļūda: Ievadītajam trūkst informācijas.",
    err_decode_v_major: "Atšifrēšanas kļūda: Kļūdaina galvenās versijas vērtība Cargo.toml failā.",
    err_decode_v_minor: "Atšifrēšanas kļūda: Kļūdaina papildversijas vērtība Cargo.toml failā.",
    warn_version_line_1: "Brīdinājums: Teksts iespējami nepareizi atšifrēts atšķirīgu versiju dēļ.",
    warn_version_line_2: "Iekodēts versijā",
    warn_version_line_3: "Dekodēts versijā",
};
