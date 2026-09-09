//! Platform interface for Aetheris SDK.
//!
//! Defines the platform interface for Aetheris SDK.

import 'dart:async'
    show Future;

abstract class AetherisSdkPlatform {
  static const String channel = 'aetheris_sdk_platform';

  static const MethodChannel _channel = MethodChannel(channel);

  // Platform-specific methods
  Future<void> initialize({required String vaultPath});
  Future<Map<String, dynamic>> insertItem({
    required String title,
    required String password,
    required String notes,
  });
  Future<Map<String, dynamic>?> getItem({required String itemId});
  Future<void> updateItem({
    required String itemId,
    required Map<String, dynamic> item,
  });
  Future<void> deleteItem({required String itemId});
  Future<void> syncData();
}

// Default implementation
class AetherisSdkPlatformImpl extends AetherisSdkPlatform {
  @override
  Future<void> initialize({required String vaultPath}) async {
    // Default implementation
    throw UnimplementedError('Not implemented');
  }

  @override
  Future<Map<String, dynamic>> insertItem({
    required String title,
    required String password,
    required String notes,
  }) async {
    // Default implementation
    throw UnimplementedError('Not implemented');
  }

  @override
  Future<Map<String, dynamic>?> getItem({required String itemId}) async {
    // Default implementation
    throw UnimplementedError('Not implemented');
  }

  @override
  Future<void> updateItem({
    required String itemId,
    required Map<String, dynamic> item,
  }) async {
    // Default implementation
    throw UnimplementedError('Not implemented');
  }

  @override
  Future<void> deleteItem({required String itemId}) async {
    // Default implementation
    throw UnimplementedError('Not implemented');
  }

  @override
  Future<void> syncData() async {
    // Default implementation
    throw UnimplementedError('Not implemented');
  }
}