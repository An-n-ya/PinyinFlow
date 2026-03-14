## 2024-03-01 - Tooltips on disabled HTML buttons
**Learning:** Tooltips do not display on natively disabled HTML buttons because disabled elements do not fire pointer events.
**Action:** When using a wrapper `span` to enable tooltips on disabled HTML buttons, do not apply `pointer-events-none` to the wrapper. Set `tabIndex` appropriately to make it focusable when disabled so screen readers and keyboards can trigger the tooltip. Also, transfer layout classes like `absolute` to the wrapper to maintain component positioning.
