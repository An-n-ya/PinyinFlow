## 2024-05-19 - React Memoization for Chat Interface
**Learning:** In a highly interactive chat component receiving frequent backend state updates (like `tts-finished` and `audio-played`), passing unmemoized callbacks down to complex children like `InputArea` forces unnecessary re-renders.
**Action:** Use `React.memo` on the complex child component (`InputArea`) and `useCallback` on the parent's handler functions (`play`, `submit_pinyin`) to maintain stable prop references and prevent rendering bottlenecks on every state update.
