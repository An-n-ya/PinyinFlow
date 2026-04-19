## 2025-05-18 - InputArea Unnecessary Re-renders
**Learning:** In chat interfaces, input areas often re-render unnecessarily when a parent component's `messages` state updates.
**Action:** Use `React.memo` for the Input component and `useCallback` for any event handlers passed down to it from the parent component.
