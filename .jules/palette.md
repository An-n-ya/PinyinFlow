## 2024-05-03 - Added Tooltip and Focus to Disabled Send Button
**Learning:** Tooltips on disabled buttons require the disabled element to be wrapped in a focusable element to enable hover and focus events, as disabled HTML buttons do not fire pointer events natively.
**Action:** Wrapped the disabled send button in a focusable `span` to ensure the tooltip triggers explaining why it is disabled.
