use crate::error::{FilefireError, Result};
use crate::document::{
    DocumentFormat, DocumentInfo, ProcessingStats, DocumentSecurity, 
    DocumentValidation, ValidationRule, ValidationResult, SecurityAnalysis
};
use crate::document::pdf::{PdfProcessor, ProcessedPdfDocument};
use crate::document::office::{OfficeProcessor, ProcessedOfficeDocument};
use crate::document::image::{ImageProcessor, ProcessedImageDocument};
use crate::document::text::{TextProcessor, ProcessedTextDocument};
use std::collections::HashMap;
use tokio::task;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Enterprise document processing engine with advanced capabilities
pub struct DocumentProcessingEngine {
    pdf_processor: Arc<RwLock<PdfProcessor>>,
    office_processor: Arc<RwLock<OfficeProcessor>>,
    image_processor: Arc<RwLock<ImageProcessor>>,
    text_processor: Arc<RwLock<TextProcessor>>,
    security_analyzer: SecurityAnalyzer,
    format_detector: FormatDetector,
    validation_engine: ValidationEngine,
    performance_monitor: PerformanceMonitor,
}

impl DocumentProcessingEngine {
    /// Create a new document processing engine
    pub fn new() -> Self {
        Self {
            pdf_processor: Arc::new(RwLock::new(PdfProcessor::new())),
            office_processor: Arc::new(RwLock::new(OfficeProcessor::new())),
            image_processor: Arc::new(RwLock::new(ImageProcessor::new())),
            text_processor: Arc::new(RwLock::new(TextProcessor::new())),
            security_analyzer: SecurityAnalyzer::new(),
            format_detector: FormatDetector::new(),
            validation_engine: ValidationEngine::new(),
            performance_monitor: PerformanceMonitor::new(),
        }
    }
    
    /// Process document with comprehensive analysis and security validation
    pub async fn process_document(
        &self,
        content: &[u8],
        filename: Option<&str>,
        password: Option<&str>,
        validation_rules: Vec<ValidationRule>,
    ) -> Result<ProcessedDocument> {
        let start_time = std::time::Instant::now();
        
        // Detect document format
        let detected_format = self.format_detector.detect_format(content, filename).await?;
        
        // Perform security analysis
        let security_analysis = self.security_analyzer.analyze_content(content, &detected_format).await?;
        
        // Validate document against rules
        let validation_result = self.validation_engine.validate_document(content, &detected_format, &validation_rules).await?;
        
        // Process document based on format
        let processing_result = match detected_format {
            DocumentFormat::Pdf | DocumentFormat::PdfA1 | DocumentFormat::PdfA2 | 
            DocumentFormat::PdfA3 | DocumentFormat::PdfUA => {
                let mut processor = self.pdf_processor.write().await;
                let pdf_result = processor.process_document(content, password).await?;
                ProcessingResult::Pdf(pdf_result)
            }
            DocumentFormat::Docx | DocumentFormat::Doc | DocumentFormat::Xlsx | 
            DocumentFormat::Xls | DocumentFormat::Pptx | DocumentFormat::Ppt => {
                let mut processor = self.office_processor.write().await;
                let office_result = processor.process_document(content, detected_format.clone()).await?;
                ProcessingResult::Office(office_result)
            }
            DocumentFormat::Jpeg | DocumentFormat::Png | DocumentFormat::Gif | 
            DocumentFormat::Bmp | DocumentFormat::Tiff | DocumentFormat::Webp | 
            DocumentFormat::Svg | DocumentFormat::Heic | DocumentFormat::Avif => {
                let mut processor = self.image_processor.write().await;
                let image_result = processor.process_document(content, detected_format.clone()).await?;
                ProcessingResult::Image(image_result)
            }
            DocumentFormat::PlainText | DocumentFormat::Rtf | DocumentFormat::Markdown | 
            DocumentFormat::Html | DocumentFormat::Csv | DocumentFormat::Json | 
            DocumentFormat::Xml | DocumentFormat::Yaml => {
                let mut processor = self.text_processor.write().await;
                let text_result = processor.process_document(content, detected_format.clone()).await?;
                ProcessingResult::Text(text_result)
            }
            _ => {
                return Err(FilefireError::UnsupportedFormat(format!("Unsupported format: {:?}", detected_format)));
            }
        };
        
        // Calculate overall processing statistics
        let processing_time = start_time.elapsed();
        let overall_stats = ProcessingStats {
            processing_time_ms: processing_time.as_millis() as u64,
            memory_used_mb: self.performance_monitor.get_memory_usage(),
            pages_processed: self.get_pages_processed(&processing_result),
            text_extracted_chars: self.get_text_chars(&processing_result),
            images_extracted: self.get_images_extracted(&processing_result),
            annotations_found: self.get_annotations_found(&processing_result),
            errors_encountered: validation_result.errors.len() as u32,
            warnings_generated: validation_result.warnings.len() as u32,
        };
        
        Ok(ProcessedDocument {
            format: detected_format,
            security_analysis,
            validation_result,
            processing_result,
            overall_stats,
            filename: filename.map(|s| s.to_string()),
            processed_at: chrono::Utc::now(),
        })
    }
    
    /// Get pages processed from processing result
    fn get_pages_processed(&self, result: &ProcessingResult) -> u32 {
        match result {
            ProcessingResult::Pdf(pdf) => pdf.stats.pages_processed,
            ProcessingResult::Office(office) => office.stats.pages_processed,
            ProcessingResult::Image(image) => image.stats.pages_processed,
            ProcessingResult::Text(text) => text.stats.pages_processed,
        }
    }
    
    /// Get text characters extracted from processing result
    fn get_text_chars(&self, result: &ProcessingResult) -> u64 {
        match result {
            ProcessingResult::Pdf(pdf) => pdf.stats.text_extracted_chars,
            ProcessingResult::Office(office) => office.stats.text_extracted_chars,
            ProcessingResult::Image(image) => image.stats.text_extracted_chars,
            ProcessingResult::Text(text) => text.stats.text_extracted_chars,
        }
    }
    
    /// Get images extracted from processing result
    fn get_images_extracted(&self, result: &ProcessingResult) -> u32 {
        match result {
            ProcessingResult::Pdf(pdf) => pdf.stats.images_extracted,
            ProcessingResult::Office(office) => office.stats.images_extracted,
            ProcessingResult::Image(image) => image.stats.images_extracted,
            ProcessingResult::Text(text) => text.stats.images_extracted,
        }
    }
    
    /// Get annotations found from processing result
    fn get_annotations_found(&self, result: &ProcessingResult) -> u32 {
        match result {
            ProcessingResult::Pdf(pdf) => pdf.stats.annotations_found,
            ProcessingResult::Office(office) => office.stats.annotations_found,
            ProcessingResult::Image(image) => image.stats.annotations_found,
            ProcessingResult::Text(text) => text.stats.annotations_found,
        }
    }
    
    /// Extract text content from any document type
    pub async fn extract_text(&self, content: &[u8], format: Option<DocumentFormat>) -> Result<String> {
        let detected_format = if let Some(fmt) = format {
            fmt
        } else {
            self.format_detector.detect_format(content, None).await?
        };
        
        match detected_format {
            DocumentFormat::Pdf | DocumentFormat::PdfA1 | DocumentFormat::PdfA2 | 
            DocumentFormat::PdfA3 | DocumentFormat::PdfUA => {
                let mut processor = self.pdf_processor.write().await;
                let result = processor.process_document(content, None).await?;
                Ok(result.text_content)
            }
            DocumentFormat::Docx | DocumentFormat::Doc | DocumentFormat::Xlsx | 
            DocumentFormat::Xls | DocumentFormat::Pptx | DocumentFormat::Ppt => {
                let mut processor = self.office_processor.write().await;
                let result = processor.process_document(content, detected_format).await?;
                Ok(result.text_content)
            }
            DocumentFormat::PlainText | DocumentFormat::Rtf | DocumentFormat::Markdown | 
            DocumentFormat::Html | DocumentFormat::Csv | DocumentFormat::Json | 
            DocumentFormat::Xml | DocumentFormat::Yaml => {
                let mut processor = self.text_processor.write().await;
                let result = processor.process_document(content, detected_format).await?;
                Ok(result.text_content)
            }
            _ => Err(FilefireError::UnsupportedFormat(format!("Text extraction not supported for format: {:?}", detected_format))),
        }
    }
    
    /// Perform security scan on document
    pub async fn security_scan(&self, content: &[u8], format: Option<DocumentFormat>) -> Result<SecurityAnalysis> {
        let detected_format = if let Some(fmt) = format {
            fmt
        } else {
            self.format_detector.detect_format(content, None).await?
        };
        
        self.security_analyzer.analyze_content(content, &detected_format).await
    }
    
    /// Validate document against enterprise policies
    pub async fn validate_document(&self, content: &[u8], rules: Vec<ValidationRule>) -> Result<ValidationResult> {
        let detected_format = self.format_detector.detect_format(content, None).await?;
        self.validation_engine.validate_document(content, &detected_format, &rules).await
    }
}

impl Default for DocumentProcessingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Security analyzer for document content
struct SecurityAnalyzer {
    threat_patterns: Vec<ThreatPattern>,
}

impl SecurityAnalyzer {
    fn new() -> Self {
        Self {
            threat_patterns: Self::load_threat_patterns(),
        }
    }
    
    /// Load known threat patterns
    fn load_threat_patterns() -> Vec<ThreatPattern> {
        vec![
            ThreatPattern {
                name: "JavaScript".to_string(),
                pattern: r"<script|javascript:|eval\(|setTimeout\(|setInterval\(".to_string(),
                severity: ThreatSeverity::Medium,
                description: "Potentially malicious JavaScript code".to_string(),
            },
            ThreatPattern {
                name: "External Links".to_string(),
                pattern: r"https?://(?!(?:localhost|127\.0\.0\.1|192\.168\.|10\.|172\.(?:1[6-9]|2[0-9]|3[01])\.))[^\s]+".to_string(),
                severity: ThreatSeverity::Low,
                description: "External links that could be malicious".to_string(),
            },
            ThreatPattern {
                name: "Embedded Files".to_string(),
                pattern: r"/EmbeddedFile|/FileAttachment".to_string(),
                severity: ThreatSeverity::Medium,
                description: "Embedded files that could contain malware".to_string(),
            },
            ThreatPattern {
                name: "Form Actions".to_string(),
                pattern: r"/AA\s|/OpenAction|/JS\s".to_string(),
                severity: ThreatSeverity::High,
                description: "Automatic actions that could be malicious".to_string(),
            },
        ]
    }
    
    /// Analyze document content for security threats
    async fn analyze_content(&self, content: &[u8], format: &DocumentFormat) -> Result<SecurityAnalysis> {
        let content_str = String::from_utf8_lossy(content);
        let mut threats = Vec::new();
        let mut security_score = 100.0;
        
        // Check for threat patterns
        for pattern in &self.threat_patterns {
            if let Ok(regex) = regex::Regex::new(&pattern.pattern) {
                let matches: Vec<_> = regex.find_iter(&content_str).collect();
                if !matches.is_empty() {
                    threats.push(DetectedThreat {
                        threat_type: pattern.name.clone(),
                        severity: pattern.severity.clone(),
                        description: pattern.description.clone(),
                        occurrences: matches.len(),
                        locations: matches.iter().map(|m| m.start()).collect(),
                    });
                    
                    // Reduce security score based on severity
                    let score_reduction = match pattern.severity {
                        ThreatSeverity::Low => 5.0,
                        ThreatSeverity::Medium => 15.0,
                        ThreatSeverity::High => 30.0,
                        ThreatSeverity::Critical => 50.0,
                    };
                    security_score -= score_reduction * matches.len() as f64;
                }
            }
        }
        
        security_score = security_score.max(0.0);
        
        // Determine overall risk level
        let risk_level = match security_score as u32 {
            80..=100 => RiskLevel::Low,
            60..=79 => RiskLevel::Medium,
            40..=59 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };
        
        Ok(SecurityAnalysis {
            risk_level,
            security_score,
            threats_detected: threats,
            is_encrypted: self.check_encryption(content, format).await?,
            has_digital_signature: self.check_digital_signature(content, format).await?,
            contains_macros: self.check_macros(content, format).await?,
            external_references: self.extract_external_references(content).await?,
        })
    }
    
    /// Check if document is encrypted
    async fn check_encryption(&self, content: &[u8], format: &DocumentFormat) -> Result<bool> {
        match format {
            DocumentFormat::Pdf | DocumentFormat::PdfA1 | DocumentFormat::PdfA2 | 
            DocumentFormat::PdfA3 | DocumentFormat::PdfUA => {
                // Check for PDF encryption dictionary
                let content_str = String::from_utf8_lossy(content);
                Ok(content_str.contains("/Encrypt"))
            }
            DocumentFormat::Docx | DocumentFormat::Xlsx | DocumentFormat::Pptx => {
                // Check for Office encryption
                Ok(content.len() > 8 && &content[0..8] == b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1")
            }
            _ => Ok(false),
        }
    }
    
    /// Check for digital signatures
    async fn check_digital_signature(&self, content: &[u8], format: &DocumentFormat) -> Result<bool> {
        match format {
            DocumentFormat::Pdf | DocumentFormat::PdfA1 | DocumentFormat::PdfA2 | 
            DocumentFormat::PdfA3 | DocumentFormat::PdfUA => {
                let content_str = String::from_utf8_lossy(content);
                Ok(content_str.contains("/Sig") || content_str.contains("/DocMDP"))
            }
            _ => Ok(false),
        }
    }
    
    /// Check for macros
    async fn check_macros(&self, content: &[u8], format: &DocumentFormat) -> Result<bool> {
        match format {
            DocumentFormat::Docx | DocumentFormat::Xlsx | DocumentFormat::Pptx => {
                let content_str = String::from_utf8_lossy(content);
                Ok(content_str.contains("vbaProject.bin") || content_str.contains("macros/"))
            }
            _ => Ok(false),
        }
    }
    
    /// Extract external references
    async fn extract_external_references(&self, content: &[u8]) -> Result<Vec<String>> {
        let content_str = String::from_utf8_lossy(content);
        let mut references = Vec::new();
        
        // Look for HTTP/HTTPS URLs
        if let Ok(url_regex) = regex::Regex::new(r"https?://[^\s\)]+") {
            for mat in url_regex.find_iter(&content_str) {
                references.push(mat.as_str().to_string());
            }
        }
        
        // Look for file:// URLs
        if let Ok(file_regex) = regex::Regex::new(r"file://[^\s\)]+") {
            for mat in file_regex.find_iter(&content_str) {
                references.push(mat.as_str().to_string());
            }
        }
        
        Ok(references)
    }
}

/// Format detection engine
struct FormatDetector {
    magic_bytes: HashMap<Vec<u8>, DocumentFormat>,
}

impl FormatDetector {
    fn new() -> Self {
        let mut magic_bytes = HashMap::new();
        
        // PDF formats
        magic_bytes.insert(b"%PDF-".to_vec(), DocumentFormat::Pdf);
        
        // Office formats
        magic_bytes.insert(b"PK\x03\x04".to_vec(), DocumentFormat::Docx); // Will need refinement
        magic_bytes.insert(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1".to_vec(), DocumentFormat::Doc);
        
        // Image formats
        magic_bytes.insert(b"\xff\xd8\xff".to_vec(), DocumentFormat::Jpeg);
        magic_bytes.insert(b"\x89PNG\r\n\x1a\n".to_vec(), DocumentFormat::Png);
        magic_bytes.insert(b"GIF87a".to_vec(), DocumentFormat::Gif);
        magic_bytes.insert(b"GIF89a".to_vec(), DocumentFormat::Gif);
        magic_bytes.insert(b"BM".to_vec(), DocumentFormat::Bmp);
        magic_bytes.insert(b"II*\x00".to_vec(), DocumentFormat::Tiff);
        magic_bytes.insert(b"MM\x00*".to_vec(), DocumentFormat::Tiff);
        magic_bytes.insert(b"RIFF".to_vec(), DocumentFormat::Webp);
        
        Self { magic_bytes }
    }
    
    /// Detect document format from content and filename
    async fn detect_format(&self, content: &[u8], filename: Option<&str>) -> Result<DocumentFormat> {
        // First try magic bytes
        for (magic, format) in &self.magic_bytes {
            if content.len() >= magic.len() && &content[0..magic.len()] == magic {
                return Ok(format.clone());
            }
        }
        
        // Fall back to filename extension
        if let Some(name) = filename {
            if let Some(extension) = std::path::Path::new(name).extension() {
                if let Some(ext_str) = extension.to_str() {
                    return match ext_str.to_lowercase().as_str() {
                        "pdf" => Ok(DocumentFormat::Pdf),
                        "docx" => Ok(DocumentFormat::Docx),
                        "doc" => Ok(DocumentFormat::Doc),
                        "xlsx" => Ok(DocumentFormat::Xlsx),
                        "xls" => Ok(DocumentFormat::Xls),
                        "pptx" => Ok(DocumentFormat::Pptx),
                        "ppt" => Ok(DocumentFormat::Ppt),
                        "jpg" | "jpeg" => Ok(DocumentFormat::Jpeg),
                        "png" => Ok(DocumentFormat::Png),
                        "gif" => Ok(DocumentFormat::Gif),
                        "bmp" => Ok(DocumentFormat::Bmp),
                        "tiff" | "tif" => Ok(DocumentFormat::Tiff),
                        "webp" => Ok(DocumentFormat::Webp),
                        "svg" => Ok(DocumentFormat::Svg),
                        "txt" => Ok(DocumentFormat::PlainText),
                        "rtf" => Ok(DocumentFormat::Rtf),
                        "md" => Ok(DocumentFormat::Markdown),
                        "html" | "htm" => Ok(DocumentFormat::Html),
                        "csv" => Ok(DocumentFormat::Csv),
                        "json" => Ok(DocumentFormat::Json),
                        "xml" => Ok(DocumentFormat::Xml),
                        "yaml" | "yml" => Ok(DocumentFormat::Yaml),
                        _ => Err(FilefireError::UnsupportedFormat(format!("Unknown extension: {}", ext_str))),
                    };
                }
            }
        }
        
        // Try to detect text-based formats by content
        if let Ok(content_str) = std::str::from_utf8(content) {
            let trimmed = content_str.trim();
            
            if trimmed.starts_with("<!DOCTYPE html") || trimmed.starts_with("<html") {
                return Ok(DocumentFormat::Html);
            }
            
            if trimmed.starts_with('{') && trimmed.ends_with('}') {
                return Ok(DocumentFormat::Json);
            }
            
            if trimmed.starts_with("<?xml") || (trimmed.starts_with('<') && trimmed.ends_with('>')) {
                return Ok(DocumentFormat::Xml);
            }
            
            if trimmed.contains("---\n") || trimmed.starts_with("---") {
                return Ok(DocumentFormat::Yaml);
            }
            
            if trimmed.contains(',') && trimmed.lines().next().unwrap_or("").contains(',') {
                return Ok(DocumentFormat::Csv);
            }
            
            if trimmed.contains('#') || trimmed.contains("**") || trimmed.contains("```") {
                return Ok(DocumentFormat::Markdown);
            }
            
            return Ok(DocumentFormat::PlainText);
        }
        
        Err(FilefireError::UnsupportedFormat("Unable to detect document format".to_string()))
    }
}

/// Document validation engine
struct ValidationEngine;

impl ValidationEngine {
    fn new() -> Self {
        Self
    }
    
    /// Validate document against rules
    async fn validate_document(
        &self,
        content: &[u8],
        format: &DocumentFormat,
        rules: &[ValidationRule],
    ) -> Result<ValidationResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut info = Vec::new();
        
        for rule in rules {
            match self.apply_rule(content, format, rule).await {
                Ok(result) => {
                    match result.severity {
                        ValidationSeverity::Error => errors.push(result),
                        ValidationSeverity::Warning => warnings.push(result),
                        ValidationSeverity::Info => info.push(result),
                    }
                }
                Err(e) => {
                    errors.push(ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: format!("Validation rule failed: {}", e),
                        severity: ValidationSeverity::Error,
                        location: None,
                    });
                }
            }
        }
        
        let is_valid = errors.is_empty();
        
        Ok(ValidationResult {
            is_valid,
            errors,
            warnings,
            info,
        })
    }
    
    /// Apply individual validation rule
    async fn apply_rule(
        &self,
        content: &[u8],
        format: &DocumentFormat,
        rule: &ValidationRule,
    ) -> Result<ValidationMessage> {
        // This is a simplified implementation
        // In production, you'd have a comprehensive rule engine
        
        let message = match rule.rule_type {
            ValidationRuleType::MaxFileSize => {
                if content.len() > rule.threshold.unwrap_or(10_000_000) as usize {
                    ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: format!("File size {} exceeds maximum allowed size", content.len()),
                        severity: ValidationSeverity::Error,
                        location: None,
                    }
                } else {
                    ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: "File size within acceptable limits".to_string(),
                        severity: ValidationSeverity::Info,
                        location: None,
                    }
                }
            }
            ValidationRuleType::AllowedFormats => {
                if rule.allowed_formats.as_ref().map_or(true, |formats| formats.contains(format)) {
                    ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: "Document format is allowed".to_string(),
                        severity: ValidationSeverity::Info,
                        location: None,
                    }
                } else {
                    ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: format!("Document format {:?} is not allowed", format),
                        severity: ValidationSeverity::Error,
                        location: None,
                    }
                }
            }
            ValidationRuleType::NoMacros => {
                // Simple macro detection
                let content_str = String::from_utf8_lossy(content);
                if content_str.contains("vbaProject.bin") || content_str.contains("macros/") {
                    ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: "Document contains macros which are not allowed".to_string(),
                        severity: ValidationSeverity::Error,
                        location: None,
                    }
                } else {
                    ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: "No macros detected".to_string(),
                        severity: ValidationSeverity::Info,
                        location: None,
                    }
                }
            }
            ValidationRuleType::RequireEncryption => {
                // Simple encryption detection
                let is_encrypted = match format {
                    DocumentFormat::Pdf | DocumentFormat::PdfA1 | DocumentFormat::PdfA2 | 
                    DocumentFormat::PdfA3 | DocumentFormat::PdfUA => {
                        String::from_utf8_lossy(content).contains("/Encrypt")
                    }
                    _ => false,
                };
                
                if rule.required.unwrap_or(false) && !is_encrypted {
                    ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: "Document encryption is required but not present".to_string(),
                        severity: ValidationSeverity::Error,
                        location: None,
                    }
                } else {
                    ValidationMessage {
                        rule_name: rule.name.clone(),
                        message: if is_encrypted { "Document is encrypted" } else { "Document is not encrypted" }.to_string(),
                        severity: ValidationSeverity::Info,
                        location: None,
                    }
                }
            }
        };
        
        Ok(message)
    }
}

/// Performance monitoring
struct PerformanceMonitor;

impl PerformanceMonitor {
    fn new() -> Self {
        Self
    }
    
    fn get_memory_usage(&self) -> f64 {
        // Placeholder implementation
        0.0
    }
}

/// Processing result enum containing format-specific results
#[derive(Debug, Clone)]
pub enum ProcessingResult {
    Pdf(ProcessedPdfDocument),
    Office(ProcessedOfficeDocument),
    Image(ProcessedImageDocument),
    Text(ProcessedTextDocument),
}

/// Complete processed document with all analysis results
#[derive(Debug, Clone)]
pub struct ProcessedDocument {
    pub format: DocumentFormat,
    pub security_analysis: SecurityAnalysis,
    pub validation_result: ValidationResult,
    pub processing_result: ProcessingResult,
    pub overall_stats: ProcessingStats,
    pub filename: Option<String>,
    pub processed_at: chrono::DateTime<chrono::Utc>,
}

/// Threat pattern for security analysis
#[derive(Debug, Clone)]
struct ThreatPattern {
    name: String,
    pattern: String,
    severity: ThreatSeverity,
    description: String,
}

/// Threat severity levels
#[derive(Debug, Clone, PartialEq)]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Detected security threat
#[derive(Debug, Clone)]
pub struct DetectedThreat {
    pub threat_type: String,
    pub severity: ThreatSeverity,
    pub description: String,
    pub occurrences: usize,
    pub locations: Vec<usize>,
}

/// Risk level assessment
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Validation rule types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationRuleType {
    MaxFileSize,
    AllowedFormats,
    NoMacros,
    RequireEncryption,
}

/// Validation message severity
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Validation message
#[derive(Debug, Clone)]
pub struct ValidationMessage {
    pub rule_name: String,
    pub message: String,
    pub severity: ValidationSeverity,
    pub location: Option<String>,
}