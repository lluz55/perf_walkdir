import 'dart:convert';

import 'file.dart';

class Data {
  List<File> files;
  bool success;

  Data(
    this.files,
    this.success,
  );

  Data copyWith({
    List<File>? files,
    bool? success,
  }) {
    return Data(
      files ?? this.files,
      success ?? this.success,
    );
  }

  Map<String, dynamic> toMap() {
    return {
      'files': files.map((x) => x.toMap()).toList(),
      'success': success,
    };
  }

  factory Data.fromMap(Map<String, dynamic> map) {
    return Data(
      map['success'] == 'false'
          ? List<File>.empty()
          : List<File>.from(map['data']?.map((x) => File.fromMap(x))),
      map['success'],
    );
  }

  String toJson() => json.encode(toMap());

  factory Data.fromJson(String source) => Data.fromMap(json.decode(source));

  @override
  String toString() => 'Data(files: $files, success: $success)';

  @override
  int get hashCode => files.hashCode ^ success.hashCode;
}
