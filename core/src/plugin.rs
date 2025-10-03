use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::error::Result;

/// Plugin trait that all plugins must implement
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Plugin name
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub settings: HashMap<String, serde_json::Value>,
    pub temp_dir: PathBuf,
    pub max_memory_mb: u64,
    pub timeout_seconds: u64,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            settings: HashMap::new(),
            temp_dir: std::env::temp_dir(),
            max_memory_mb: 512,
            timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInput {
    pub data: Vec<u8>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub format: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginOutput {
    pub data: Vec<u8>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub format: String,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Plugin registry for managing loaded plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    plugin_paths: HashMap<String, PathBuf>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_paths: HashMap::new(),
        }
    }
    
    /// Load a plugin from a dynamic library
    pub async fn load_plugin(&mut self, path: PathBuf) -> Result<String> {
        // Plugin loading implementation would go here
        // For now, return a placeholder
        let plugin_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
            
        self.plugin_paths.insert(plugin_name.clone(), path);
        Ok(plugin_name)
    }
    
    /// Register a plugin instance
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
    }
    
    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }
    
    /// Get all loaded plugins
    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
    
    /// Check if a plugin supports a capability
    pub fn supports_capability(&self, capability: &PluginCapability) -> Vec<&str> {
        self.plugins
            .iter()
            .filter(|(_, plugin)| plugin.capabilities().contains(capability))
            .map(|(name, _)| name.as_str())
            .collect()
    }
    
    /// Unload a plugin
    pub async fn unload_plugin(&mut self, name: &str) -> Result<()> {
        if let Some(mut plugin) = self.plugins.remove(name) {
            plugin.cleanup().await?;
        }
        self.plugin_paths.remove(name);
        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Dummy plugin for testing
#[derive(Debug)]
pub struct DummyPlugin {
    name: String,
    version: String,
}

impl DummyPlugin {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "1.0.0".to_string(),
        }
    }
}

#[async_trait]
impl Plugin for DummyPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn description(&self) -> &str {
        "A dummy plugin for testing"
    }
    
    fn author(&self) -> &str {
        "FileFire Team"
    }
    
    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![PluginCapability::Custom("dummy".to_string())]
    }
    
    async fn initialize(&mut self, _config: PluginConfig) -> Result<()> {
        log::info!("Initializing dummy plugin: {}", self.name);
        Ok(())
    }
    
    async fn process(&self, input: &PluginInput) -> Result<PluginOutput> {
        log::info!("Processing data with dummy plugin: {}", self.name);
        Ok(PluginOutput {
            data: input.data.clone(),
            metadata: input.metadata.clone(),
            format: input.format.clone(),
            success: true,
            error_message: None,
        })
    }
    
    async fn cleanup(&mut self) -> Result<()> {
        log::info!("Cleaning up dummy plugin: {}", self.name);
        Ok(())
    }
}