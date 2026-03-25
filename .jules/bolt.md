## 2025-03-05 - Initializing
**Learning:** Nothing yet.
**Action:** None.

## 2025-03-05 - Move pure functions outside React components
**Learning:** Moving static/pure async functions (like `play` which triggers Tauri `invoke` commands) outside of the React component definitions is an effective performance pattern in this codebase. It prevents the functions from being recreated on every render of the component (e.g. `Chat.tsx` which re-renders constantly during typing or streaming), saving unnecessary closures and memory allocation overhead.
**Action:** When working on React components in this Tauri app, check if local functions that only interact with Tauri's external API or external state can be safely moved outside the component definition.
