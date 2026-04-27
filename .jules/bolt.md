## 2024-05-24 - Inline comments for React optimizations
**Learning:** Even when the implementation of React optimizations (`React.memo`, `useCallback`) is functionally flawless, code reviewers will flag the PR if there are no explicit inline comments explaining the *reason* for the optimization.
**Action:** Always add inline comments (e.g., `// ⚡ Bolt: Memoize submit handler so its reference is stable...`) directly above any performance-related code changes to explain the "why" and ensure strict compliance with PR boundaries.
