## 2024-05-25 - Tooltip on Disabled Submit Button
**Learning:** Tooltips do not display on natively disabled HTML buttons because disabled elements do not fire pointer events.
**Action:** Wrap disabled buttons in an event-receiving `span` with `tabIndex` and `aria-disabled`. Attach the `TooltipTrigger` to the wrapper, and transfer layout classes to it. NEVER add `role="button"` to this wrapper span.
