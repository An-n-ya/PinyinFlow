## 2026-05-01 - Prevent InputArea Re-renders in Chat Component

**Learning:** Frequent backend events (like `tts-finished` and `audio-played`) trigger state updates in the `Chat` component's `messages` array, causing all child components, including the `InputArea`, to re-render unnecessarily.

**Action:** Wrap `InputArea` with `React.memo` to skip re-renders when its props haven't changed. To ensure this optimization works, stabilize its callback props using `useCallback` in the parent `Chat` component (`submit_pinyin` and its dependency `play`).
