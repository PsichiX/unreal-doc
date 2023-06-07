use crate::config::Settings;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub type Type = String;
pub type Template = String;

fn replace_self_names(content: &str, owner: &str) -> String {
    content.replace("$Self$", owner)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Proxy<T> {
    #[serde(default)]
    pub tags: HashSet<String>,
    #[serde(default)]
    pub item: T,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Document {
    #[serde(default)]
    pub enums: Vec<Enum>,
    #[serde(default)]
    pub classes: Vec<StructClass>,
    #[serde(default)]
    pub structs: Vec<StructClass>,
    #[serde(default)]
    pub functions: Vec<Function>,
    #[serde(default)]
    pub book: HashMap<String, String>,
    #[serde(default)]
    pub snippets: HashMap<String, String>,
    #[serde(skip)]
    pub proxy_functions: Vec<Proxy<Function>>,
    #[serde(skip)]
    pub proxy_properties: Vec<Proxy<Property>>,
}

impl Document {
    pub fn sort_items_by_name(&mut self) {
        for item in &mut self.classes {
            item.sort_items_by_name();
        }
        for item in &mut self.structs {
            item.sort_items_by_name();
        }

        self.enums.sort_by(|a, b| a.name.cmp(&b.name));
        self.classes.sort_by(|a, b| a.name.cmp(&b.name));
        self.structs.sort_by(|a, b| a.name.cmp(&b.name));
        self.functions.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn resolve_injects(&mut self) {
        let proxy_functions = std::mem::take(&mut self.proxy_functions);
        let proxy_properties = std::mem::take(&mut self.proxy_properties);
        for item in &mut self.structs {
            item.resolve_injects(&proxy_functions, &proxy_properties);
        }
    }

    pub fn resolve_self_names_in_docs(&mut self) {
        for item in &mut self.enums {
            item.resolve_self_names_in_docs();
        }
        for item in &mut self.classes {
            item.resolve_self_names_in_docs();
        }
        for item in &mut self.structs {
            item.resolve_self_names_in_docs();
        }
        for item in &mut self.functions {
            item.resolve_self_names_in_docs(None);
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Specifiers {
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    #[serde(default)]
    pub meta: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Attribute {
    Single(String),
    Pair { key: String, value: String },
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Private,
    Protected,
    #[default]
    Public,
}

impl Visibility {
    pub fn can_export(self, settings: &Settings) -> bool {
        match self {
            Self::Public => true,
            Self::Protected => settings.document_protected,
            Self::Private => settings.document_private,
        }
    }

    pub fn signature(self) -> String {
        match self {
            Self::Public => "public".to_owned(),
            Self::Protected => "protected".to_owned(),
            Self::Private => "private".to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Enum {
    #[serde(default)]
    pub specifiers: Option<Specifiers>,
    pub name: String,
    #[serde(default)]
    pub variants: Vec<String>,
    #[serde(default)]
    pub doc_comments: Option<String>,
}

impl Enum {
    pub fn can_export(&self, settings: &Settings) -> bool {
        settings.show_all || self.doc_comments.is_some()
    }

    pub fn signature(&self) -> String {
        let variants = self
            .variants
            .iter()
            .map(|v| format!("    {}", v))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("enum class {} : uint8 {{\n{}\n}};", self.name, variants)
    }

    pub fn resolve_self_names_in_docs(&mut self) {
        if let Some(content) = &mut self.doc_comments {
            *content = replace_self_names(content, &self.name);
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructClassMode {
    #[default]
    Struct,
    Class,
}

impl StructClassMode {
    pub fn default_visibility(self) -> Visibility {
        match self {
            Self::Struct => Visibility::Public,
            Self::Class => Visibility::Private,
        }
    }

    pub fn signature(self) -> String {
        match self {
            Self::Struct => "struct".to_owned(),
            Self::Class => "class".to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StructClass {
    #[serde(default)]
    pub specifiers: Option<Specifiers>,
    #[serde(default)]
    pub api: Option<String>,
    pub mode: StructClassMode,
    pub name: String,
    #[serde(default)]
    pub inherits: Vec<(Visibility, String)>,
    #[serde(default)]
    pub template: Option<Template>,
    #[serde(default)]
    pub properties: Vec<Property>,
    #[serde(default)]
    pub methods: Vec<Function>,
    #[serde(default)]
    pub doc_comments: Option<String>,
    #[serde(skip)]
    pub injects: HashSet<String>,
}

impl StructClass {
    pub fn can_export(&self, settings: &Settings) -> bool {
        settings.show_all
            || self.doc_comments.is_some()
            || self.properties.iter().any(|e| e.can_export(settings))
            || self.methods.iter().any(|e| e.can_export(settings))
    }

    pub fn signature(&self) -> String {
        let mut result = String::new();
        if let Some(template) = &self.template {
            result.push_str(template);
            result.push('\n');
        }
        result.push_str(&self.mode.signature());
        result.push(' ');
        if let Some(api) = &self.api {
            result.push_str(api);
            result.push(' ');
        }
        result.push_str(&self.name);
        if !self.inherits.is_empty() {
            for (i, (visibility, name)) in self.inherits.iter().enumerate() {
                result.push('\n');
                result.push_str("    ");
                if i == 0 {
                    result.push_str(": ");
                } else {
                    result.push_str(", ");
                }
                result.push_str(&visibility.signature());
                result.push(' ');
                result.push_str(name);
            }
        }
        result.push(';');
        result
    }

    pub fn sort_items_by_name(&mut self) {
        self.properties.sort_by(|a, b| a.name.cmp(&b.name));
        self.methods.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn resolve_injects(
        &mut self,
        functions: &[Proxy<Function>],
        properties: &[Proxy<Property>],
    ) {
        let tags = std::mem::take(&mut self.injects);
        for item in functions {
            if tags.iter().any(|tag| item.tags.contains(tag)) {
                self.methods.push(item.item.to_owned());
            }
        }
        for item in properties {
            if tags.iter().any(|tag| item.tags.contains(tag)) {
                self.properties.push(item.item.to_owned());
            }
        }
    }

    pub fn resolve_self_names_in_docs(&mut self) {
        if let Some(content) = &mut self.doc_comments {
            *content = replace_self_names(content, &self.name);
        }
        for item in &mut self.properties {
            item.resolve_self_names_in_docs(&self.name);
        }
        for item in &mut self.methods {
            item.resolve_self_names_in_docs(Some(&self.name));
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum PropertyArray {
    #[default]
    None,
    Unsized,
    Sized(String),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Property {
    #[serde(default)]
    pub specifiers: Option<Specifiers>,
    pub name: String,
    pub value_type: Type,
    #[serde(default)]
    pub array: PropertyArray,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub visibility: Visibility,
    #[serde(default)]
    pub is_static: bool,
    #[serde(default)]
    pub doc_comments: Option<String>,
}

impl Property {
    pub fn can_export(&self, settings: &Settings) -> bool {
        self.doc_comments.is_some() && self.visibility.can_export(settings)
    }

    pub fn signature(&self) -> String {
        let mut result = self.visibility.signature();
        result.push_str(":\n");
        if self.is_static {
            result.push_str("static ");
        }
        result.push_str(&self.value_type);
        result.push(' ');
        result.push_str(&self.name);
        match &self.array {
            PropertyArray::None => {}
            PropertyArray::Unsized => result.push_str("[]"),
            PropertyArray::Sized(size) => result.push_str(&format!("[{}]", size)),
        }
        result.push(';');
        result
    }

    pub fn resolve_self_names_in_docs(&mut self, owner: &str) {
        if let Some(content) = &mut self.doc_comments {
            *content = replace_self_names(content, owner);
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Function {
    #[serde(default)]
    pub specifiers: Option<Specifiers>,
    pub name: String,
    pub return_type: Option<Type>,
    #[serde(default)]
    pub visibility: Visibility,
    #[serde(default)]
    pub template: Option<Template>,
    #[serde(default)]
    pub arguments: Vec<Argument>,
    #[serde(default)]
    pub is_static: bool,
    #[serde(default)]
    pub is_virtual: bool,
    #[serde(default)]
    pub is_const_this: bool,
    #[serde(default)]
    pub is_override: bool,
    #[serde(default)]
    pub doc_comments: Option<String>,
}

impl Function {
    pub fn can_export(&self, settings: &Settings) -> bool {
        self.doc_comments.is_some() && self.visibility.can_export(settings)
    }

    pub fn signature(&self) -> String {
        let mut result = self.visibility.signature();
        result.push_str(":\n");
        if let Some(template) = &self.template {
            result.push_str(template);
            result.push('\n');
        }
        if self.is_static {
            result.push_str("static ");
        }
        if self.is_virtual {
            result.push_str("virtual ");
        }
        if let Some(return_type) = &self.return_type {
            result.push_str(return_type);
            result.push(' ');
        }
        result.push_str(&self.name);
        result.push('(');
        for (i, argument) in self.arguments.iter().enumerate() {
            result.push_str("\n    ");
            result.push_str(&argument.signature());
            if i < self.arguments.len() - 1 {
                result.push(',');
            } else {
                result.push('\n');
            }
        }
        result.push(')');
        if self.is_const_this {
            result.push_str(" const");
        }
        if self.is_override {
            result.push_str(" override");
        }
        result.push(';');
        result
    }

    pub fn resolve_self_names_in_docs(&mut self, owner: Option<&str>) {
        if let (Some(owner), Some(content)) = (owner, &mut self.doc_comments) {
            *content = replace_self_names(content, owner);
        }
        for item in &mut self.arguments {
            item.resolve_self_names_in_docs(owner);
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Argument {
    #[serde(default)]
    pub name: Option<String>,
    pub value_type: Type,
    #[serde(default)]
    pub default_value: Option<String>,
    #[serde(default)]
    pub doc_comments: Option<String>,
}

impl Argument {
    pub fn signature(&self) -> String {
        let mut result = self.value_type.to_owned();
        if let Some(name) = &self.name {
            result.push(' ');
            result.push_str(name);
        }
        if let Some(default_value) = &self.default_value {
            result.push_str(" = ");
            result.push_str(default_value);
        }
        result
    }

    pub fn resolve_self_names_in_docs(&mut self, owner: Option<&str>) {
        if let (Some(owner), Some(content)) = (owner, &mut self.doc_comments) {
            *content = replace_self_names(content, owner);
        }
    }
}
