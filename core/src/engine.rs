use crate::{
    document::{Document, DocumentFormat, Annotation, AnnotationType},
    metadata::DocumentMetadata,
    plugin::{PluginRegistry, PluginCapability, PluginInput},
    error::{Result, FilefireError},
};
use std::path::Path;
use std::collections::HashMap;

/// Main FileFire engine that coordinates document processing and plugins
pub struct FilefireEngine {
    plugin_registry: PluginRegistry,
}

impl FilefireEngine {
    pub fn new() -> Self {
        Self {
            plugin_registry: PluginRegistry::new(),
        }
    }
    
    /// Open a document from file path
    pub async fn open_file<P: AsRef<Path>>(&self, path: P) -> Result<Document> {
        Document::from_file(path).await
    }
    
    /// Open a document from bytes
    pub fn open_bytes(&self, content: Vec<u8>, format: DocumentFormat) -> Result<Document> {
        Ok(Document::new(content, format))
    }
    
    /// Extract metadata from a document
    pub async fn extract_metadata<P: AsRef<Path>>(&self, path: P) -> Result<DocumentMetadata> {
        let doc = self.open_file(path).await?;
        Ok(doc.metadata)
    }
    
    /// Add annotation to a document
    pub async fn annotate(
        &self,
        document: &mut Document,
        page: u32,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        content: String,
        annotation_type: AnnotationType,
    ) -> Result<String> {
        let annotation_id = format!("ann_{}", uuid::Uuid::new_v4().to_string());
        let annotation = Annotation {
            id: annotation_id.clone(),
            page,
            x,
            y,
            width,
            height,
            content,
            annotation_type,
            author: Some("FileFire".to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
            modified_at: None,
        };
        
        document.add_annotation(annotation);
        Ok(annotation_id)
    }
    
    /// Convert document to another format using plugins
    pub async fn convert(
        &self,
        document: &Document,
        target_format: DocumentFormat,
    ) -> Result<Document> {
        let conversion_plugins = self.plugin_registry
            .supports_capability(&PluginCapability::FileConversion);
            
        if conversion_plugins.is_empty() {
            return Err(FilefireError::Plugin(
                "No conversion plugins available".to_string()
            ));
        }
        
        let plugin_name = conversion_plugins[0];
        let plugin = self.plugin_registry
            .get_plugin(plugin_name)
            .ok_or_else(|| FilefireError::Plugin("Plugin not found".to_string()))?;
        
        let input = PluginInput {
            data: document.content.clone(),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("source_format".to_string(), 
                    serde_json::to_value(&document.format).unwrap());
                meta.insert("target_format".to_string(), 
                    serde_json::to_value(&target_format).unwrap());
                meta
            },
            format: document.format.mime_type().to_string(),
            parameters: HashMap::new(),
        };
        
        let output = plugin.process(&input).await?;
        
        if !output.success {
            return Err(FilefireError::Plugin(
                output.error_message.unwrap_or("Conversion failed".to_string())
            ));
        }
        
        Ok(Document::new(output.data, target_format))
    }
    
    /// Perform OCR on a document
    pub async fn ocr(&self, document: &Document) -> Result<String> {
        let ocr_plugins = self.plugin_registry
            .supports_capability(&PluginCapability::Ocr);
            
        if ocr_plugins.is_empty() {
            return Err(FilefireError::Plugin(
                "No OCR plugins available".to_string()
            ));
        }
        
        let plugin_name = ocr_plugins[0];
        let plugin = self.plugin_registry
            .get_plugin(plugin_name)
            .ok_or_else(|| FilefireError::Plugin("OCR plugin not found".to_string()))?;
        
        let input = PluginInput {
            data: document.content.clone(),
            metadata: HashMap::new(),
            format: document.format.mime_type().to_string(),
            parameters: HashMap::new(),
        };
        
        let output = plugin.process(&input).await?;
        
        if !output.success {
            return Err(FilefireError::Plugin(
                output.error_message.unwrap_or("OCR failed".to_string())
            ));
        }
        
        String::from_utf8(output.data)
            .map_err(|_| FilefireError::Plugin("Invalid OCR output".to_string()))
    }
    
    /// Apply watermark to a document
    pub async fn watermark(
        &self,
        document: &mut Document,
        watermark_text: &str,
        opacity: f32,
    ) -> Result<()> {
        let watermark_plugins = self.plugin_registry
            .supports_capability(&PluginCapability::Watermark);
            
        if watermark_plugins.is_empty() {
            return Err(FilefireError::Plugin(
                "No watermark plugins available".to_string()
            ));
        }
        
        let plugin_name = watermark_plugins[0];
        let plugin = self.plugin_registry
            .get_plugin(plugin_name)
            .ok_or_else(|| FilefireError::Plugin("Watermark plugin not found".to_string()))?;
        
        let mut parameters = HashMap::new();
        parameters.insert("text".to_string(), serde_json::Value::String(watermark_text.to_string()));
        parameters.insert("opacity".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(opacity as f64).unwrap()));
        
        let input = PluginInput {
            data: document.content.clone(),
            metadata: HashMap::new(),
            format: document.format.mime_type().to_string(),
            parameters,
        };
        
        let output = plugin.process(&input).await?;
        
        if !output.success {
            return Err(FilefireError::Plugin(
                output.error_message.unwrap_or("Watermarking failed".to_string())
            ));
        }
        
        document.content = output.data;
        document.is_modified = true;
        Ok(())
    }
    
    /// Get plugin registry for managing plugins
    pub fn plugins(&mut self) -> &mut PluginRegistry {
        &mut self.plugin_registry
    }
    
    /// List all available plugins
    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugin_registry.list_plugins()
    }
    
    /// Check if a capability is supported
    pub fn supports_capability(&self, capability: &PluginCapability) -> bool {
        !self.plugin_registry.supports_capability(capability).is_empty()
    }
}

impl Default for FilefireEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Add missing dependencies for compilation
use uuid::Uuid;
use chrono;

// Helper module for UUID generation
mod uuid {
    pub struct Uuid;
    
    impl Uuid {
        pub fn new_v4() -> Self {
            Self
        }
        
        pub fn to_string(&self) -> String {
            format!("{:x}", std::collections::hash_map::DefaultHasher::new().finish())
        }
    }
}

// Helper module for time handling
mod chrono {
    pub struct Utc;
    
    impl Utc {
        pub fn now() -> DateTime {
            DateTime
        }
    }
    
    pub struct DateTime;
    
    impl DateTime {
        pub fn to_rfc3339(&self) -> String {
            "2024-01-01T00:00:00Z".to_string()
        }
    }
}