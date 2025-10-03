class FilefireException implements Exception {
  final String message;
  final String? code;

  const FilefireException(this.message, [this.code]);

  @override
  String toString() =>
      'FilefireException: $message${code != null ? ' (Code: $code)' : ''}';
}

class DocumentNotFoundException extends FilefireException {
  const DocumentNotFoundException(String message)
      : super(message, 'DOCUMENT_NOT_FOUND');
}

class UnsupportedFormatException extends FilefireException {
  const UnsupportedFormatException(String message)
      : super(message, 'UNSUPPORTED_FORMAT');
}

class AnnotationException extends FilefireException {
  const AnnotationException(String message)
      : super(message, 'ANNOTATION_ERROR');
}

class PluginException extends FilefireException {
  const PluginException(String message) : super(message, 'PLUGIN_ERROR');
}
