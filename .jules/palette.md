## 2024-04-22 - Accessible Tooltips for Disabled Buttons
**Learning:** Tooltips do not display natively on disabled HTML buttons because they don't fire pointer events.
**Action:** When a button can be disabled and uses a tooltip, conditionally wrap the disabled state in a focusable `span` (`tabIndex={0}`) to capture pointer events for the tooltip, but do not wrap the active button to preserve `aria-describedby` semantics. Also, ensure the tooltip content dynamically explains *why* the button is disabled.
