## 2024-03-12 - Prevent InputArea Re-renders

**Learning:** `InputArea` component in `Chat.tsx` was re-rendering unnecessarily every time a new message was added to the chat history, which can become slow as the history grows. This happened because `submit_pinyin` was recreated on every render of `Chat`.

**Action:** Wrap `InputArea` with `React.memo()`, wrap `submit_pinyin` with `useCallback()`, and hoist non-dependent functions like `play` outside of the React component to ensure stable prop references and prevent child component re-renders. Also ensure `MessageType.id` is typed as `string` to match `crypto.randomUUID()` usage.
