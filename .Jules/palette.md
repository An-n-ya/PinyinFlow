## 2024-05-24 - Tauri App Testing in Headless Environments
**Learning:** Testing Tauri React apps in a headless browser (Playwright) is extremely brittle because the app often has immediate, synchronous dependencies on `window.__TAURI__` APIs (like `getCurrentWindow`) during the initial render phase (e.g., in `RootSideEffects`). Standard web mocks often fail because the app expects the Tauri environment to be fully present before the first paint.
**Action:** When working with Tauri apps, either:
1. Ensure the app has a "web mode" flag that bypasses Tauri API calls.
2. Invest in a robust, reusable Playwright init script that fully mocks the Tauri 2.x API surface (Window, Webview, Core, Event) *before* navigation.
3. For micro-UX changes, rely on TypeScript compilation and unit tests for logic, acknowledging that E2E verification might require a fully built binary or a specialized test runner.
