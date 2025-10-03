enum AnnotationType {
  text,
  highlight,
  underline,
  strikethrough,
  note,
  drawing,
  stamp,
  link;

  static AnnotationType fromString(String type) {
    switch (type.toLowerCase()) {
      case 'text':
        return AnnotationType.text;
      case 'highlight':
        return AnnotationType.highlight;
      case 'underline':
        return AnnotationType.underline;
      case 'strikethrough':
        return AnnotationType.strikethrough;
      case 'note':
        return AnnotationType.note;
      case 'drawing':
        return AnnotationType.drawing;
      case 'stamp':
        return AnnotationType.stamp;
      case 'link':
        return AnnotationType.link;
      default:
        return AnnotationType.text;
    }
  }

  @override
  String toString() {
    return name;
  }
}

class Annotation {
  final String id;
  final int page;
  final double x;
  final double y;
  final double width;
  final double height;
  final String content;
  final AnnotationType type;
  final String? author;
  final DateTime createdAt;
  final DateTime? modifiedAt;

  const Annotation({
    required this.id,
    required this.page,
    required this.x,
    required this.y,
    required this.width,
    required this.height,
    required this.content,
    required this.type,
    this.author,
    required this.createdAt,
    this.modifiedAt,
  });

  factory Annotation.fromJson(Map<String, dynamic> json) {
    return Annotation(
      id: json['id'] as String,
      page: json['page'] as int,
      x: (json['x'] as num).toDouble(),
      y: (json['y'] as num).toDouble(),
      width: (json['width'] as num).toDouble(),
      height: (json['height'] as num).toDouble(),
      content: json['content'] as String,
      type: AnnotationType.fromString(json['annotation_type'] as String),
      author: json['author'] as String?,
      createdAt: DateTime.parse(json['created_at'] as String),
      modifiedAt: json['modified_at'] != null
          ? DateTime.parse(json['modified_at'] as String)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'page': page,
      'x': x,
      'y': y,
      'width': width,
      'height': height,
      'content': content,
      'annotation_type': type.toString(),
      'author': author,
      'created_at': createdAt.toIso8601String(),
      'modified_at': modifiedAt?.toIso8601String(),
    };
  }

  @override
  String toString() {
    return 'Annotation{id: $id, page: $page, type: $type, content: $content}';
  }
}
