import 'dart:convert';

class DocumentMetadata {
  final String? title;
  final String? author;
  final String? subject;
  final List<String> keywords;
  final String? creator;
  final String? producer;
  final String? creationDate;
  final String? modificationDate;
  final int pageCount;
  final int fileSize;
  final String mimeType;
  final Map<String, String> customProperties;

  const DocumentMetadata({
    this.title,
    this.author,
    this.subject,
    this.keywords = const [],
    this.creator,
    this.producer,
    this.creationDate,
    this.modificationDate,
    required this.pageCount,
    required this.fileSize,
    required this.mimeType,
    this.customProperties = const {},
  });

  factory DocumentMetadata.fromJson(Map<String, dynamic> json) {
    return DocumentMetadata(
      title: json['title'] as String?,
      author: json['author'] as String?,
      subject: json['subject'] as String?,
      keywords: (json['keywords'] as List<dynamic>?)?.cast<String>() ?? [],
      creator: json['creator'] as String?,
      producer: json['producer'] as String?,
      creationDate: json['creation_date'] as String?,
      modificationDate: json['modification_date'] as String?,
      pageCount: json['page_count'] as int? ?? 0,
      fileSize: json['file_size'] as int? ?? 0,
      mimeType: json['mime_type'] as String? ?? '',
      customProperties: (json['custom_properties'] as Map<String, dynamic>?)
              ?.cast<String, String>() ??
          {},
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'title': title,
      'author': author,
      'subject': subject,
      'keywords': keywords,
      'creator': creator,
      'producer': producer,
      'creation_date': creationDate,
      'modification_date': modificationDate,
      'page_count': pageCount,
      'file_size': fileSize,
      'mime_type': mimeType,
      'custom_properties': customProperties,
    };
  }

  @override
  String toString() {
    return 'DocumentMetadata{title: $title, author: $author, pageCount: $pageCount, fileSize: $fileSize}';
  }
}
