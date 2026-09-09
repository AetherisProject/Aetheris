//! Flutter SDK method channel for Aetheris.
//!
//! Implements method channel communication between Flutter and native.

import 'package:flutter/services.dart'
    show MethodChannel, PlatformException;

import 'dart:convert'
    show json;

import 'package:aetheris_sdk/aetheris_sdk_platform_interface.dart'
    show AetherisSdkPlatform;

class AetherisSdkMethodChannel extends AetherisSdkPlatform {
  static const _channel = MethodChannel('aetheris_sdk');

  @override
  Future<void> initialize({required String vaultPath}) async {
    try {
      await _channel.invokeMethod('initialize', json.encode({
        'vaultPath': vaultPath,
      }));
    } on PlatformException catch (e) {
      throw Exception('Failed to initialize: ${e.message}');
    }
  }

  @override
  Future<Map<String, dynamic>> insertItem({
    required String title,
    required String password,
    required String notes,
  }) async {
    try {
      final result = await _channel.invokeMethod('insertItem', json.encode({
        'title': title,
        'password': password,
        'notes': notes,
      }));
      return json.decode(result) as Map<String, dynamic>;
    } on PlatformException catch (e) {
      throw Exception('Failed to insert item: ${e.message}');
    }
  }

  @override
  Future<Map<String, dynamic>?> getItem({required String itemId}) async {
    try {
      final result = await _channel.invokeMethod('getItem', json.encode({
        'itemId': itemId,
      }));
      return json.decode(result) as Map<String, dynamic>?;
    } on PlatformException catch (e) {
      throw Exception('Failed to get item: ${e.message}');
    }
  }

  @override
  Future<void> updateItem({
    required String itemId,
    required Map<String, dynamic> item,
  }) async {
    try {
      await _channel.invokeMethod('updateItem', json.encode({
        'itemId': itemId,
        'item': item,
      }));
    } on PlatformException catch (e) {
      throw Exception('Failed to update item: ${e.message}');
    }
  }

  @override
  Future<void> deleteItem({required String itemId}) async {
    try {
      await _channel.invokeMethod('deleteItem', json.encode({
        'itemId': itemId,
      }));
    } on PlatformException catch (e) {
      throw Exception('Failed to delete item: ${e.message}');
    }
  }

  @override
  Future<void> syncData() async {
    try {
      await _channel.invokeMethod('syncData');
    } on PlatformException catch (e) {
      throw Exception('Failed to sync data: ${e.message}');
    }
  }
}