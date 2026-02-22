## 2024-05-24 - Testing Memoized Components
**Learning:** When using `vi.mock` to count renders of a memoized component, the mock itself must be wrapped in `React.memo`. Otherwise, the mock is just a plain function component and will re-render on parent updates even if props are stable, leading to false negatives in performance tests.
**Action:** Always wrap component mocks in `memo` when testing `React.memo` optimizations.
