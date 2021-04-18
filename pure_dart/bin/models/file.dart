import 'dart:convert';

enum FileType {
  File,
  Dir,
}

class File {
  String name;
  FileType ty;
  int size;

  File(
    this.name,
    this.size,
    this.ty,
  );

  File copyWith({
    String? name,
    int? size,
    FileType? ty,
  }) {
    return File(name ?? this.name, size ?? this.size, ty ?? this.ty);
  }

  Map<String, dynamic> toMap() {
    return {
      'name': name,
      'size': size,
    };
  }

  factory File.fromMap(Map<String, dynamic> map) {
    return File(map['path'], map['size'],
        map['ty'] == 'file' ? FileType.File : FileType.Dir);
  }

  String toJson() => json.encode(toMap());

  factory File.fromJson(String source) => File.fromMap(json.decode(source));

  @override
  String toString() =>
      '{"name": "$name", "size": $size, "type": "${ty.toString()}")';

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;

    return other is File && other.name == name && other.size == size;
  }

  @override
  int get hashCode => name.hashCode ^ size.hashCode;
}
