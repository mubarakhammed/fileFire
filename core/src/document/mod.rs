pub mod pdf;
pub mod office;
pub mod image;
pub mod text;
pub mod processor;

use crate::error::{FilefireError, Result};
use crate::metadata::DocumentMetadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use bytes::Bytes;

/// Comprehensive document format enumeration with MIME type support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DocumentFormat {
    // PDF formats
    Pdf,
    PdfA1, // PDF/A-1 (ISO 19005-1)
    PdfA2, // PDF/A-2 (ISO 19005-2)  
    PdfA3, // PDF/A-3 (ISO 19005-3)
    PdfUA, // PDF/UA (Universal Accessibility)
    
    // Microsoft Office formats
    Doc,   // Legacy Word
    Docx,  // Word 2007+
    Docm,  // Word with macros
    Dot,   // Word template
    Dotx,  // Word template 2007+
    Xls,   // Legacy Excel
    Xlsx,  // Excel 2007+
    Xlsm,  // Excel with macros
    Xlt,   // Excel template
    Xltx,  // Excel template 2007+
    Ppt,   // Legacy PowerPoint
    Pptx,  // PowerPoint 2007+
    Pptm,  // PowerPoint with macros
    
    // OpenDocument formats
    Odt,   // OpenDocument Text
    Ods,   // OpenDocument Spreadsheet
    Odp,   // OpenDocument Presentation
    Odg,   // OpenDocument Graphics
    
    // Image formats
    Jpeg,
    Png,
    Tiff,
    Gif,
    Bmp,
    Svg,
    Webp,
    Avif,
    Heic,
    
    // Text formats
    Txt,
    Rtf,
    Html,
    Xml,
    Json,
    Csv,
    Markdown,
    
    // Archive formats
    Zip,
    Rar,
    SevenZ,
    Tar,
    GzTar,
    
    // Other formats
    Epub,
    Mobi,
    Djvu,
    
    Unknown(String),
}

impl DocumentFormat {
    /// Get format from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            // PDF
            "pdf" => Self::Pdf,
            
            // Microsoft Office
            "doc" => Self::Doc,
            "docx" => Self::Docx,
            "docm" => Self::Docm,
            "dot" => Self::Dot,
            "dotx" => Self::Dotx,
            "xls" => Self::Xls,
            "xlsx" => Self::Xlsx,
            "xlsm" => Self::Xlsm,
            "xlt" => Self::Xlt,
            "xltx" => Self::Xltx,
            "ppt" => Self::Ppt,
            "pptx" => Self::Pptx,
            "pptm" => Self::Pptm,
            
            // OpenDocument
            "odt" => Self::Odt,
            "ods" => Self::Ods,
            "odp" => Self::Odp,
            "odg" => Self::Odg,
            
            // Images
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            "tiff" | "tif" => Self::Tiff,
            "gif" => Self::Gif,
            "bmp" => Self::Bmp,
            "svg" => Self::Svg,
            "webp" => Self::Webp,
            "avif" => Self::Avif,
            "heic" => Self::Heic,
            
            // Text
            "txt" => Self::Txt,
            "rtf" => Self::Rtf,
            "html" | "htm" => Self::Html,
            "xml" => Self::Xml,
            "json" => Self::Json,
            "csv" => Self::Csv,
            "md" | "markdown" => Self::Markdown,
            
            // Archives
            "zip" => Self::Zip,
            "rar" => Self::Rar,
            "7z" => Self::SevenZ,
            "tar" => Self::Tar,
            "tar.gz" | "tgz" => Self::GzTar,
            
            // Other
            "epub" => Self::Epub,
            "mobi" => Self::Mobi,
            "djvu" => Self::Djvu,
            
            _ => Self::Unknown(ext.to_string()),
        }
    }
    
    /// Get MIME type for the format
    pub fn mime_type(&self) -> &'static str {
        match self {
            // PDF
            Self::Pdf | Self::PdfA1 | Self::PdfA2 | Self::PdfA3 | Self::PdfUA => "application/pdf",
            
            // Microsoft Office
            Self::Doc => "application/msword",
            Self::Docx => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            Self::Docm => "application/vnd.ms-word.document.macroEnabled.12",
            Self::Dot => "application/msword",
            Self::Dotx => "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
            Self::Xls => "application/vnd.ms-excel",
            Self::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Self::Xlsm => "application/vnd.ms-excel.sheet.macroEnabled.12",
            Self::Xlt => "application/vnd.ms-excel",
            Self::Xltx => "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
            Self::Ppt => "application/vnd.ms-powerpoint",
            Self::Pptx => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            Self::Pptm => "application/vnd.ms-powerpoint.presentation.macroEnabled.12",
            
            // OpenDocument
            Self::Odt => "application/vnd.oasis.opendocument.text",
            Self::Ods => "application/vnd.oasis.opendocument.spreadsheet",
            Self::Odp => "application/vnd.oasis.opendocument.presentation",
            Self::Odg => "application/vnd.oasis.opendocument.graphics",
            
            // Images
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Tiff => "image/tiff",
            Self::Gif => "image/gif",
            Self::Bmp => "image/bmp",
            Self::Svg => "image/svg+xml",
            Self::Webp => "image/webp",
            Self::Avif => "image/avif",
            Self::Heic => "image/heic",
            
            // Text
            Self::Txt => "text/plain",
            Self::Rtf => "application/rtf",
            Self::Html => "text/html",
            Self::Xml => "application/xml",
            Self::Json => "application/json",
            Self::Csv => "text/csv",
            Self::Markdown => "text/markdown",
            
            // Archives
            Self::Zip => "application/zip",
            Self::Rar => "application/vnd.rar",
            Self::SevenZ => "application/x-7z-compressed",
            Self::Tar => "application/x-tar",
            Self::GzTar => "application/gzip",
            
            // Other
            Self::Epub => "application/epub+zip",
            Self::Mobi => "application/x-mobipocket-ebook",
            Self::Djvu => "image/vnd.djvu",
            
            Self::Unknown(_) => "application/octet-stream",
        }
    }
    
    /// Check if format supports text extraction
    pub fn supports_text_extraction(&self) -> bool {
        matches!(self,
            Self::Pdf | Self::PdfA1 | Self::PdfA2 | Self::PdfA3 | Self::PdfUA |
            Self::Doc | Self::Docx | Self::Docm | Self::Dot | Self::Dotx |
            Self::Xls | Self::Xlsx | Self::Xlsm | Self::Xlt | Self::Xltx |
            Self::Ppt | Self::Pptx | Self::Pptm |
            Self::Odt | Self::Ods | Self::Odp |
            Self::Txt | Self::Rtf | Self::Html | Self::Xml | Self::Json | Self::Csv | Self::Markdown |
            Self::Epub
        )
    }
    
    /// Check if format supports annotations
    pub fn supports_annotations(&self) -> bool {
        matches!(self,
            Self::Pdf | Self::PdfA1 | Self::PdfA2 | Self::PdfA3 | Self::PdfUA |
            Self::Docx | Self::Docm | Self::Dotx |
            Self::Pptx | Self::Pptm
        )
    }
    
    /// Check if format supports digital signatures
    pub fn supports_digital_signatures(&self) -> bool {
        matches!(self,
            Self::Pdf | Self::PdfA1 | Self::PdfA2 | Self::PdfA3 | Self::PdfUA |
            Self::Docx | Self::Docm | Self::Dotx |
            Self::Xlsx | Self::Xlsm | Self::Xltx |
            Self::Pptx | Self::Pptm
        )
    }
    
    /// Check if format is encrypted/password protected
    pub fn supports_encryption(&self) -> bool {
        matches!(self,
            Self::Pdf | Self::PdfA1 | Self::PdfA2 | Self::PdfA3 | Self::PdfUA |
            Self::Doc | Self::Docx | Self::Docm | Self::Dot | Self::Dotx |
            Self::Xls | Self::Xlsx | Self::Xlsm | Self::Xlt | Self::Xltx |
            Self::Ppt | Self::Pptx | Self::Pptm |
            Self::Zip | Self::Rar | Self::SevenZ
        )
    }
}

/// Document security level and permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSecurity {
    pub is_encrypted: bool,
    pub requires_password: bool,
    pub has_user_password: bool,
    pub has_owner_password: bool,
    pub permissions: DocumentPermissions,
    pub security_handler: Option<String>,
    pub encryption_algorithm: Option<String>,
    pub key_length: Option<u32>,
}

/// Document permission flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPermissions {
    pub print: bool,
    pub print_high_quality: bool,
    pub modify: bool,
    pub copy: bool,
    pub modify_annotations: bool,
    pub fill_forms: bool,
    pub extract_for_accessibility: bool,
    pub assemble: bool,
}

impl Default for DocumentPermissions {
    fn default() -> Self {
        Self {
            print: true,
            print_high_quality: true,
            modify: true,
            copy: true,
            modify_annotations: true,
            fill_forms: true,
            extract_for_accessibility: true,
            assemble: true,
        }
    }
}

/// Advanced document properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentProperties {
    pub language: Option<String>,
    pub character_count: Option<u64>,
    pub word_count: Option<u64>,
    pub paragraph_count: Option<u32>,
    pub line_count: Option<u32>,
    pub page_layout: Option<String>,
    pub page_mode: Option<String>,
    pub viewer_preferences: HashMap<String, String>,
    pub bookmarks: Vec<DocumentBookmark>,
    pub attachments: Vec<DocumentAttachment>,
    pub forms: Vec<DocumentForm>,
    pub javascript: Vec<String>,
}

/// Document bookmark/outline entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentBookmark {
    pub title: String,
    pub page: u32,
    pub level: u32,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub children: Vec<DocumentBookmark>,
}

/// Document attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentAttachment {
    pub name: String,
    pub description: Option<String>,
    pub mime_type: String,
    pub size: u64,
    pub checksum: String,
    pub creation_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
}

/// Document form field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentForm {
    pub name: String,
    pub field_type: FormFieldType,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub page: u32,
    pub rect: FormRect,
    pub required: bool,
    pub readonly: bool,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormFieldType {
    Text,
    Password,
    Checkbox,
    Radio,
    ComboBox,
    ListBox,
    Button,
    Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Document processing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub processing_time_ms: u64,
    pub memory_used_mb: f64,
    pub pages_processed: u32,
    pub text_extracted_chars: u64,
    pub images_extracted: u32,
    pub annotations_found: u32,
    pub errors_encountered: u32,
    pub warnings_generated: u32,
}

/// Document validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub format_compliant: bool,
    pub accessibility_compliant: bool,
    pub security_issues: Vec<SecurityIssue>,
    pub validation_errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub severity: SecuritySeverity,
    pub issue_type: String,
    pub description: String,
    pub recommendation: String,
    pub page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_code: String,
    pub description: String,
    pub page: Option<u32>,
    pub position: Option<(f64, f64)>,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Warning,
    Error,
    Fatal,
}

/// Main document structure with enterprise features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    pub id: Uuid,
    pub format: DocumentFormat,
    pub metadata: DocumentMetadata,
    pub security: DocumentSecurity,
    pub properties: DocumentProperties,
    pub file_path: Option<String>,
    pub file_size: u64,
    pub checksum_md5: String,
    pub checksum_sha256: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub processing_stats: Option<ProcessingStats>,
    pub validation_result: Option<ValidationResult>,
}

impl DocumentInfo {
    /// Create new document info from file path
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let metadata = fs::metadata(path).await?;
        
        if !metadata.is_file() {
            return Err(FilefireError::InvalidDocument("Path is not a file".to_string()));
        }
        
        let format = path.extension()
            .and_then(|ext| ext.to_str())
            .map(DocumentFormat::from_extension)
            .unwrap_or(DocumentFormat::Unknown("".to_string()));
            
        let file_size = metadata.len();
        let content = fs::read(path).await?;
        
        // Calculate checksums
        let checksum_md5 = format!("{:x}", md5::compute(&content));
        let checksum_sha256 = {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(&content);
            format!("{:x}", hasher.finalize())
        };
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            format,
            metadata: DocumentMetadata::default(),
            security: DocumentSecurity {
                is_encrypted: false,
                requires_password: false,
                has_user_password: false,
                has_owner_password: false,
                permissions: DocumentPermissions::default(),
                security_handler: None,
                encryption_algorithm: None,
                key_length: None,
            },
            properties: DocumentProperties {
                language: None,
                character_count: None,
                word_count: None,
                paragraph_count: None,
                line_count: None,
                page_layout: None,
                page_mode: None,
                viewer_preferences: HashMap::new(),
                bookmarks: Vec::new(),
                attachments: Vec::new(),
                forms: Vec::new(),
                javascript: Vec::new(),
            },
            file_path: Some(path.to_string_lossy().to_string()),
            file_size,
            checksum_md5,
            checksum_sha256,
            created_at: now,
            modified_at: now,
            accessed_at: now,
            processing_stats: None,
            validation_result: None,
        })
    }
    
    /// Create document info from bytes
    pub fn from_bytes(content: Bytes, format: DocumentFormat) -> Self {
        // Calculate checksums
        let checksum_md5 = format!("{:x}", md5::compute(&content));
        let checksum_sha256 = {
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(&content);
            format!("{:x}", hasher.finalize())
        };
        
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            format,
            metadata: DocumentMetadata::default(),
            security: DocumentSecurity {
                is_encrypted: false,
                requires_password: false,
                has_user_password: false,
                has_owner_password: false,
                permissions: DocumentPermissions::default(),
                security_handler: None,
                encryption_algorithm: None,
                key_length: None,
            },
            properties: DocumentProperties {
                language: None,
                character_count: None,
                word_count: None,
                paragraph_count: None,
                line_count: None,
                page_layout: None,
                page_mode: None,
                viewer_preferences: HashMap::new(),
                bookmarks: Vec::new(),
                attachments: Vec::new(),
                forms: Vec::new(),
                javascript: Vec::new(),
            },
            file_path: None,
            file_size: content.len() as u64,
            checksum_md5,
            checksum_sha256,
            created_at: now,
            modified_at: now,
            accessed_at: now,
            processing_stats: None,
            validation_result: None,
        }
    }
}

// Re-export commonly used types from processor module
pub use processor::{
    DocumentProcessingEngine, ProcessedDocument, ProcessingResult,
    DetectedThreat, ThreatSeverity, RiskLevel,
    ValidationRuleType, ValidationSeverity, ValidationMessage,
};