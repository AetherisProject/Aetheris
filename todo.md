# Aetheris UI/UX Design-System Integration — All 25 Tasks

Phase 1: Design System Export (4) — DONE
- Serialization (tokens.rs)
- CSS-in-JS export
- TypeScript/Dart codegen
- Validation vs design-system.md

Phase 2: CLI TUI Deep (5) — DONE
- Component primitives (ui_components.rs)
- Theme switching (theme.rs)
- Responsive layout (layout.rs)
- Wire vault/ssh/apikey (wire.rs)
- Keyboard navigation / form / modal (added)

Phase 3: Desktop Deep (5) — DONE
- Tauri 2 scaffold (tauri.conf.json, package.json)
- React component library (AetherisComponents.tsx)
- Dashboard/Vault/SSH/API screens (pages/*.tsx)
- i18n fluent (i18n/en.json)
- Build test (test_build.sh)

Phase 4: Mobile Deep (5) — DONE
- Flutter scaffold (pubspec.yaml)
- Dart token translation (design_tokens.dart)
- Widget library (widgets.dart)
- FFI VaultStore (ffi.dart)
- FFI CryptoEngine (ffi_crypto.dart)
- Mobile responsive layout (layout.dart)

Phase 5: Web Deep (5) — DONE
- Vite scaffold (vite.config.ts, package.json)
- Shared component library (Shared.tsx)
- Responsive screens (ResponsiveDashboard.tsx)
- i18n (en.ts)
- Build/test (test.sh)

Phase 6: Browser Deep (5) — DONE
- Manifest MV3 (manifest.json)
- WASM core (src/wasm.rs)
- Popup (popup.html)
- Options (options.html)
- Content script (content.js)
- Build/test (build.sh)
