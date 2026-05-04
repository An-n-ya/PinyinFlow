## 2024-05-15 - React.memo on InputArea
**Learning:** Frequent backend events (like `tts-finished` and `audio-played`) causing parent state updates will trigger expensive re-renders in child components like `InputArea` if they are not properly memoized, leading to input jank.
**Action:** Always wrap heavy input components in `React.memo` and ensure stable prop references using `useCallback` when parent components handle frequent event subscriptions.
