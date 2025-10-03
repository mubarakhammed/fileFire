import 'dart:ffi' as ffi;
import 'dart:io';
import 'dart:typed_data';
import 'document.dart';
import 'metadata.dart';
import 'exceptions.dart';

class FileFire {
  static FileFire? _instance;
  static bool _initialized = false;

  FileFire._();

  /// Get the singleton instance of FileFire
  static FileFire get instance {
    _instance ??= FileFire._();
    return _instance!;
  }

  /// Initialize the FileFire engine
  static Future<void> initialize() async {
    if (_initialized) return;

    try {
      // TODO: Load native library and initialize
      // For now, simulate initialization
      await Future.delayed(const Duration(milliseconds: 100));
      _initialized = true;
    } catch (e) {
      throw FilefireException('Failed to initialize FileFire: $e');
    }
  }

  /// Convenience method to open a document from file path
  static Future<Document> open(String filePath) async {
    await initialize();
    return Document.fromFile(filePath);
  }

  /// Convenience method to open a document from bytes
  static Future<Document> openBytes(
      Uint8List bytes, DocumentFormat format) async {
    await initialize();
    return Document.fromBytes(bytes, format);
  }

  /// Extract metadata from a file without loading the full document
  static Future<DocumentMetadata> extractMetadata(String filePath) async {
    await initialize();

    if (!File(filePath).existsSync()) {
      throw DocumentNotFoundException('File not found: $filePath');
    }

    // TODO: Call native code to extract metadata only
    // For now, simulate metadata extraction
    await Future.delayed(const Duration(milliseconds: 50));

    final extension = filePath.split('.').last;
    final format = DocumentFormat.fromExtension(extension);

    return DocumentMetadata(
      title: 'Sample Document',
      author: 'Unknown Author',
      pageCount: 1,
      fileSize: File(filePath).lengthSync(),
      mimeType: format.mimeType,
      creationDate: File(filePath).statSync().modified.toIso8601String(),
    );
  }

  /// Get list of supported document formats
  static List<DocumentFormat> getSupportedFormats() {
    return [
      DocumentFormat.pdf,
      DocumentFormat.docx,
      DocumentFormat.xlsx,
      DocumentFormat.pptx,
      DocumentFormat.jpeg,
      DocumentFormat.png,
      DocumentFormat.tiff,
    ];
  }

  /// Check if a format is supported
  static bool isFormatSupported(DocumentFormat format) {
    return getSupportedFormats().contains(format);
  }

  /// Get available plugins
  Future<List<String>> getAvailablePlugins() async {
    await initialize();

    // TODO: Call native code to get plugin list
    // For now, return mock plugins
    return ['ocr', 'watermark', 'signature', 'ai'];
  }

  /// Check if a plugin is loaded
  Future<bool> isPluginLoaded(String pluginName) async {
    await initialize();

    // TODO: Call native code to check plugin status
    // For now, simulate plugin check
    final availablePlugins = await getAvailablePlugins();
    return availablePlugins.contains(pluginName);
  }

  /// Get FileFire version
  static String getVersion() {
    return '0.1.0';
  }

  /// Cleanup resources
  static Future<void> cleanup() async {
    if (!_initialized) return;

    try {
      // TODO: Call native cleanup
      await Future.delayed(const Duration(milliseconds: 50));
      _initialized = false;
      _instance = null;
    } catch (e) {
      throw FilefireException('Failed to cleanup FileFire: $e');
    }
  }
}
