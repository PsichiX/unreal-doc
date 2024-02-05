#[macro_use]
extern crate pest_derive;

mod ast;
mod backends;
mod config;
mod document;

use crate::{
    ast::unreal_cpp_header::parse_unreal_cpp_header,
    backends::{json::bake_json, mdbook::bake_mdbook},
    config::*,
    document::Document,
};
use clap::{Arg, Command};
use std::{
    fs::{create_dir_all, read_to_string},
    io::Result,
    path::{Path, PathBuf},
};

const BOM: char = '\u{FEFF}';

fn main() {
    let matches = Command::new(env!("CARGO_BIN_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .takes_value(true)
                .value_name("FILE")
                .default_value("./UnrealDoc.toml")
                .help("UnrealDoc.toml config file"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .value_name("DIR")
                .required(false)
                .help("Force documentation output to specified directory"),
        )
        .get_matches();

    let input = matches
        .value_of("input")
        .expect("No `input` argument provided!");
    let input = PathBuf::from(input);
    let output = matches.value_of("output").map(|path| PathBuf::from(path));
    let output = output.as_ref().map(|path| path.as_path());
    let (mut config, dir) = load_config(&input, output);

    let mut document = Document::default();
    for path in &config.input_dirs {
        document_path(&path, &path, &mut document, &config.settings);
    }
    document.resolve_injects();
    document.resolve_self_names_in_docs();
    document.sort_items_by_name();

    match config.backend {
        Backend::Json => bake_json(&document, &config),
        Backend::MdBook => {
            if let Ok(site_url) = std::env::var("UNREAL_DOC_MDBOOK_SITE_URL") {
                if let Some(config) = config.backend_mdbook.as_mut() {
                    config.site_url = Some(site_url.to_owned());
                }
            }
            bake_mdbook(&document, &config, &dir)
        }
    }
}

fn load_config(input: &Path, output: Option<&Path>) -> (Config, PathBuf) {
    let content =
        read_file(input).unwrap_or_else(|_| panic!("Input config file not found: {:?}", input));
    let mut config = toml::from_str::<Config>(&content)
        .unwrap_or_else(|_| panic!("Could not parse config file:\n{}", content));
    let mut dir = PathBuf::from(input);
    if dir.is_file() {
        dir.pop();
    }
    for path in &mut config.dependencies {
        if path.is_relative() {
            *path = dir.join(&path);
        }
    }
    for path in &mut config.input_dirs {
        if path.is_relative() {
            *path = dir.join(&path);
        }
    }
    if let Some(output) = output {
        config.output_dir = output.into();
    }
    if config.output_dir.is_relative() {
        config.output_dir = dir.join(&config.output_dir);
    }
    for path in &config.dependencies {
        let inputs = load_config(path, None).0.input_dirs;
        config.input_dirs.extend(inputs);
    }
    (config, dir)
}

fn document_path(path: &Path, root: &Path, document: &mut Document, settings: &Settings) {
    if path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "h" {
                let path = path.canonicalize().unwrap_or_else(|_| path.to_owned());
                let content =
                    read_file(&path).unwrap_or_else(|_| panic!("Could not read file: {:?}", &path));
                document_header(&path, &content, document, settings);
            } else if ext == "md" {
                let content =
                    read_file(path).unwrap_or_else(|_| panic!("Could not read file: {:?}", path));
                let root = root.to_string_lossy().into_owned();
                let path = path.to_string_lossy().into_owned();
                let pat: &[_] = &['/', '\\'];
                let relative = path
                    .trim_start_matches(&root)
                    .trim_start_matches(pat)
                    .replace("\\", "/")
                    .to_owned();
                document.book.insert(relative, content);
            } else if let Some(file_name) = path.file_name() {
                if file_name == "index.txt" {
                    let content = read_file(path)
                        .unwrap_or_else(|_| panic!("Could not read file: {:?}", path));
                    let root = root.to_string_lossy().into_owned();
                    let path = path.to_string_lossy().into_owned();
                    let pat: &[_] = &['/', '\\'];
                    let relative = path
                        .trim_start_matches(&root)
                        .trim_start_matches(pat)
                        .replace("\\", "/")
                        .to_owned();
                    document.book.insert(relative, content);
                }
            }
        }
    } else if path.is_dir() {
        for entry in path
            .read_dir()
            .unwrap_or_else(|_| panic!("Could not read directory: {:?}", path))
        {
            let path = entry.expect("Could not read directory entry!").path();
            document_path(&path, root, document, settings);
        }
    }
}

fn document_header(path: &Path, content: &str, document: &mut Document, settings: &Settings) {
    parse_unreal_cpp_header(content, document, settings).unwrap_or_else(|error| {
        panic!(
            "Could not parse Unreal C++ header file content!\nFile: {:?}\nError:\n{}",
            path, error
        )
    });
}

fn ensure_dir(path: &Path) {
    if path.is_dir() {
        let _ = create_dir_all(path);
    } else {
        let mut path = path.to_path_buf();
        path.pop();
        let _ = create_dir_all(&path);
    }
}

fn read_file(path: impl AsRef<Path>) -> Result<String> {
    let content = read_to_string(path)?;
    if content.starts_with(BOM) {
        Ok(content.chars().skip(1).collect())
    } else {
        Ok(content)
    }
}
