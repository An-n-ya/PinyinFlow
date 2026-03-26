## 2024-05-19 - Enable tooltip on disabled button
**Learning:** Tooltips do not display on natively disabled HTML buttons because disabled elements do not fire pointer events.
**Action:** Wrap disabled buttons in an event-receiving element (e.g., `<span tabIndex={isDisabled ? 0 : -1} aria-disabled={isDisabled ? 'true' : undefined}>`). NEVER add `role="button"` to this wrapper span when wrapping a native `<button>`, as it creates an accessibility regression (nested interactive elements) that confuses screen readers. Attach the `TooltipTrigger` to the wrapper, and transfer layout classes to it.
