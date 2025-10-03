use wasm_bindgen::prelude::*;
use crate::{
    engine::FilefireEngine,
    document::{Document, DocumentFormat, AnnotationType},
    error::Result,
};
use serde::{Deserialize, Serialize};

// WASM-compatible structures
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct WasmDocument {
    handle: usize,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct WasmMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub page_count: u32,
    pub file_size: u64,
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct WasmAnnotation {
    pub id: String,
    pub page: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub content: String,
}

// Global state for WASM
static mut WASM_ENGINE: Option<FilefireEngine> = None;
static mut WASM_DOCUMENTS: Vec<Document> = Vec::new();

fn get_wasm_engine() -> &'static mut FilefireEngine {
    unsafe {
        if WASM_ENGINE.is_none() {
            WASM_ENGINE = Some(FilefireEngine::new());
        }
        WASM_ENGINE.as_mut().unwrap()
    }
}

#[wasm_bindgen]
pub struct FileFire;

#[wasm_bindgen]
impl FileFire {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FileFire {
        console_error_panic_hook::set_once();
        FileFire
    }
    
    #[wasm_bindgen(js_name = openBytes)]
    pub async fn open_bytes(&self, data: &[u8], format: &str) -> Result<WasmDocument, JsValue> {
        let format = match format {
            "pdf" => DocumentFormat::Pdf,
            "docx" => DocumentFormat::Docx,
            "xlsx" => DocumentFormat::Xlsx,
            "pptx" => DocumentFormat::Pptx,
            "jpeg" | "jpg" => DocumentFormat::Jpeg,
            "png" => DocumentFormat::Png,
            "tiff" | "tif" => DocumentFormat::Tiff,
            _ => DocumentFormat::Unknown(format.to_string()),
        };
        
        let engine = get_wasm_engine();
        let doc = engine.open_bytes(data.to_vec(), format)
            .map_err(|e| JsValue::from_str(&format!("Failed to open document: {}", e)))?;
        
        unsafe {
            WASM_DOCUMENTS.push(doc);
            Ok(WasmDocument {
                handle: WASM_DOCUMENTS.len(),
            })
        }
    }
    
    #[wasm_bindgen(js_name = getMetadata)]
    pub fn get_metadata(&self, doc: &WasmDocument) -> Result<JsValue, JsValue> {
        if doc.handle == 0 || doc.handle > unsafe { WASM_DOCUMENTS.len() } {
            return Err(JsValue::from_str("Invalid document handle"));
        }
        
        let document = unsafe { &WASM_DOCUMENTS[doc.handle - 1] };
        let metadata = WasmMetadata {
            title: document.metadata.title.clone(),
            author: document.metadata.author.clone(),
            page_count: document.metadata.page_count,
            file_size: document.metadata.file_size,
        };
        
        serde_wasm_bindgen::to_value(&metadata)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize metadata: {}", e)))
    }
    
    #[wasm_bindgen(js_name = annotate)]
    pub async fn annotate(
        &self,
        doc: &WasmDocument,
        page: u32,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        content: &str,
        annotation_type: &str,
    ) -> Result<String, JsValue> {
        if doc.handle == 0 || doc.handle > unsafe { WASM_DOCUMENTS.len() } {
            return Err(JsValue::from_str("Invalid document handle"));
        }
        
        let annotation_type = match annotation_type {
            "text" => AnnotationType::Text,
            "highlight" => AnnotationType::Highlight,
            "underline" => AnnotationType::Underline,
            "strikethrough" => AnnotationType::Strikethrough,
            "note" => AnnotationType::Note,
            "drawing" => AnnotationType::Drawing,
            "stamp" => AnnotationType::Stamp,
            "link" => AnnotationType::Link,
            _ => AnnotationType::Text,
        };
        
        let document = unsafe { &mut WASM_DOCUMENTS[doc.handle - 1] };
        let engine = get_wasm_engine();
        
        engine.annotate(
            document,
            page,
            x,
            y,
            width,
            height,
            content.to_string(),
            annotation_type,
        ).await
        .map_err(|e| JsValue::from_str(&format!("Failed to add annotation: {}", e)))
    }
    
    #[wasm_bindgen(js_name = save)]
    pub async fn save(&self, doc: &WasmDocument) -> Result<Vec<u8>, JsValue> {
        if doc.handle == 0 || doc.handle > unsafe { WASM_DOCUMENTS.len() } {
            return Err(JsValue::from_str("Invalid document handle"));
        }
        
        let document = unsafe { &WASM_DOCUMENTS[doc.handle - 1] };
        
        // For WASM, we return the document bytes instead of saving to file
        document.render_with_annotations().await
            .map_err(|e| JsValue::from_str(&format!("Failed to render document: {}", e)))
    }
    
    #[wasm_bindgen(js_name = ocr)]
    pub async fn ocr(&self, doc: &WasmDocument) -> Result<String, JsValue> {
        if doc.handle == 0 || doc.handle > unsafe { WASM_DOCUMENTS.len() } {
            return Err(JsValue::from_str("Invalid document handle"));
        }
        
        let document = unsafe { &WASM_DOCUMENTS[doc.handle - 1] };
        let engine = get_wasm_engine();
        
        engine.ocr(document).await
            .map_err(|e| JsValue::from_str(&format!("OCR failed: {}", e)))
    }
    
    #[wasm_bindgen(js_name = listPlugins)]
    pub fn list_plugins(&self) -> Vec<JsValue> {
        let engine = get_wasm_engine();
        engine.list_plugins()
            .into_iter()
            .map(|name| JsValue::from_str(name))
            .collect()
    }
}

// Add a method to Document for WASM compatibility
impl Document {
    pub async fn render_with_annotations(&self) -> Result<Vec<u8>> {
        // This is a placeholder implementation
        // In a real implementation, this would render the document with annotations
        Ok(self.content.clone())
    }
}