## 2024-05-12 - Tooltips on disabled elements
**Learning:** Disabled elements do not emit pointer events, which prevents hover-based tooltips from appearing. This makes it impossible to convey why a button is disabled to users who rely on those tooltips.
**Action:** When adding tooltips to disabled buttons, conditionally wrap the disabled element in a focusable span (e.g., `<span tabIndex={0}>`), and transfer absolute positioning classes to the span to maintain layout.
