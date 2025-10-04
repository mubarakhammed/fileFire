use crate::error::{FilefireError, Result};
use crate::document::{DocumentFormat, ProcessingStats};
use std::collections::HashMap;
use std::str;

/// Enterprise-grade text processor with advanced analysis
pub struct TextProcessor {
    performance_monitor: PerformanceMonitor,
    language_detector: LanguageDetector,
    encoding_detector: EncodingDetector,
}

impl TextProcessor {
    pub fn new() -> Self {
        Self {
            performance_monitor: PerformanceMonitor::new(),
            language_detector: LanguageDetector::new(),
            encoding_detector: EncodingDetector::new(),
        }
    }
    
    /// Process text document with comprehensive analysis
    pub async fn process_document(&mut self, content: &[u8], format: DocumentFormat) -> Result<ProcessedTextDocument> {
        let start_time = std::time::Instant::now();
        
        // Detect text encoding
        let encoding_info = self.encoding_detector.detect_encoding(content)?;
        
        // Convert to UTF-8 text
        let text_content = self.decode_text(content, &encoding_info)?;
        
        // Analyze text structure
        let structure = self.analyze_text_structure(&text_content);
        
        // Detect language
        let language_info = self.language_detector.detect_language(&text_content)?;
        
        // Perform linguistic analysis
        let linguistic_analysis = self.analyze_linguistics(&text_content)?;
        
        // Extract metadata based on format
        let metadata = self.extract_format_metadata(&text_content, &format)?;
        
        // Analyze readability
        let readability = self.analyze_readability(&text_content)?;
        
        // Detect patterns and structures
        let patterns = self.detect_patterns(&text_content)?;
        
        // Calculate statistics
        let text_stats = self.calculate_text_statistics(&text_content);
        
        // Calculate processing statistics
        let processing_time = start_time.elapsed();
        let stats = ProcessingStats {
            processing_time_ms: processing_time.as_millis() as u64,
            memory_used_mb: self.performance_monitor.get_memory_usage(),
            pages_processed: 1,
            text_extracted_chars: text_content.chars().count() as u64,
            images_extracted: 0,
            annotations_found: 0,
            errors_encountered: 0,
            warnings_generated: 0,
        };
        
        Ok(ProcessedTextDocument {
            format,
            encoding_info,
            text_content,
            structure,
            language_info,
            linguistic_analysis,
            metadata,
            readability,
            patterns,
            text_stats,
            stats,
        })
    }
    
    /// Decode text from bytes using detected encoding
    fn decode_text(&self, content: &[u8], encoding_info: &EncodingInfo) -> Result<String> {
        match encoding_info.encoding.as_str() {
            "UTF-8" => {
                match str::from_utf8(content) {
                    Ok(text) => Ok(text.to_string()),
                    Err(_) => {
                        // Try to recover invalid UTF-8
                        Ok(String::from_utf8_lossy(content).to_string())
                    }
                }
            }
            "ASCII" => {
                // ASCII is a subset of UTF-8
                Ok(String::from_utf8_lossy(content).to_string())
            }
            "UTF-16" => {
                // Try to decode UTF-16 (both LE and BE)
                if content.len() >= 2 {
                    let bom_le = &content[0..2] == &[0xFF, 0xFE];
                    let bom_be = &content[0..2] == &[0xFE, 0xFF];
                    
                    if bom_le || bom_be {
                        let start_idx = if bom_le || bom_be { 2 } else { 0 };
                        let utf16_bytes = &content[start_idx..];
                        
                        if utf16_bytes.len() % 2 == 0 {
                            let mut utf16_chars = Vec::new();
                            for chunk in utf16_bytes.chunks_exact(2) {
                                let code_unit = if bom_be {
                                    u16::from_be_bytes([chunk[0], chunk[1]])
                                } else {
                                    u16::from_le_bytes([chunk[0], chunk[1]])
                                };
                                utf16_chars.push(code_unit);
                            }
                            
                            match String::from_utf16(&utf16_chars) {
                                Ok(text) => Ok(text),
                                Err(_) => Ok(String::from_utf8_lossy(content).to_string()),
                            }
                        } else {
                            Ok(String::from_utf8_lossy(content).to_string())
                        }
                    } else {
                        Ok(String::from_utf8_lossy(content).to_string())
                    }
                } else {
                    Ok(String::from_utf8_lossy(content).to_string())
                }
            }
            "Latin1" | "ISO-8859-1" => {
                // Convert Latin1 to UTF-8
                let text: String = content.iter().map(|&b| b as char).collect();
                Ok(text)
            }
            _ => {
                // Default fallback
                Ok(String::from_utf8_lossy(content).to_string())
            }
        }
    }
    
    /// Analyze text structure (paragraphs, lines, etc.)
    fn analyze_text_structure(&self, text: &str) -> TextStructure {
        let lines: Vec<&str> = text.lines().collect();
        let paragraphs: Vec<&str> = text.split("\n\n").filter(|p| !p.trim().is_empty()).collect();
        
        // Count sentences (simple heuristic)
        let sentence_count = text.matches(|c| c == '.' || c == '!' || c == '?').count();
        
        // Find headers (lines that are short and followed by empty lines)
        let mut headers = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            let line_trimmed = line.trim();
            if !line_trimmed.is_empty() 
                && line_trimmed.len() < 100 
                && !line_trimmed.ends_with('.')
                && i + 1 < lines.len() 
                && lines[i + 1].trim().is_empty() {
                headers.push(line_trimmed.to_string());
            }
        }
        
        // Detect lists
        let mut lists = Vec::new();
        let mut current_list = Vec::new();
        let mut in_list = false;
        
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("- ") || trimmed.starts_with("* ") || 
               trimmed.starts_with("+ ") || 
               (trimmed.len() > 2 && trimmed.chars().nth(1) == Some('.') && trimmed.chars().nth(0).unwrap().is_ascii_digit()) {
                current_list.push(trimmed.to_string());
                in_list = true;
            } else if in_list && trimmed.is_empty() {
                // Empty line might continue list
                continue;
            } else if in_list {
                // End of list
                if !current_list.is_empty() {
                    lists.push(current_list.clone());
                    current_list.clear();
                }
                in_list = false;
            }
        }
        
        // Add final list if exists
        if !current_list.is_empty() {
            lists.push(current_list);
        }
        
        TextStructure {
            line_count: lines.len(),
            paragraph_count: paragraphs.len(),
            sentence_count,
            headers,
            lists,
            has_tables: self.detect_tables(text),
            has_code_blocks: self.detect_code_blocks(text),
        }
    }
    
    /// Detect table-like structures
    fn detect_tables(&self, text: &str) -> bool {
        let lines: Vec<&str> = text.lines().collect();
        let mut potential_table_lines = 0;
        
        for line in &lines {
            let tab_count = line.matches('\t').count();
            let pipe_count = line.matches('|').count();
            
            if tab_count >= 2 || pipe_count >= 2 {
                potential_table_lines += 1;
            }
        }
        
        potential_table_lines >= 3 // At least 3 lines that look like table rows
    }
    
    /// Detect code blocks
    fn detect_code_blocks(&self, text: &str) -> bool {
        // Simple heuristics for code detection
        text.contains("```") || 
        text.contains("    ") || // 4-space indentation
        text.contains("function ") ||
        text.contains("class ") ||
        text.contains("def ") ||
        text.contains("import ") ||
        text.contains("#include") ||
        text.contains("<?php")
    }
    
    /// Perform linguistic analysis
    fn analyze_linguistics(&self, text: &str) -> Result<LinguisticAnalysis> {
        let words: Vec<&str> = text.split_whitespace().collect();
        
        // Word frequency analysis
        let mut word_frequency: HashMap<String, u32> = HashMap::new();
        for word in &words {
            let clean_word = word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            
            if !clean_word.is_empty() && clean_word.len() > 2 {
                *word_frequency.entry(clean_word).or_insert(0) += 1;
            }
        }
        
        // Find most common words
        let mut word_counts: Vec<(String, u32)> = word_frequency.into_iter().collect();
        word_counts.sort_by(|a, b| b.1.cmp(&a.1));
        let most_common_words = word_counts.into_iter().take(20).collect();
        
        // Vocabulary analysis
        let unique_words = most_common_words.len();
        let total_words = words.len();
        let vocabulary_richness = if total_words > 0 {
            unique_words as f64 / total_words as f64
        } else {
            0.0
        };
        
        // Average word length
        let average_word_length = if !words.is_empty() {
            words.iter().map(|w| w.len()).sum::<usize>() as f64 / words.len() as f64
        } else {
            0.0
        };
        
        // Sentence complexity (average words per sentence)
        let sentences = text.split(|c| c == '.' || c == '!' || c == '?').count();
        let average_sentence_length = if sentences > 0 {
            words.len() as f64 / sentences as f64
        } else {
            0.0
        };
        
        Ok(LinguisticAnalysis {
            most_common_words,
            vocabulary_richness,
            average_word_length,
            average_sentence_length,
            total_words,
            unique_words,
        })
    }
    
    /// Extract format-specific metadata
    fn extract_format_metadata(&self, text: &str, format: &DocumentFormat) -> Result<HashMap<String, String>> {
        let mut metadata = HashMap::new();
        
        match format {
            DocumentFormat::Markdown => {
                self.extract_markdown_metadata(text, &mut metadata);
            }
            DocumentFormat::Html => {
                self.extract_html_metadata(text, &mut metadata);
            }
            DocumentFormat::Csv => {
                self.extract_csv_metadata(text, &mut metadata);
            }
            DocumentFormat::Json => {
                self.extract_json_metadata(text, &mut metadata);
            }
            DocumentFormat::Xml => {
                self.extract_xml_metadata(text, &mut metadata);
            }
            DocumentFormat::Yaml => {
                self.extract_yaml_metadata(text, &mut metadata);
            }
            _ => {
                // Generic text metadata
                metadata.insert("format".to_string(), format!("{:?}", format));
            }
        }
        
        Ok(metadata)
    }
    
    /// Extract Markdown-specific metadata
    fn extract_markdown_metadata(&self, text: &str, metadata: &mut HashMap<String, String>) {
        // Count Markdown elements
        let header_count = text.matches('#').count();
        let link_count = text.matches("](").count();
        let image_count = text.matches("![").count();
        let code_block_count = text.matches("```").count() / 2;
        
        metadata.insert("headers".to_string(), header_count.to_string());
        metadata.insert("links".to_string(), link_count.to_string());
        metadata.insert("images".to_string(), image_count.to_string());
        metadata.insert("code_blocks".to_string(), code_block_count.to_string());
        
        // Extract title (first # header)
        if let Some(title_line) = text.lines().find(|line| line.starts_with("# ")) {
            let title = title_line.trim_start_matches('#').trim();
            metadata.insert("title".to_string(), title.to_string());
        }
    }
    
    /// Extract HTML-specific metadata
    fn extract_html_metadata(&self, text: &str, metadata: &mut HashMap<String, String>) {
        // Simple HTML tag counting
        let tag_counts = [
            ("div", text.matches("<div").count()),
            ("p", text.matches("<p").count()),
            ("h1", text.matches("<h1").count()),
            ("h2", text.matches("<h2").count()),
            ("h3", text.matches("<h3").count()),
            ("img", text.matches("<img").count()),
            ("a", text.matches("<a ").count()),
            ("table", text.matches("<table").count()),
        ];
        
        for (tag, count) in tag_counts {
            if count > 0 {
                metadata.insert(format!("{}_tags", tag), count.to_string());
            }
        }
        
        // Extract title
        if let Some(start) = text.find("<title>") {
            if let Some(end) = text[start..].find("</title>") {
                let title = &text[start + 7..start + end];
                metadata.insert("title".to_string(), title.to_string());
            }
        }
    }
    
    /// Extract CSV-specific metadata
    fn extract_csv_metadata(&self, text: &str, metadata: &mut HashMap<String, String>) {
        let lines: Vec<&str> = text.lines().collect();
        if !lines.is_empty() {
            let first_line = lines[0];
            let column_count = first_line.split(',').count();
            
            metadata.insert("rows".to_string(), lines.len().to_string());
            metadata.insert("columns".to_string(), column_count.to_string());
            
            // Extract headers (first row)
            let headers: Vec<&str> = first_line.split(',').map(|h| h.trim()).collect();
            metadata.insert("headers".to_string(), headers.join(", "));
        }
    }
    
    /// Extract JSON-specific metadata
    fn extract_json_metadata(&self, text: &str, metadata: &mut HashMap<String, String>) {
        // Simple JSON structure analysis
        let object_count = text.matches('{').count();
        let array_count = text.matches('[').count();
        let string_count = text.matches('"').count() / 2;
        
        metadata.insert("objects".to_string(), object_count.to_string());
        metadata.insert("arrays".to_string(), array_count.to_string());  
        metadata.insert("strings".to_string(), string_count.to_string());
    }
    
    /// Extract XML-specific metadata
    fn extract_xml_metadata(&self, text: &str, metadata: &mut HashMap<String, String>) {
        // Count XML elements
        let element_count = text.matches('<').count() - text.matches("<!--").count();
        let comment_count = text.matches("<!--").count();
        
        metadata.insert("elements".to_string(), element_count.to_string());
        metadata.insert("comments".to_string(), comment_count.to_string());
        
        // Extract root element
        if let Some(start) = text.find('<') {
            if let Some(end) = text[start..].find('>') {
                let tag = &text[start + 1..start + end];
                if !tag.starts_with('?') && !tag.starts_with('!') {
                    let root_tag = tag.split_whitespace().next().unwrap_or(tag);
                    metadata.insert("root_element".to_string(), root_tag.to_string());
                }
            }
        }
    }
    
    /// Extract YAML-specific metadata  
    fn extract_yaml_metadata(&self, text: &str, metadata: &mut HashMap<String, String>) {
        let lines: Vec<&str> = text.lines().collect();
        let mut key_count = 0;
        let mut list_count = 0;
        
        for line in &lines {
            let trimmed = line.trim();
            if trimmed.contains(':') && !trimmed.starts_with('#') {
                key_count += 1;
            }
            if trimmed.starts_with('-') {
                list_count += 1;
            }
        }
        
        metadata.insert("keys".to_string(), key_count.to_string());
        metadata.insert("list_items".to_string(), list_count.to_string());
    }
    
    /// Analyze text readability
    fn analyze_readability(&self, text: &str) -> Result<ReadabilityAnalysis> {
        let sentences = text.split(|c| c == '.' || c == '!' || c == '?').count();
        let words: Vec<&str> = text.split_whitespace().collect();
        let syllable_count = self.estimate_syllables(text);
        
        // Flesch Reading Ease Score
        let flesch_score = if sentences > 0 && !words.is_empty() {
            206.835 - (1.015 * (words.len() as f64 / sentences as f64)) - (84.6 * (syllable_count / words.len() as f64))
        } else {
            0.0
        };
        
        // Flesch-Kincaid Grade Level
        let grade_level = if sentences > 0 && !words.is_empty() {
            (0.39 * (words.len() as f64 / sentences as f64)) + (11.8 * (syllable_count / words.len() as f64)) - 15.59
        } else {
            0.0
        };
        
        let reading_level = match flesch_score as i32 {
            90..=100 => "Very Easy",
            80..=89 => "Easy", 
            70..=79 => "Fairly Easy",
            60..=69 => "Standard",
            50..=59 => "Fairly Difficult",
            30..=49 => "Difficult",
            _ => "Very Difficult",
        };
        
        Ok(ReadabilityAnalysis {
            flesch_score,
            grade_level,
            reading_level: reading_level.to_string(),
            estimated_reading_time_minutes: words.len() as f64 / 250.0, // Average reading speed
        })
    }
    
    /// Estimate syllable count (simple heuristic)
    fn estimate_syllables(&self, text: &str) -> f64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut total_syllables = 0.0;
        
        for word in words {
            let clean_word = word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>();
            if !clean_word.is_empty() {
                // Simple syllable counting heuristic
                let vowel_count = clean_word.chars().filter(|c| "aeiou".contains(*c)).count();
                let syllables = std::cmp::max(1, vowel_count) as f64;
                total_syllables += syllables;
            }
        }
        
        total_syllables
    }
    
    /// Detect patterns in text
    fn detect_patterns(&self, text: &str) -> Result<Vec<DetectedPattern>> {
        let mut patterns = Vec::new();
        
        // Email patterns
        let email_regex = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        for mat in email_regex.find_iter(text) {
            patterns.push(DetectedPattern {
                pattern_type: "email".to_string(),
                value: mat.as_str().to_string(),
                position: mat.start(),
            });
        }
        
        // URL patterns
        let url_regex = regex::Regex::new(r"https?://[^\s]+").unwrap();
        for mat in url_regex.find_iter(text) {
            patterns.push(DetectedPattern {
                pattern_type: "url".to_string(),
                value: mat.as_str().to_string(),
                position: mat.start(),
            });
        }
        
        // Phone number patterns (simple)
        let phone_regex = regex::Regex::new(r"\b\d{3}-\d{3}-\d{4}\b|\b\(\d{3}\)\s?\d{3}-\d{4}\b").unwrap();
        for mat in phone_regex.find_iter(text) {
            patterns.push(DetectedPattern {
                pattern_type: "phone".to_string(),
                value: mat.as_str().to_string(),
                position: mat.start(),
            });
        }
        
        // Date patterns
        let date_regex = regex::Regex::new(r"\b\d{1,2}[/-]\d{1,2}[/-]\d{2,4}\b|\b\d{4}-\d{2}-\d{2}\b").unwrap();
        for mat in date_regex.find_iter(text) {
            patterns.push(DetectedPattern {
                pattern_type: "date".to_string(),
                value: mat.as_str().to_string(),
                position: mat.start(),
            });
        }
        
        Ok(patterns)
    }
    
    /// Calculate comprehensive text statistics
    fn calculate_text_statistics(&self, text: &str) -> TextStatistics {
        let chars = text.chars().collect::<Vec<char>>();
        let words: Vec<&str> = text.split_whitespace().collect();
        let lines: Vec<&str> = text.lines().collect();
        let sentences = text.split(|c| c == '.' || c == '!' || c == '?').count();
        
        // Character analysis
        let alphabetic_chars = chars.iter().filter(|c| c.is_alphabetic()).count();
        let numeric_chars = chars.iter().filter(|c| c.is_numeric()).count();
        let whitespace_chars = chars.iter().filter(|c| c.is_whitespace()).count();
        let punctuation_chars = chars.iter().filter(|c| c.is_ascii_punctuation()).count();
        
        TextStatistics {
            total_characters: chars.len(),
            alphabetic_characters: alphabetic_chars,
            numeric_characters: numeric_chars,
            whitespace_characters: whitespace_chars,
            punctuation_characters: punctuation_chars,
            total_words: words.len(),
            total_lines: lines.len(),
            total_sentences: sentences,
            blank_lines: lines.iter().filter(|line| line.trim().is_empty()).count(),
            average_words_per_line: if lines.len() > 0 { words.len() as f64 / lines.len() as f64 } else { 0.0 },
            average_characters_per_word: if words.len() > 0 { chars.len() as f64 / words.len() as f64 } else { 0.0 },
        }
    }
}

impl Default for TextProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Language detection functionality
struct LanguageDetector;

impl LanguageDetector {
    fn new() -> Self {
        Self
    }
    
    fn detect_language(&self, text: &str) -> Result<LanguageInfo> {
        // Simple language detection based on character frequency and common words
        // In a production system, you'd use a proper language detection library
        
        let common_english_words = ["the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by"];
        let common_spanish_words = ["el", "la", "y", "o", "pero", "en", "por", "para", "de", "con"];
        let common_french_words = ["le", "la", "et", "ou", "mais", "dans", "sur", "pour", "de", "avec"];
        
        let words: Vec<&str> = text.to_lowercase().split_whitespace().collect();
        
        let english_matches = words.iter().filter(|word| common_english_words.contains(word)).count();
        let spanish_matches = words.iter().filter(|word| common_spanish_words.contains(word)).count();
        let french_matches = words.iter().filter(|word| common_french_words.contains(word)).count();
        
        let (detected_language, confidence) = if english_matches > spanish_matches && english_matches > french_matches {
            ("English", english_matches as f64 / words.len() as f64)
        } else if spanish_matches > french_matches {
            ("Spanish", spanish_matches as f64 / words.len() as f64)
        } else if french_matches > 0 {
            ("French", french_matches as f64 / words.len() as f64)
        } else {
            ("Unknown", 0.0)
        };
        
        Ok(LanguageInfo {
            detected_language: detected_language.to_string(),
            confidence,
            script: "Latin".to_string(), // Simplified
            is_rtl: false, // Simplified
        })
    }
}

/// Encoding detection functionality
struct EncodingDetector;

impl EncodingDetector {
    fn new() -> Self {
        Self
    }
    
    fn detect_encoding(&self, content: &[u8]) -> Result<EncodingInfo> {
        // Check for BOM (Byte Order Mark)
        if content.len() >= 3 && &content[0..3] == &[0xEF, 0xBB, 0xBF] {
            return Ok(EncodingInfo {
                encoding: "UTF-8".to_string(),
                confidence: 1.0,
                has_bom: true,
            });
        }
        
        if content.len() >= 2 {
            if &content[0..2] == &[0xFF, 0xFE] {
                return Ok(EncodingInfo {
                    encoding: "UTF-16LE".to_string(),
                    confidence: 1.0,
                    has_bom: true,
                });
            }
            
            if &content[0..2] == &[0xFE, 0xFF] {
                return Ok(EncodingInfo {
                    encoding: "UTF-16BE".to_string(),
                    confidence: 1.0,
                    has_bom: true,
                });
            }
        }
        
        // Try to validate as UTF-8
        match str::from_utf8(content) {
            Ok(_) => Ok(EncodingInfo {
                encoding: "UTF-8".to_string(),
                confidence: 0.9,
                has_bom: false,
            }),
            Err(_) => {
                // Check if it's ASCII
                if content.iter().all(|&b| b < 128) {
                    Ok(EncodingInfo {
                        encoding: "ASCII".to_string(),
                        confidence: 0.95,
                        has_bom: false,
                    })
                } else {
                    // Default to UTF-8 with lossy conversion
                    Ok(EncodingInfo {
                        encoding: "UTF-8".to_string(),
                        confidence: 0.5,
                        has_bom: false,
                    })
                }
            }
        }
    }
}

/// Performance monitoring for text processing
struct PerformanceMonitor {
    start_memory: usize,
}

impl PerformanceMonitor {
    fn new() -> Self {
        Self {
            start_memory: 0,
        }
    }
    
    fn get_memory_usage(&self) -> f64 {
        // Placeholder for memory usage calculation
        0.0
    }
}

/// Processed text document result
#[derive(Debug, Clone)]
pub struct ProcessedTextDocument {
    pub format: DocumentFormat,
    pub encoding_info: EncodingInfo,
    pub text_content: String,
    pub structure: TextStructure,
    pub language_info: LanguageInfo,
    pub linguistic_analysis: LinguisticAnalysis,
    pub metadata: HashMap<String, String>,
    pub readability: ReadabilityAnalysis,
    pub patterns: Vec<DetectedPattern>,
    pub text_stats: TextStatistics,
    pub stats: ProcessingStats,
}

/// Text encoding information
#[derive(Debug, Clone)]
pub struct EncodingInfo {
    pub encoding: String,
    pub confidence: f64,
    pub has_bom: bool,
}

/// Text structure analysis
#[derive(Debug, Clone)]
pub struct TextStructure {
    pub line_count: usize,
    pub paragraph_count: usize,
    pub sentence_count: usize,
    pub headers: Vec<String>,
    pub lists: Vec<Vec<String>>,
    pub has_tables: bool,
    pub has_code_blocks: bool,
}

/// Language detection information
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    pub detected_language: String,
    pub confidence: f64,
    pub script: String,
    pub is_rtl: bool,
}

/// Linguistic analysis results
#[derive(Debug, Clone)]
pub struct LinguisticAnalysis {
    pub most_common_words: Vec<(String, u32)>,
    pub vocabulary_richness: f64,
    pub average_word_length: f64,
    pub average_sentence_length: f64,
    pub total_words: usize,
    pub unique_words: usize,
}

/// Readability analysis
#[derive(Debug, Clone)]
pub struct ReadabilityAnalysis {
    pub flesch_score: f64,
    pub grade_level: f64,
    pub reading_level: String,
    pub estimated_reading_time_minutes: f64,
}

/// Detected pattern in text
#[derive(Debug, Clone)]
pub struct DetectedPattern {
    pub pattern_type: String,
    pub value: String,
    pub position: usize,
}

/// Comprehensive text statistics
#[derive(Debug, Clone)]
pub struct TextStatistics {
    pub total_characters: usize,
    pub alphabetic_characters: usize,
    pub numeric_characters: usize,
    pub whitespace_characters: usize,
    pub punctuation_characters: usize,
    pub total_words: usize,
    pub total_lines: usize,
    pub total_sentences: usize,
    pub blank_lines: usize,
    pub average_words_per_line: f64,
    pub average_characters_per_word: f64,
}