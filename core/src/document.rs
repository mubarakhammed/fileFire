use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::{metadata::DocumentMetadata, error::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentFormat {
    Pdf,
    Docx,
    Xlsx,
    Pptx,
    Jpeg,
    Png,
    Tiff,
    Unknown(String),
}

impl DocumentFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "pdf" => Self::Pdf,
            "docx" => Self::Docx,
            "xlsx" => Self::Xlsx,
            "pptx" => Self::Pptx,
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            "tiff" | "tif" => Self::Tiff,
            _ => Self::Unknown(ext.to_string()),
        }
    }
    
    pub fn mime_type(&self) -> &str {
        match self {
            Self::Pdf => "application/pdf",
            Self::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            Self::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Self::Pptx => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Tiff => "image/tiff",
            Self::Unknown(_) => "application/octet-stream",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub page: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub content: String,
    pub annotation_type: AnnotationType,
    pub author: Option<String>,
    pub created_at: String,
    pub modified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnnotationType {
    Text,
    Highlight,
    Underline,
    Strikethrough,
    Note,
    Drawing,
    Stamp,
    Link,
}

#[derive(Debug)]
pub struct Document {
    pub metadata: DocumentMetadata,
    pub format: DocumentFormat,
    pub content: Vec<u8>,
    pub annotations: Vec<Annotation>,
    pub is_modified: bool,
}

impl Document {
    pub fn new(content: Vec<u8>, format: DocumentFormat) -> Self {
        Self {
            metadata: DocumentMetadata::default(),
            format,
            content,
            annotations: Vec::new(),
            is_modified: false,
        }
    }
    
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = tokio::fs::read(path).await?;
        
        let format = path.extension()
            .and_then(|ext| ext.to_str())
            .map(DocumentFormat::from_extension)
            .unwrap_or(DocumentFormat::Unknown("".to_string()));
            
        let mut document = Self::new(content, format.clone());
        
        // Extract metadata based on format
        document.metadata = match format {
            DocumentFormat::Pdf => extract_pdf_metadata(&document.content)?,
            _ => {
                let mut metadata = DocumentMetadata::default();
                metadata.file_size = document.content.len() as u64;
                metadata.mime_type = format.mime_type().to_string();
                metadata
            }
        };
        
        Ok(document)
    }
    
    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.push(annotation);
        self.is_modified = true;
    }
    
    pub fn remove_annotation(&mut self, annotation_id: &str) -> bool {
        let initial_len = self.annotations.len();
        self.annotations.retain(|a| a.id != annotation_id);
        let removed = self.annotations.len() != initial_len;
        if removed {
            self.is_modified = true;
        }
        removed
    }
    
    pub fn get_annotations_for_page(&self, page: u32) -> Vec<&Annotation> {
        self.annotations.iter().filter(|a| a.page == page).collect()
    }
    
    pub async fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let output = self.render_with_annotations().await?;
        tokio::fs::write(path, output).await?;
        Ok(())
    }
    
    async fn render_with_annotations(&self) -> Result<Vec<u8>> {
        // For now, just return the original content
        // In a real implementation, this would merge annotations into the document
        match self.format {
            DocumentFormat::Pdf => render_pdf_with_annotations(&self.content, &self.annotations),
            _ => Ok(self.content.clone()),
        }
    }
}

fn extract_pdf_metadata(content: &[u8]) -> Result<DocumentMetadata> {
    // Basic PDF metadata extraction
    // In a real implementation, this would use a PDF parsing library
    let mut metadata = DocumentMetadata::default();
    metadata.mime_type = "application/pdf".to_string();
    metadata.file_size = content.len() as u64;
    
    // Try to parse with lopdf for basic metadata
    match lopdf::Document::load_mem(content) {
        Ok(doc) => {
            metadata.page_count = doc.get_pages().len() as u32;
            
            // Extract metadata from document info
            if let Ok(info) = doc.trailer.get(b"Info") {
                if let Ok(info_dict) = info.as_dict() {
                    if let Ok(title) = info_dict.get(b"Title") {
                        if let Ok(title_str) = title.as_str() {
                            metadata.title = Some(title_str.to_string());
                        }
                    }
                    if let Ok(author) = info_dict.get(b"Author") {
                        if let Ok(author_str) = author.as_str() {
                            metadata.author = Some(author_str.to_string());
                        }
                    }
                    if let Ok(subject) = info_dict.get(b"Subject") {
                        if let Ok(subject_str) = subject.as_str() {
                            metadata.subject = Some(subject_str.to_string());
                        }
                    }
                    if let Ok(creator) = info_dict.get(b"Creator") {
                        if let Ok(creator_str) = creator.as_str() {
                            metadata.creator = Some(creator_str.to_string());
                        }
                    }
                    if let Ok(producer) = info_dict.get(b"Producer") {
                        if let Ok(producer_str) = producer.as_str() {
                            metadata.producer = Some(producer_str.to_string());
                        }
                    }
                }
            }
        }
        Err(_) => {
            // Fallback: estimate page count by counting "endobj" occurrences
            let content_str = String::from_utf8_lossy(content);
            metadata.page_count = content_str.matches("/Type /Page").count() as u32;
        }
    }
    
    Ok(metadata)
}

fn render_pdf_with_annotations(_content: &[u8], _annotations: &[Annotation]) -> Result<Vec<u8>> {
    // Placeholder for PDF annotation rendering
    // In a real implementation, this would use a PDF library to merge annotations
    Ok(_content.to_vec())
}