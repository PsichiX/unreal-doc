use crate::{config::Config, document::Document, ensure_dir};
use std::fs::write;

pub fn bake_json(document: &Document, config: &Config) {
    let content =
        serde_json::to_string_pretty(&document).expect("Could not serialize document into JSON!");
    let path = config.output_dir.join("documentation.json");
    ensure_dir(&path);
    write(&path, content)
        .unwrap_or_else(|_| panic!("Could not write document into JSON file: {:?}", path));
}
