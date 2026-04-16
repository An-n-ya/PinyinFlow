## 2024-04-16 - Prevent Unnecessary Re-renders on Message Updates
**Learning:** In a chat application structure where the chat history and input area are siblings under a parent component holding the message state, every new message addition forces a re-render of the input area. This can be problematic if the input area has complex logic or animations.
**Action:** Always verify if sibling components that don't depend on the frequently changing state (like chat history) can be memoized using `React.memo` and ensuring stable prop references via `useCallback`.
