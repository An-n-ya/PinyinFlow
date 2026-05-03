## 2026-05-03 - Prevent unnecessary InputArea re-renders
**Learning:** Components subject to frequent parent state updates (like Chat receiving backend events) cause expensive child re-renders. We should use React.memo and useCallback to provide stable prop references.
**Action:** Always wrap InputArea in React.memo and pass stable prop references (like submit_pinyin and play wrapped in useCallback) to maintain optimal rendering performance.
