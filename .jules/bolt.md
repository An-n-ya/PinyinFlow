## 2025-05-20 - Type Mismatch in Frontend/Backend Interfaces
**Learning:** `src/vite-env.d.ts` defined `MessageType.id` as `number`, but the implementation (`Chat.tsx`) and backend (via `invoke`) use UUID strings. This caused build failures when stricter type checking was applied during `pnpm build`.
**Action:** Always verify that interface definitions in `d.ts` files match the actual data structures used in the application. Always run `pnpm build` before submitting to catch type errors.

## 2025-05-20 - Unnecessary Re-renders in Chat Input
**Learning:** `InputArea` was re-rendering on every message update because `onSendMessage` (passed as `submit_pinyin`) was recreated on every render of `Chat.tsx`.
**Action:** Use `useCallback` for event handlers passed to heavy child components, and wrap those child components in `React.memo`.
