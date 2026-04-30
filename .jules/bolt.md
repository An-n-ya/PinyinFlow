## 2024-05-01 - Optimizing React context re-renders with memoization
**Learning:** In React components that subscribe to frequently updating parent state (like a chat application where new messages are added, or audio/TTS states update), child components like `InputArea` will re-render unnecessarily on every parent update if not memoized.
**Action:** Always wrap child components in `React.memo` and pass stable references (using `useCallback`) for event handlers (e.g., `onSendMessage`, `play`) to prevent expensive and unnecessary DOM reconciliations.
