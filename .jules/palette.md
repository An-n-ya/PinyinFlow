## 2024-03-24 - Tooltips on disabled buttons using span wrapper
**Learning:** Tooltips do not display on natively disabled HTML buttons because they don't fire pointer events. Wrapping them in a `<span tabIndex={0} aria-disabled="true">` allows tooltips to trigger, but in headless tests `hover()` on the span might not consistently trigger the tooltip. `focus()` on the wrapper correctly triggers the tooltip display.
**Action:** When writing Playwright tests for tooltips on disabled elements, test using `.focus()` on the wrapper span instead of just `.hover()` to ensure consistent evaluation.
