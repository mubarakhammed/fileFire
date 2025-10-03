import 'dart:io';
import 'dart:typed_data';
import 'metadata.dart';
import 'annotation.dart';
import 'exceptions.dart';

enum DocumentFormat {
  pdf,
  docx,
  xlsx,
  pptx,
  jpeg,
  png,
  tiff,
  unknown;

  static DocumentFormat fromExtension(String extension) {
    switch (extension.toLowerCase()) {
      case 'pdf':
        return DocumentFormat.pdf;
      case 'docx':
        return DocumentFormat.docx;
      case 'xlsx':
        return DocumentFormat.xlsx;
      case 'pptx':
        return DocumentFormat.pptx;
      case 'jpg':
      case 'jpeg':
        return DocumentFormat.jpeg;
      case 'png':
        return DocumentFormat.png;
      case 'tiff':
      case 'tif':
        return DocumentFormat.tiff;
      default:
        return DocumentFormat.unknown;
    }
  }

  String get mimeType {
    switch (this) {
      case DocumentFormat.pdf:
        return 'application/pdf';
      case DocumentFormat.docx:
        return 'application/vnd.openxmlformats-officedocument.wordprocessingml.document';
      case DocumentFormat.xlsx:
        return 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet';
      case DocumentFormat.pptx:
        return 'application/vnd.openxmlformats-officedocument.presentationml.presentation';
      case DocumentFormat.jpeg:
        return 'image/jpeg';
      case DocumentFormat.png:
        return 'image/png';
      case DocumentFormat.tiff:
        return 'image/tiff';
      case DocumentFormat.unknown:
        return 'application/octet-stream';
    }
  }
}

class Document {
  final int _handle;
  final DocumentFormat format;
  DocumentMetadata? _metadata;
  final List<Annotation> _annotations = [];
  bool _isModified = false;

  Document._(this._handle, this.format);

  /// Create a document from file path
  static Future<Document> fromFile(String filePath) async {
    if (!File(filePath).existsSync()) {
      throw DocumentNotFoundException('File not found: $filePath');
    }

    final extension = filePath.split('.').last;
    final format = DocumentFormat.fromExtension(extension);

    if (format == DocumentFormat.unknown) {
      throw UnsupportedFormatException('Unsupported file format: $extension');
    }

    // TODO: Call native code to open file
    // For now, simulate with a dummy handle
    final handle = DateTime.now().millisecondsSinceEpoch % 1000000;
    final document = Document._(handle, format);

    // Load metadata
    await document._loadMetadata();

    return document;
  }

  /// Create a document from bytes
  static Future<Document> fromBytes(
      Uint8List bytes, DocumentFormat format) async {
    if (format == DocumentFormat.unknown) {
      throw UnsupportedFormatException('Unknown document format');
    }

    // TODO: Call native code to open bytes
    // For now, simulate with a dummy handle
    final handle = DateTime.now().millisecondsSinceEpoch % 1000000;
    final document = Document._(handle, format);

    // Load metadata
    await document._loadMetadata();

    return document;
  }

  /// Get document metadata
  DocumentMetadata? get metadata => _metadata;

  /// Get all annotations
  List<Annotation> get annotations => List.unmodifiable(_annotations);

  /// Check if document has been modified
  bool get isModified => _isModified;

  /// Add an annotation to the document
  Future<String> annotate({
    required int page,
    required double x,
    required double y,
    required double width,
    required double height,
    required String content,
    AnnotationType type = AnnotationType.text,
  }) async {
    if (page < 1 || (metadata != null && page > metadata!.pageCount)) {
      throw AnnotationException('Invalid page number: $page');
    }

    // TODO: Call native code to add annotation
    // For now, simulate annotation creation
    final annotationId = 'ann_${DateTime.now().millisecondsSinceEpoch}';
    final annotation = Annotation(
      id: annotationId,
      page: page,
      x: x,
      y: y,
      width: width,
      height: height,
      content: content,
      type: type,
      author: 'Flutter User',
      createdAt: DateTime.now(),
    );

    _annotations.add(annotation);
    _isModified = true;

    return annotationId;
  }

  /// Remove an annotation by ID
  bool removeAnnotation(String annotationId) {
    final initialLength = _annotations.length;
    _annotations.removeWhere((annotation) => annotation.id == annotationId);
    final removed = _annotations.length != initialLength;
    if (removed) {
      _isModified = true;
    }
    return removed;
  }

  /// Get annotations for a specific page
  List<Annotation> getAnnotationsForPage(int page) {
    return _annotations.where((annotation) => annotation.page == page).toList();
  }

  /// Save document to file path
  Future<void> save(String filePath) async {
    // TODO: Call native code to save document
    // For now, simulate save operation
    await Future.delayed(const Duration(milliseconds: 100));
    _isModified = false;
  }

  /// Convert document to another format
  Future<Document> convertTo(DocumentFormat targetFormat) async {
    if (targetFormat == format) {
      return this;
    }

    // TODO: Call native plugin system for conversion
    // For now, simulate conversion
    await Future.delayed(const Duration(milliseconds: 500));

    throw UnsupportedFormatException(
        'Conversion from $format to $targetFormat not yet implemented');
  }

  /// Perform OCR on the document
  Future<String> performOCR() async {
    // TODO: Call OCR plugin through native code
    // For now, simulate OCR
    await Future.delayed(const Duration(milliseconds: 2000));

    return 'Sample OCR text extracted from document.\nThis would be the actual text content.';
  }

  /// Apply watermark to the document
  Future<void> addWatermark({
    required String text,
    double opacity = 0.3,
    String position = 'center',
  }) async {
    // TODO: Call watermark plugin through native code
    // For now, simulate watermarking
    await Future.delayed(const Duration(milliseconds: 800));
    _isModified = true;
  }

  Future<void> _loadMetadata() async {
    // TODO: Call native code to extract metadata
    // For now, simulate metadata loading
    _metadata = DocumentMetadata(
      title: 'Sample Document',
      author: 'FileFire User',
      pageCount: 10,
      fileSize: 1024 * 1024, // 1MB
      mimeType: format.mimeType,
      creationDate: DateTime.now().toIso8601String(),
    );
  }

  @override
  String toString() {
    return 'Document{handle: $_handle, format: $format, modified: $_isModified}';
  }
}
