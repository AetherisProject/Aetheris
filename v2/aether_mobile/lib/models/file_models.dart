import 'package:flutter/foundation.dart';

/// Represents a file or directory in the Aether filesystem
class FileInfo {
  final String name;
  final String path;
  final int size;
  final DateTime modified;
  final bool isDirectory;
  final String? mimeType;
  final String? extension;

  const FileInfo({
    required this.name,
    required this.path,
    required this.size,
    required this.modified,
    required this.isDirectory,
    this.mimeType,
    this.extension,
  });

  /// Create FileInfo from JSON
  factory FileInfo.fromJson(Map<String, dynamic> json) {
    return FileInfo(
      name: json['name'] as String? ?? '',
      path: json['path'] as String? ?? '',
      size: json['size'] as int? ?? 0,
      modified: json['modified'] != null 
          ? DateTime.parse(json['modified'] as String)
          : DateTime.now(),
      isDirectory: json['isDirectory'] as bool? ?? false,
      mimeType: json['mimeType'] as String?,
      extension: json['extension'] as String?,
    );
  }

  /// Convert to JSON
  Map<String, dynamic> toJson() {
    return {
      'name': name,
      'path': path,
      'size': size,
      'modified': modified.toIso8601String(),
      'isDirectory': isDirectory,
      'mimeType': mimeType,
      'extension': extension,
    };
  }

  /// Create a copy with updated fields
  FileInfo copyWith({
    String? name,
    String? path,
    int? size,
    DateTime? modified,
    bool? isDirectory,
    String? mimeType,
    String? extension,
  }) {
    return FileInfo(
      name: name ?? this.name,
      path: path ?? this.path,
      size: size ?? this.size,
      modified: modified ?? this.modified,
      isDirectory: isDirectory ?? this.isDirectory,
      mimeType: mimeType ?? this.mimeType,
      extension: extension ?? this.extension,
    );
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is FileInfo && 
        other.name == name && 
        other.path == path &&
        other.isDirectory == isDirectory;
  }

  @override
  int get hashCode => name.hashCode ^ path.hashCode ^ isDirectory.hashCode;

  @override
  String toString() {
    return 'FileInfo(name: $name, path: $path, isDirectory: $isDirectory)';
  }
}

/// Represents a file content response
class FileContent {
  final String path;
  final String content;
  final String? mimeType;
  final DateTime modified;

  const FileContent({
    required this.path,
    required this.content,
    this.mimeType,
    required this.modified,
  });

  factory FileContent.fromJson(Map<String, dynamic> json) {
    return FileContent(
      path: json['path'] as String? ?? '',
      content: json['content'] as String? ?? '',
      mimeType: json['mimeType'] as String?,
      modified: json['modified'] != null 
          ? DateTime.parse(json['modified'] as String)
          : DateTime.now(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'path': path,
      'content': content,
      'mimeType': mimeType,
      'modified': modified.toIso8601String(),
    };
  }
}

/// Request to create a new file
class CreateFileRequest {
  final String path;
  final String content;
  final bool overwrite;

  const CreateFileRequest({
    required this.path,
    required this.content,
    this.overwrite = false,
  });

  Map<String, dynamic> toJson() {
    return {
      'path': path,
      'content': content,
      'overwrite': overwrite,
    };
  }
}

/// Request to delete a file
class DeleteFileRequest {
  final String path;
  final bool recursive;

  const DeleteFileRequest({
    required this.path,
    this.recursive = false,
  });

  Map<String, dynamic> toJson() {
    return {
      'path': path,
      'recursive': recursive,
    };
  }
}

/// Response for file operations
class FileOperationResponse {
  final bool success;
  final String? message;
  final String? error;

  const FileOperationResponse({
    required this.success,
    this.message,
    this.error,
  });

  factory FileOperationResponse.fromJson(Map<String, dynamic> json) {
    return FileOperationResponse(
      success: json['success'] as bool? ?? false,
      message: json['message'] as String?,
      error: json['error'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'success': success,
      'message': message,
      'error': error,
    };
  }
}

/// Represents a directory listing
class DirectoryListing {
  final String path;
  final List<FileInfo> files;
  final int totalCount;

  const DirectoryListing({
    required this.path,
    required this.files,
    required this.totalCount,
  });

  factory DirectoryListing.fromJson(Map<String, dynamic> json) {
    final files = (json['files'] as List<dynamic>? ?? [])
        .map((e) => FileInfo.fromJson(e as Map<String, dynamic>))
        .toList();
    
    return DirectoryListing(
      path: json['path'] as String? ?? '',
      files: files,
      totalCount: json['totalCount'] as int? ?? files.length,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'path': path,
      'files': files.map((f) => f.toJson()).toList(),
      'totalCount': totalCount,
    };
  }
}