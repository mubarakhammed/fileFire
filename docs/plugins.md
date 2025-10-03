# Plugin Development Guide

This guide explains how to develop plugins for FileFire's microkernel architecture.

## Overview

FileFire plugins extend the core functionality through a well-defined interface. Plugins can provide:

- OCR (Optical Character Recognition)
- Digital signatures
- Watermarking
- AI/ML processing
- Format conversion
- Custom document processing

## Plugin Architecture

### Plugin Trait

All plugins must implement the `Plugin` trait:

```rust
use async_trait::async_trait;
use filefire_core::{Plugin, PluginCapability, PluginConfig, PluginInput, PluginOutput, Result};

#[async_trait]
pub trait Plugin: Send + Sync {
    /// Plugin name (must be unique)
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Plugin description
    fn description(&self) -> &str;
    
    /// Plugin author
    fn author(&self) -> &str;
    
    /// Plugin capabilities
    fn capabilities(&self) -> Vec<PluginCapability>;
    
    /// Initialize the plugin
    async fn initialize(&mut self, config: PluginConfig) -> Result<()>;
    
    /// Process data through the plugin
    async fn process(&self, input: &PluginInput) -> Result<PluginOutput>;
    
    /// Cleanup plugin resources
    async fn cleanup(&mut self) -> Result<()>;
}
```

### Plugin Capabilities

Define what your plugin can do:

```rust
#[derive(Debug, Clone)]
pub enum PluginCapability {
    Ocr,
    DigitalSignature,
    Watermark,
    FileConversion,
    AiSummarization,
    AiTagging,
    SemanticSearch,
    Compression,
    Encryption,
    Custom(String),
}
```

## Creating a Plugin

### 1. Setup Project Structure

```
my_plugin/
├── Cargo.toml
├── src/
│   └── lib.rs
└── README.md
```

### 2. Configure Cargo.toml

```toml
[package]
name = "my-filefire-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
filefire-core = { path = "../core" }
async-trait = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 3. Implement the Plugin

```rust
use filefire_core::{
    Plugin, PluginCapability, PluginConfig, PluginInput, PluginOutput, Result
};
use async_trait::async_trait;
use std::collections::HashMap;

pub struct MyPlugin {
    config: Option<PluginConfig>,
    // Plugin-specific state
}

impl MyPlugin {
    pub fn new() -> Self {
        Self { 
            config: None 
        }
    }
}

#[async_trait]
impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn description(&self) -> &str {
        "My custom FileFire plugin"
    }
    
    fn author(&self) -> &str {
        "Your Name"
    }
    
    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::Custom("my-capability".to_string())]
    }
    
    async fn initialize(&mut self, config: PluginConfig) -> Result<()> {
        // Initialize your plugin
        // Load models, connect to services, etc.
        self.config = Some(config);
        Ok(())
    }
    
    async fn process(&self, input: &PluginInput) -> Result<PluginOutput> {
        // Process the input data
        // Return results
        
        let processed_data = self.do_processing(&input.data).await?;
        
        Ok(PluginOutput {
            data: processed_data,
            metadata: HashMap::new(),
            format: "application/json".to_string(),
            success: true,
            error_message: None,
        })
    }
    
    async fn cleanup(&mut self) -> Result<()> {
        // Clean up resources
        self.config = None;
        Ok(())
    }
}

impl MyPlugin {
    async fn do_processing(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Your plugin logic here
        Ok(data.to_vec())
    }
}

// Export functions for dynamic loading
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    let plugin = Box::new(MyPlugin::new());
    Box::into_raw(plugin) as *mut dyn Plugin
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn Plugin) {
    if !plugin.is_null() {
        unsafe {
            let _ = Box::from_raw(plugin);
        }
    }
}
```

## Plugin Input/Output

### PluginInput Structure

```rust
pub struct PluginInput {
    /// Input data (document bytes, image data, etc.)
    pub data: Vec<u8>,
    
    /// Metadata about the input
    pub metadata: HashMap<String, serde_json::Value>,
    
    /// Input format (MIME type)
    pub format: String,
    
    /// Processing parameters
    pub parameters: HashMap<String, serde_json::Value>,
}
```

### PluginOutput Structure

```rust
pub struct PluginOutput {
    /// Processed data
    pub data: Vec<u8>,
    
    /// Output metadata
    pub metadata: HashMap<String, serde_json::Value>,
    
    /// Output format (MIME type)
    pub format: String,
    
    /// Processing success status
    pub success: bool,
    
    /// Error message if processing failed
    pub error_message: Option<String>,
}
```

## Plugin Examples

### OCR Plugin

```rust
use tesseract::Tesseract;

pub struct OCRPlugin {
    tesseract: Option<Tesseract>,
}

#[async_trait]
impl Plugin for OCRPlugin {
    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::Ocr]
    }
    
    async fn initialize(&mut self, config: PluginConfig) -> Result<()> {
        // Initialize Tesseract
        let language = config.settings
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("eng");
            
        self.tesseract = Some(
            Tesseract::new(None, Some(language))?
        );
        Ok(())
    }
    
    async fn process(&self, input: &PluginInput) -> Result<PluginOutput> {
        let tesseract = self.tesseract.as_ref()
            .ok_or_else(|| "OCR not initialized")?;
            
        // Convert input to image format if needed
        let image = if input.format.starts_with("image/") {
            input.data.clone()
        } else {
            // Convert PDF page to image
            convert_pdf_to_image(&input.data)?
        };
        
        // Perform OCR
        let text = tesseract
            .set_image_from_mem(&image)?
            .get_text()?;
            
        let mut metadata = HashMap::new();
        metadata.insert("confidence".to_string(), 
            serde_json::Value::Number(
                serde_json::Number::from_f64(tesseract.mean_text_conf() as f64).unwrap()
            )
        );
        
        Ok(PluginOutput {
            data: text.into_bytes(),
            metadata,
            format: "text/plain".to_string(),
            success: true,
            error_message: None,
        })
    }
}
```

### Watermark Plugin

```rust
pub struct WatermarkPlugin;

#[async_trait]
impl Plugin for WatermarkPlugin {
    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::Watermark]
    }
    
    async fn process(&self, input: &PluginInput) -> Result<PluginOutput> {
        let text = input.parameters
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("CONFIDENTIAL");
            
        let opacity = input.parameters
            .get("opacity")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.3) as f32;
            
        let watermarked_data = match input.format.as_str() {
            "application/pdf" => apply_pdf_watermark(&input.data, text, opacity)?,
            "image/jpeg" | "image/png" => apply_image_watermark(&input.data, text, opacity)?,
            _ => return Err("Unsupported format for watermarking".into()),
        };
        
        Ok(PluginOutput {
            data: watermarked_data,
            metadata: HashMap::new(),
            format: input.format.clone(),
            success: true,
            error_message: None,
        })
    }
}
```

## Configuration

### Plugin Configuration

```rust
pub struct PluginConfig {
    /// Plugin-specific settings
    pub settings: HashMap<String, serde_json::Value>,
    
    /// Temporary directory for processing
    pub temp_dir: PathBuf,
    
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    
    /// Timeout for processing in seconds
    pub timeout_seconds: u64,
}
```

### Example Configuration

```json
{
  "ocr": {
    "language": "eng",
    "dpi": 300,
    "preserve_layout": true
  },
  "watermark": {
    "default_opacity": 0.3,
    "default_position": "center",
    "font_size": 24
  }
}
```

## Error Handling

### Plugin Errors

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    
    #[error("Resource not available: {0}")]
    ResourceUnavailable(String),
}
```

### Error Handling Best Practices

1. **Provide detailed error messages**
2. **Include context and suggestions**
3. **Handle recoverable errors gracefully**
4. **Log errors for debugging**

```rust
async fn process(&self, input: &PluginInput) -> Result<PluginOutput> {
    match self.do_processing(input).await {
        Ok(result) => Ok(result),
        Err(e) => {
            log::error!("Plugin {} failed to process: {}", self.name(), e);
            Ok(PluginOutput {
                data: vec![],
                metadata: HashMap::new(),
                format: "text/plain".to_string(),
                success: false,
                error_message: Some(format!("Processing failed: {}", e)),
            })
        }
    }
}
```

## Testing Plugins

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_plugin_initialization() {
        let mut plugin = MyPlugin::new();
        let config = PluginConfig::default();
        
        let result = plugin.initialize(config).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_plugin_processing() {
        let mut plugin = MyPlugin::new();
        plugin.initialize(PluginConfig::default()).await.unwrap();
        
        let input = PluginInput {
            data: b"test data".to_vec(),
            metadata: HashMap::new(),
            format: "text/plain".to_string(),
            parameters: HashMap::new(),
        };
        
        let result = plugin.process(&input).await;
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.success);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_plugin_with_core_engine() {
    let mut engine = FilefireEngine::new();
    let plugin = Box::new(MyPlugin::new());
    
    engine.plugins().register_plugin(plugin);
    
    // Test plugin integration
    // ...
}
```

## Performance Considerations

### Memory Management

1. **Minimize memory allocations**
2. **Use streaming for large files**
3. **Clean up resources promptly**
4. **Respect memory limits**

```rust
async fn process_large_file(&self, input: &PluginInput) -> Result<PluginOutput> {
    // Use streaming instead of loading entire file
    let mut reader = std::io::Cursor::new(&input.data);
    let mut output = Vec::new();
    
    // Process in chunks
    let mut buffer = vec![0; 8192];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 { break; }
        
        let processed_chunk = self.process_chunk(&buffer[..bytes_read])?;
        output.extend_from_slice(&processed_chunk);
    }
    
    Ok(PluginOutput {
        data: output,
        // ...
    })
}
```

### Async Processing

```rust
async fn process(&self, input: &PluginInput) -> Result<PluginOutput> {
    // Use tokio for concurrent processing
    let tasks: Vec<_> = input.data
        .chunks(1024)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            tokio::spawn(async move {
                process_chunk(chunk).await
            })
        })
        .collect();
    
    let results = futures::future::try_join_all(tasks).await?;
    let combined_result = combine_results(results);
    
    Ok(combined_result)
}
```

## Plugin Distribution

### Building for Distribution

```bash
# Build release version
cargo build --release

# Create plugin package
mkdir my-plugin-1.0.0
cp target/release/libmy_filefire_plugin.so my-plugin-1.0.0/
cp README.md my-plugin-1.0.0/
cp plugin.toml my-plugin-1.0.0/

# Create archive
tar -czf my-plugin-1.0.0.tar.gz my-plugin-1.0.0/
```

### Plugin Manifest

Create a `plugin.toml` file:

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "My custom FileFire plugin"
author = "Your Name"
license = "MIT"
homepage = "https://github.com/yourname/my-plugin"

[dependencies]
filefire-core = "^0.1.0"

[capabilities]
custom = ["my-capability"]

[configuration]
required = []
optional = ["setting1", "setting2"]
```

## Best Practices

1. **Keep plugins focused** - One plugin, one responsibility
2. **Handle errors gracefully** - Don't crash the core engine
3. **Respect resource limits** - Memory, CPU, and time constraints
4. **Provide good documentation** - Clear examples and API docs
5. **Test thoroughly** - Unit tests, integration tests, and edge cases
6. **Version carefully** - Semantic versioning and compatibility
7. **Log appropriately** - Help with debugging but don't spam
8. **Clean up resources** - Implement proper cleanup in the cleanup method

This guide should help you create powerful, reliable plugins for the FileFire document processing ecosystem.