# Aetheris Mobile Agent

## Overview
This directory contains the mobile implementation for Aetheris, built using Flutter for cross-platform compatibility. The mobile app integrates with the Rust backend via FFI (Foreign Function Interface) and adheres to project conventions.

## Project Structure
- **`mobile/`**: Root directory for the Flutter project.
- **`main.dart`**: Entry point for the mobile app.
- **`pubspec.yaml`**: Dependencies and configurations for the Flutter project.

## Key Features
- **Secure Storage**: Uses `flutter_secure_storage` for encrypted key management.
- **Internationalization**: Supports i18n via `i18n` and `flutter_localizations`.
- **Feature Flags**: Platform-specific code is managed via feature flags in the Rust backend.
- **Rust Integration**: Placeholder for FFI integration with the Rust core for shared logic.

## Setup Instructions
1. **Dependencies**: Ensure Flutter SDK is installed and add dependencies as listed in `pubspec.yaml`.
2. **Build**: Run `flutter pub get` to fetch dependencies.
3. **Platform-Specific Configurations**: Configure `ios/` and `android/` directories for iOS and Android platforms.

## Security Rules
- **Master Password**: Never store or log; use `flutter_secure_storage` for encrypted key management.
- **Encryption**: All sensitive data must use authenticated encryption (AEAD).
- **Random Values**: Use `rand::rngs::OsRng` for cryptographic randomness.

## Integration with Rust
- **FFI**: Define Rust bindings for shared logic (e.g., crypto, vault operations).
- **Feature Flags**: Use Rust feature flags to enable/disable platform-specific code.

## Next Steps
- Implement platform-specific configurations for iOS and Android.
- Define Rust FFI bindings for shared functionality.
- Integrate with the Rust backend for secure operations.

## References
- [Flutter Documentation](https://flutter.dev/docs)
- [Flutter Secure Storage](https://pub.dev/packages/flutter_secure_storage)
- [Flutter Internationalization](https://flutter.dev/docs/cookbook/internationalization)