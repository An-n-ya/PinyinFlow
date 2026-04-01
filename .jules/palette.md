## 2024-04-01 - Tooltips on Disabled Buttons

**Learning:** Tooltips attached directly to native HTML `<button>` elements do not trigger when the button is `disabled`, because disabled elements do not fire pointer events.
**Action:** Wrap disabled buttons in an event-receiving `<span>` element. Give the wrapper `tabIndex={isDisabled ? 0 : -1}` and `aria-disabled={isDisabled ? 'true' : undefined}`. Transfer layout/positioning classes to the wrapper span, and attach the `TooltipTrigger` to the wrapper. NEVER add `role="button"` to the wrapper, as it confuses screen readers.
