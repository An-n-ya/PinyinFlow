## 2026-03-11 - Component Memoization
**Learning:** In React, passing unstable callbacks (like `submit_pinyin`) to complex child components (`InputArea`) causes unnecessary re-renders when parent state (`messages`) updates. Using `useCallback` and `React.memo` prevents this.
**Action:** Always verify if child components with complex inputs can be memoized, and ensure their callback props are stabilized with `useCallback`.
