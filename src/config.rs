use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Backend {
    Json,
    MdBook,
}

impl Default for Backend {
    fn default() -> Self {
        Self::Json
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackendMdBook {
    #[serde(default = "BackendMdBook::default_title")]
    pub title: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default = "BackendMdBook::default_language")]
    pub language: String,
    #[serde(default)]
    pub multilingual: bool,
    #[serde(default)]
    pub build: bool,
    #[serde(default)]
    pub cleanup: bool,
    #[serde(default)]
    pub header: Option<PathBuf>,
    #[serde(default)]
    pub footer: Option<PathBuf>,
    #[serde(default)]
    pub assets: Option<PathBuf>,
}

impl Default for BackendMdBook {
    fn default() -> Self {
        Self {
            title: Self::default_title(),
            authors: vec![],
            language: Self::default_language(),
            multilingual: false,
            build: false,
            cleanup: false,
            header: None,
            footer: None,
            assets: None,
        }
    }
}

impl BackendMdBook {
    fn default_title() -> String {
        "Documentation".to_owned()
    }

    fn default_language() -> String {
        "en".to_owned()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub input_dirs: Vec<PathBuf>,
    pub output_dir: PathBuf,
    #[serde(default)]
    pub backend: Backend,
    #[serde(default)]
    pub settings: Settings,
    pub backend_mdbook: Option<BackendMdBook>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub show_all: bool,
    #[serde(default)]
    pub document_protected: bool,
    #[serde(default)]
    pub document_private: bool,
}
