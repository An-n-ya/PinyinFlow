## 2025-03-03 - Disabled Button Tooltips
**Learning:** Tooltips do not display on natively disabled buttons (`<button disabled>`) because disabled interactive elements do not fire pointer events in the DOM, making them invisible to hover triggers.
**Action:** When a tooltip is needed on a disabled button (e.g., to explain why it is disabled or to just identify it), wrap the button in an element that *can* receive pointer events, like a `<span tabIndex={0}>`, and attach the tooltip trigger to the wrapper instead.
