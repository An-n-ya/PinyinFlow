## 2024-04-12 - Accessibility improvement for disabled Tooltip Trigger buttons
**Learning:** Tooltips do not display on natively disabled HTML buttons because disabled elements do not fire pointer events.
**Action:** When using Radix UI `TooltipTrigger asChild`, conditionally wrap the disabled button in a focusable `<span>` (`tabIndex={0}`) inside the `TooltipTrigger`. For active buttons, wrap the button directly.
