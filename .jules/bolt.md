
## 2025-03-08 - Stable Callbacks with Tauri Commands
**Learning:** In a React functional component (`Chat.tsx`), defining async functions like `play` that wrap Tauri `invoke` calls *inside* the component creates unstable references on every render. If these functions are used within a `useCallback` dependency array (e.g., for `submit_pinyin`), the callback also becomes unstable, completely breaking `React.memo` optimizations on child components (`InputArea`).
**Action:** Always extract static async logic (like raw Tauri `invoke` wrappers) outside of the React component body to ensure a stable reference, allowing `useCallback` to effectively memoize handlers passed to heavily-rerendered children.
