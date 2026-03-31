## 2025-03-05 - Tooltip on Disabled Buttons
**Learning:** Tooltips do not display on natively disabled HTML buttons because disabled elements do not fire pointer events.
**Action:** Wrap disabled buttons in an event-receiving element (e.g., `<span tabIndex={isDisabled ? 0 : -1} aria-disabled={isDisabled ? 'true' : undefined}>`) and attach the `TooltipTrigger` to the wrapper, transferring any necessary layout classes to it. NEVER add `role="button"` to this wrapper when wrapping a native `<button>`.
