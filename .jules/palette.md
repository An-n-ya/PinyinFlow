## 2024-05-15 - Add Tooltip Support for Disabled Elements
**Learning:** Tooltips do not display on natively disabled HTML buttons because disabled elements do not fire pointer events.
**Action:** When using tooltips with disabled elements, conditionally wrap the disabled element in a focusable `<span>` (`tabIndex={0}`) inside the `TooltipTrigger` to restore tooltip behavior and ensure keyboard accessibility.
