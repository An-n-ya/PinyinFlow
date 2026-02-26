## 2025-05-23 - Tauri v2 Mocking in Playwright
**Learning:** To test Tauri v2 apps in Playwright, mocking `window.__TAURI__.window` is insufficient. You must mock `window.__TAURI_INTERNALS__.metadata.currentWindow` (and likely `currentWebview`) because `@tauri-apps/api` internals access these properties during initialization (e.g., in `getCurrentWindow`).
**Action:** Always include a comprehensive `__TAURI_INTERNALS__` mock with metadata in Playwright init scripts for Tauri v2 projects.
