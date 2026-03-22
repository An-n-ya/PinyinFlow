## 2024-05-19 - Pure functions outside components
**Learning:** React performance pattern: Moving static or pure functions (like `play` in `Chat.tsx`) outside of the component definition prevents them from being recreated on every render, reducing memory allocations and garbage collection pressure without altering behavior.
**Action:** Identify and extract pure functions that do not depend on component state or props to the top level of the file where appropriate to optimize rendering performance.
