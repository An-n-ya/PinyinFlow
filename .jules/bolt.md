## 2024-05-23 - Type Mismatch Blocked Build
**Learning:** Found critical type mismatch in `vite-env.d.ts` where `id` was `number` but codebase uses UUID strings. This prevented `pnpm build` from succeeding, blocking optimization verification.
**Action:** Always verify `pnpm build` passes before starting optimization to ensure a clean baseline, especially when type definitions are involved.
