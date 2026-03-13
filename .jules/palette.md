
## 2024-05-18 - Tooltips on Disabled Buttons
**Learning:** Natively disabled HTML buttons do not fire pointer events and are removed from tab order, meaning tooltips attached directly to them will never trigger on hover or focus.
**Action:** Always wrap disabled interactive elements in a non-disabled element (like a `span`) configured with dynamic `tabIndex={isDisabled ? 0 : -1}`. Attach the `TooltipTrigger` to this wrapper and transfer positional layout classes to the wrapper to prevent visual regressions while ensuring accessibility.
