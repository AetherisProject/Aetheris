# Aetheris Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- **Two-Factor Authentication (TOTP)**: Implemented RFC 6238 compliant TOTP support with QR code generation and verification
- **Biometric Login Support**: Added FaceID, TouchID, and Windows Hello support for secure biometric authentication
- **Family/Enterprise Plan Features**: Implemented family sharing, enterprise teams, plan management, and user role management
- **Firefox Browser Extension**: Added complete Firefox extension support with manifest, popup, and background scripts
- **Feature Comparison Documentation**: Comprehensive comparison against 1Password, Bitwarden, Terminus, and Mushi
- **UI/UX Test Plan**: Detailed test plan with 16 test cases across all platforms
- **Feature Comparison Issues**: 8 GitHub issues for competitor analysis
- **Enhanced Documentation**: Updated CONTEXT.md, AGENTS.md, and created multiple documentation files

### Changed
- **Authentication System**: Enhanced with TOTP and biometric support
- **Billing System**: Added plan management, family sharing, and enterprise features
- **Browser Extension**: Extended to support Firefox in addition to Chrome
- **Documentation**: Comprehensive updates across all documentation files

### Fixed
- **Cargo.toml**: Fixed dependency issues and restored original configuration
- **CI/CD Pipeline**: Verified and tested across all platforms (web, desktop, mobile)

### Security
- **TOTP Implementation**: Secure time-based one-time password generation and verification
- **Biometric Authentication**: Secure biometric credential storage and verification
- **Family Sharing**: Secure vault sharing with role-based access control
- **Enterprise Features**: Secure team management with role-based permissions

---

## [0.1.0] - 2026-09-09

### Added
- **Phase 1: Cryptographic Primitives**: Post-quantum hybrid (Kyber/Dilithium), memory encryption, Duress mode
- **Phase 1: Vault Features**: All VaultItem variants, encryption/decryption, sled backend integration
- **Phase 1: Security Utilities**: SecureString, SecureVec, constant-time comparison
- **Phase 2: API Key Management**: Structure, storage, rotation logic, Vault integration
- **Phase 2: Zero-Knowledge Sync**: CRDT-based sync protocols, data structures, Vault integration
- **Phase 2: Web/Mobile Integration**: Web client compatibility, Flutter SDK, mobile SDK integration
- **Phase 2: Authentication**: OAuth2 provider/client implementation, session management
- **Phase 2: Proactive Engine**: Monitoring, alerting, integration with crypto engine
- **Phase 3: Final Verification**: End-to-end testing, feature validation, security compliance
- **CI/CD Pipeline**: GitHub Actions workflow for screenshot capture across web, desktop, mobile

### Changed
- **Project Structure**: Organized into clear phases with comprehensive documentation
- **Security Compliance**: Strict adherence to all security rules and best practices

### Security
- **Master Password**: Never stored or logged
- **Keys**: Always encrypted using authenticated encryption
- **Randomness**: Uses secure OsRng
- **User Input**: Always validated and sanitized

---

## [0.0.1] - 2026-08-01

### Added
- **Initial Project Setup**: Repository structure, Cargo.toml, basic dependencies
- **Core Architecture**: Modular design with crypto, vault, auth, sync, web, mobile modules
- **Documentation Framework**: Initial documentation structure and templates

---

## Template

### Added
- for new features

### Changed
- for changes in existing functionality

### Deprecated
- for soon-to-be removed features

### Removed
- for now removed features

### Fixed
- for any bug fixes

### Security
- in case of vulnerabilities

---

## Contributing

Please follow the [contribution guidelines](CONTRIBUTING.md) when contributing to this project.