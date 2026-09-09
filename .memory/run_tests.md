# Running Tests Directly

## Overview
This document provides instructions for running all tests directly in the Eval environment.

## Setup
Ensure the `apikey` feature is enabled:
```bash
cargo build --features apikey
```

## Run Tests
### API Key Management Tests
```bash
cargo test --features apikey --path tests/apikey_integration_tests --lib
```

### Zero-Knowledge Sync Tests
```bash
cargo test --features apikey --path tests/crdt_sync_tests --lib
```

### Web and Mobile Integration Tests
```bash
cargo test --features apikey --path tests/web_mobile_tests --lib
```

### Authentication Tests
```bash
cargo test --features apikey --path tests/authentication_tests --lib
```

### Proactive Engine Tests
```bash
cargo test --features apikey --path tests/proactive_tests --lib
```

## Expected Output
All tests should pass without errors. If any test fails, review the error output and fix the issue.

## Conclusion
Run the tests as outlined above to ensure all Phase 2 features are thoroughly tested and verified.

### Next Steps
- **Review Test Outputs**: Ensure all tests pass.
- **Document Failures**: If any tests fail, document the failures and fix them.

All tests should confirm the correctness and robustness of the implementation.