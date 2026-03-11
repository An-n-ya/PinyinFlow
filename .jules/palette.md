## 2024-03-11 - Tooltips on Disabled Elements
**Learning:** Tooltips built with Radix UI (like `TooltipPrimitive` in `src/components/ui/tooltip.tsx`) do not trigger on naturally disabled HTML elements (like a `<button disabled>`) because they don't fire pointer events.
**Action:** Always wrap disabled interactive elements in an event-receiving wrapper (like `<span tabIndex={isDisabled ? 0 : -1}>`) and transfer positioning classes to the wrapper to maintain layout while enabling the tooltip.
