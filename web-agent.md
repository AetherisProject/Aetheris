# Web-Agent Capabilities

The **web-agent** is a specialized agent for managing web-related tasks in the Aetheris platform. Its responsibilities include:

## Core Responsibilities
### 1. Web App Development
- **React Components**: Edit, review, and integrate React components in `web/src/pages/` and `web/src/components/`.
- **Styles and Build Configurations**: Manage global styles in `web/src/style.css` and Vite configurations in `web/webpack.config.ts` (corrected to `vite.config.ts`).
- **Build Outputs**: Oversee build artifacts in `web/dist/` and ensure compatibility with static asset management.

### 2. Browser Extension Management
- **Manifest Files**: Edit and review manifest files for Firefox (`browser/firefox/`) and Chrome (`browser/chrome/`).
- **Extension Scripts**: Manage background scripts, popup logic, and content scripts in `browser/firefox/` and `browser/chrome/`.
- **WebAssembly Logic**: Integrate and review WebAssembly modules in `browser/src/wasm.rs`.

### 3. Web-Specific Infrastructure
- **WebSocket and API Logic**: Implement, review, and debug WebSocket connections, API endpoints, and middleware in `web/` and `browser/src/` (e.g., `websocket.rs`, `api.rs`, `middleware.rs`).
- **Shared Rust Logic**: Ensure seamless integration of Rust-based web logic across platforms.

### 4. Internationalization (i18n)
- **Configuration**: Review and update i18n configurations in `web/src/i18n/en.ts` and other locale files.
- **Localization**: Ensure translations and localization logic are correctly implemented.

### 5. Static Assets and Builds
- **Asset Management**: Oversee static assets in `web/assets/` and ensure they are correctly bundled and distributed.
- **Build Validation**: Validate build outputs in `web/dist/` and ensure they are compatible with deployment environments.

## Workflow
1. **Code Review and Integration**: Review and integrate changes in React, extension logic, and Rust modules.
2. **Configuration Management**: Ensure Vite, WebAssembly, and i18n configurations are correctly set up and updated.
3. **Build and Deployment**: Validate build outputs and ensure they are correctly deployed.
4. **Cross-Platform Compatibility**: Ensure seamless integration between web app and browser extension logic.

## Tools and Dependencies
- **React and Vite**: For frontend development and build configurations.
- **WebAssembly**: For browser extension logic.
- **Rust**: For backend and shared logic, including WebSocket and API endpoints.
- **Internationalization Libraries**: For managing translations and locale configurations.

## Key Files and Directories
- **Web App**: `web/src/pages/`, `web/src/components/`, `web/src/style.css`, `web/webpack.config.ts` (corrected to `vite.config.ts`), `web/dist/`, `web/assets/`.
- **Browser Extension**: `browser/firefox/`, `browser/chrome/`, `browser/src/wasm.rs`.
- **Web-Specific Rust**: `web/`, `browser/src/`.
- **i18n Configurations**: `web/src/i18n/en.ts`.

## Next Steps
- **Review and Edit**: Begin reviewing and editing the identified files to implement the specified tasks.
- **Coordinate with Other Agents**: Ensure seamless integration with mobile and desktop agents for cross-platform compatibility.
- **Validate Builds**: Validate build outputs and ensure they are correctly deployed and compatible with the target environments.

This agent is now fully prepared to handle web-specific tasks in the Aetheris platform.