
## 2024-05-24 - Unnecessary Re-renders from Frequent Event Listeners
**Learning:** In a React component tree, if a parent component (`Chat`) updates state frequently in response to global events (like `tts-finished` or `audio-played`), it can cause complex child components (`InputArea`) to needlessly re-render.
**Action:** When a child component receives callbacks from a frequently updating parent, wrap the child in `React.memo` and wrap the passed callbacks in `useCallback` to prevent continuous and unnecessary rendering cycles.
