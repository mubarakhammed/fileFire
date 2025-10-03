use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Vec<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub page_count: u32,
    pub file_size: u64,
    pub mime_type: String,
    pub custom_properties: HashMap<String, String>,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            subject: None,
            keywords: Vec::new(),
            creator: None,
            producer: None,
            creation_date: None,
            modification_date: None,
            page_count: 0,
            file_size: 0,
            mime_type: String::new(),
            custom_properties: HashMap::new(),
        }
    }
}

impl DocumentMetadata {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }
    
    pub fn add_keyword(mut self, keyword: String) -> Self {
        self.keywords.push(keyword);
        self
    }
    
    pub fn add_custom_property(mut self, key: String, value: String) -> Self {
        self.custom_properties.insert(key, value);
        self
    }
}