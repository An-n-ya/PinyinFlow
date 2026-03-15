## 2024-03-15 - Tooltips on Disabled Elements
**Learning:** Tooltips on natively disabled HTML buttons fail to display because disabled elements do not fire pointer events (like hover) required by Radix UI tooltips.
**Action:** When adding tooltips to buttons that may be disabled, wrap the disabled button in an event-receiving element (like a `<span tabIndex={0}>`), attach the `TooltipTrigger asChild` to the wrapper, and transfer layout classes (like absolute positioning) to the wrapper to preserve the design while fixing the UX interaction.
