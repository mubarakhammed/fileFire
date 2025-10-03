use crate::{
    engine::FilefireEngine,
    document::{Document, DocumentFormat, AnnotationType},
    error::{Result, FilefireError},
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// FFI structures
#[repr(C)]
pub struct CDocument {
    pub handle: usize,
}

#[repr(C)]
pub struct CMetadata {
    pub title: *const c_char,
    pub author: *const c_char,
    pub page_count: u32,
    pub file_size: u64,
}

#[repr(C)]
pub struct CAnnotation {
    pub id: *const c_char,
    pub page: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub content: *const c_char,
    pub annotation_type: c_int,
}

// Global engine instance
static mut ENGINE: Option<FilefireEngine> = None;
static mut DOCUMENTS: Vec<Document> = Vec::new();

fn get_engine() -> &'static mut FilefireEngine {
    unsafe {
        if ENGINE.is_none() {
            ENGINE = Some(FilefireEngine::new());
        }
        ENGINE.as_mut().unwrap()
    }
}

// Initialize the FileFire engine
#[no_mangle]
pub extern "C" fn filefire_init() -> c_int {
    unsafe {
        ENGINE = Some(FilefireEngine::new());
    }
    0 // Success
}

// Open a document from file path
#[no_mangle]
pub extern "C" fn filefire_open_file(path: *const c_char) -> CDocument {
    if path.is_null() {
        return CDocument { handle: 0 };
    }
    
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CDocument { handle: 0 },
    };
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let engine = get_engine();
    
    match rt.block_on(engine.open_file(path_str)) {
        Ok(doc) => {
            unsafe {
                DOCUMENTS.push(doc);
                CDocument {
                    handle: DOCUMENTS.len(),
                }
            }
        }
        Err(_) => CDocument { handle: 0 },
    }
}

// Get document metadata
#[no_mangle]
pub extern "C" fn filefire_get_metadata(doc: CDocument) -> CMetadata {
    if doc.handle == 0 || doc.handle > unsafe { DOCUMENTS.len() } {
        return CMetadata {
            title: ptr::null(),
            author: ptr::null(),
            page_count: 0,
            file_size: 0,
        };
    }
    
    let document = unsafe { &DOCUMENTS[doc.handle - 1] };
    let metadata = &document.metadata;
    
    let title = metadata.title.as_ref()
        .map(|s| CString::new(s.as_str()).unwrap().into_raw() as *const c_char)
        .unwrap_or(ptr::null());
        
    let author = metadata.author.as_ref()
        .map(|s| CString::new(s.as_str()).unwrap().into_raw() as *const c_char)
        .unwrap_or(ptr::null());
    
    CMetadata {
        title,
        author,
        page_count: metadata.page_count,
        file_size: metadata.file_size,
    }
}

// Add annotation to document
#[no_mangle]
pub extern "C" fn filefire_annotate(
    doc: CDocument,
    page: u32,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    content: *const c_char,
    annotation_type: c_int,
) -> *const c_char {
    if doc.handle == 0 || doc.handle > unsafe { DOCUMENTS.len() } || content.is_null() {
        return ptr::null();
    }
    
    let c_str = unsafe { CStr::from_ptr(content) };
    let content_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return ptr::null(),
    };
    
    let annotation_type = match annotation_type {
        0 => AnnotationType::Text,
        1 => AnnotationType::Highlight,
        2 => AnnotationType::Underline,
        3 => AnnotationType::Strikethrough,
        4 => AnnotationType::Note,
        5 => AnnotationType::Drawing,
        6 => AnnotationType::Stamp,
        7 => AnnotationType::Link,
        _ => AnnotationType::Text,
    };
    
    let document = unsafe { &mut DOCUMENTS[doc.handle - 1] };
    let engine = get_engine();
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(engine.annotate(
        document,
        page,
        x,
        y,
        width,
        height,
        content_str,
        annotation_type,
    )) {
        Ok(id) => CString::new(id).unwrap().into_raw(),
        Err(_) => ptr::null(),
    }
}

// Save document to file
#[no_mangle]
pub extern "C" fn filefire_save(doc: CDocument, path: *const c_char) -> c_int {
    if doc.handle == 0 || doc.handle > unsafe { DOCUMENTS.len() } || path.is_null() {
        return -1;
    }
    
    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    
    let document = unsafe { &DOCUMENTS[doc.handle - 1] };
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    match rt.block_on(document.save(path_str)) {
        Ok(_) => 0,  // Success
        Err(_) => -1, // Error
    }
}

// Cleanup document
#[no_mangle]
pub extern "C" fn filefire_close_document(doc: CDocument) -> c_int {
    if doc.handle == 0 || doc.handle > unsafe { DOCUMENTS.len() } {
        return -1;
    }
    
    // Note: In a real implementation, we'd properly manage document lifecycle
    // For now, we'll just mark it as handled
    0
}

// Free string memory allocated by FFI functions
#[no_mangle]
pub extern "C" fn filefire_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

// Cleanup the engine
#[no_mangle]
pub extern "C" fn filefire_cleanup() -> c_int {
    unsafe {
        ENGINE = None;
        DOCUMENTS.clear();
    }
    0
}