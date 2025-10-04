import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';

/// Native library handle
DynamicLibrary? _library;

/// Document handle for native operations
typedef DocumentHandle = int;

/// Native function signatures
typedef _CreateEngineNative = Pointer<Void> Function();
typedef _CreateEngineDart = Pointer<Void> Function();

typedef _ProcessDocumentNative = IntPtr Function(Pointer<Void> engine,
    Pointer<Uint8> data, IntPtr length, Pointer<Utf8> format);
typedef _ProcessDocumentDart = int Function(Pointer<Void> engine,
    Pointer<Uint8> data, int length, Pointer<Utf8> format);

typedef _GetDocumentTextNative = Pointer<Utf8> Function(IntPtr handle);
typedef _GetDocumentTextDart = Pointer<Utf8> Function(int handle);

typedef _GetDocumentMetadataNative = Pointer<Utf8> Function(IntPtr handle);
typedef _GetDocumentMetadataDart = Pointer<Utf8> Function(int handle);

typedef _AddAnnotationNative = Bool Function(IntPtr handle, Double x, Double y,
    Double width, Double height, Pointer<Utf8> content);
typedef _AddAnnotationDart = bool Function(int handle, double x, double y,
    double width, double height, Pointer<Utf8> content);

typedef _SaveDocumentNative = Bool Function(IntPtr handle, Pointer<Utf8> path);
typedef _SaveDocumentDart = bool Function(int handle, Pointer<Utf8> path);

typedef _CleanupDocumentNative = Void Function(IntPtr handle);
typedef _CleanupDocumentDart = void Function(int handle);

typedef _DestroyEngineNative = Void Function(Pointer<Void> engine);
typedef _DestroyEngineDart = void Function(Pointer<Void> engine);

/// FFI Bridge for FileFire Flutter bindings
class FFIBridge {
  static FFIBridge? _instance;
  static FFIBridge get instance => _instance ??= FFIBridge._();

  late Pointer<Void> _engine;
  late _CreateEngineDart _createEngine;
  late _ProcessDocumentDart _processDocument;
  late _GetDocumentTextDart _getDocumentText;
  late _GetDocumentMetadataDart _getDocumentMetadata;
  late _AddAnnotationDart _addAnnotation;
  late _SaveDocumentDart _saveDocument;
  late _CleanupDocumentDart _cleanupDocument;
  late _DestroyEngineDart _destroyEngine;

  FFIBridge._();

  /// Initialize the FFI bridge
  Future<void> initialize() async {
    if (_library != null) return;

    // Load the native library
    try {
      if (Platform.isAndroid) {
        _library = DynamicLibrary.open('libfilefire_core.so');
      } else if (Platform.isIOS) {
        _library = DynamicLibrary.process();
      } else if (Platform.isLinux) {
        _library = DynamicLibrary.open('libfilefire_core.so');
      } else if (Platform.isMacOS) {
        _library = DynamicLibrary.open('libfilefire_core.dylib');
      } else if (Platform.isWindows) {
        _library = DynamicLibrary.open('filefire_core.dll');
      } else {
        throw UnsupportedError('Unsupported platform');
      }
    } catch (e) {
      throw Exception('Failed to load native library: $e');
    }

    // Bind functions
    _createEngine = _library!
        .lookupFunction<_CreateEngineNative, _CreateEngineDart>(
            'filefire_create_engine');
    _processDocument = _library!
        .lookupFunction<_ProcessDocumentNative, _ProcessDocumentDart>(
            'filefire_process_document');
    _getDocumentText = _library!
        .lookupFunction<_GetDocumentTextNative, _GetDocumentTextDart>(
            'filefire_get_document_text');
    _getDocumentMetadata = _library!
        .lookupFunction<_GetDocumentMetadataNative, _GetDocumentMetadataDart>(
            'filefire_get_document_metadata');
    _addAnnotation = _library!
        .lookupFunction<_AddAnnotationNative, _AddAnnotationDart>(
            'filefire_add_annotation');
    _saveDocument = _library!
        .lookupFunction<_SaveDocumentNative, _SaveDocumentDart>(
            'filefire_save_document');
    _cleanupDocument = _library!
        .lookupFunction<_CleanupDocumentNative, _CleanupDocumentDart>(
            'filefire_cleanup_document');
    _destroyEngine = _library!
        .lookupFunction<_DestroyEngineNative, _DestroyEngineDart>(
            'filefire_destroy_engine');

    // Create engine instance
    _engine = _createEngine();
    if (_engine == nullptr) {
      throw Exception('Failed to create FileFire engine');
    }
  }

  /// Process document from bytes
  Future<DocumentHandle> processDocument(Uint8List bytes, String format) async {
    await initialize();

    final dataPtr = malloc<Uint8>(bytes.length);
    final data = dataPtr.asTypedList(bytes.length);
    data.setAll(0, bytes);

    final formatPtr = format.toNativeUtf8();

    try {
      final handle =
          _processDocument(_engine, dataPtr, bytes.length, formatPtr);
      if (handle == 0) {
        throw Exception('Failed to process document');
      }
      return handle;
    } finally {
      malloc.free(dataPtr);
      malloc.free(formatPtr);
    }
  }

  /// Get document text content
  Future<String> getDocumentText(DocumentHandle handle) async {
    final textPtr = _getDocumentText(handle);
    if (textPtr == nullptr) {
      return '';
    }

    try {
      return textPtr.toDartString();
    } finally {
      // Note: In a real implementation, we might need to free this
      // depending on how the native side allocates the string
    }
  }

  /// Get document metadata as JSON string
  Future<String> getDocumentMetadata(DocumentHandle handle) async {
    final metadataPtr = _getDocumentMetadata(handle);
    if (metadataPtr == nullptr) {
      return '{}';
    }

    try {
      return metadataPtr.toDartString();
    } finally {
      // Note: In a real implementation, we might need to free this
    }
  }

  /// Add annotation to document
  Future<bool> addAnnotation(DocumentHandle handle, double x, double y,
      double width, double height, String content) async {
    final contentPtr = content.toNativeUtf8();

    try {
      return _addAnnotation(handle, x, y, width, height, contentPtr);
    } finally {
      malloc.free(contentPtr);
    }
  }

  /// Save document to file
  Future<bool> saveDocument(DocumentHandle handle, String path) async {
    final pathPtr = path.toNativeUtf8();

    try {
      return _saveDocument(handle, pathPtr);
    } finally {
      malloc.free(pathPtr);
    }
  }

  /// Cleanup document resources
  void cleanupDocument(DocumentHandle handle) {
    _cleanupDocument(handle);
  }

  /// Cleanup the FFI bridge
  void dispose() {
    if (_library != null && _engine != nullptr) {
      _destroyEngine(_engine);
      _engine = nullptr;
    }
    _library = null;
    _instance = null;
  }
}
