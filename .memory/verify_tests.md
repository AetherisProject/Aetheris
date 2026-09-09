# Verify All Tests

## Overview
This document provides a step-by-step guide to verify all tests for Phase 2 features. Since the `Cargo.toml` issue persists, we'll run tests directly in the workspace root.

## Prerequisites
Ensure the `apikey` feature is enabled:
```bash
cargo build --features apikey
```

## Run Tests
### Run All Tests
```bash
cargo test --features apikey --lib
```

### Run Specific Test Files
#### API Key Management Tests
```bash
cargo test --features apikey --path tests/apikey_integration_tests
```

#### Zero-Knowledge Sync Tests
```bash
cargo test --features apikey --path tests/crdt_sync_tests
```

#### Web and Mobile Integration Tests
```bash
cargo test --features apikey --path tests/web_mobile_tests
```

#### Authentication Tests
```bash
cargo test --features apikey --path tests/authentication_tests
```

#### Proactive Engine Tests
```bash
cargo test --features apikey --path tests/proactive_tests
```

## Expected Output
All tests should pass without errors. If any test fails, review the error output and address the issue.

## Manual Verification
If tests fail, manually verify each module's functionality by running the individual test files:
- **API Key Management**: Verify rotation and listing logic.
- **Zero-Knowledge Sync**: Confirm CRDT node operations and sync.
- **Web/Mobile Integration**: Ensure web client and mobile SDK operations.
- **Authentication**: Validate OAuth2 and session management.
- **Proactive Engine**: Check monitoring and alerting systems.

## Conclusion
Run the tests as outlined above to ensure all Phase 2 features are thoroughly tested and verified. If any issues are found, address them immediately.

### Next Steps
- **Document Failures**: If any tests fail, document the failures and fix them.
- **Finalize Documentation**: Ensure all tests are documented in the repository.