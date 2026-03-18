## 2025-02-13 - Tooltip wrapper for disabled buttons
**Learning:** In Radix UI tooltips, adding tooltips to buttons that can be `disabled` natively does not show the tooltip. HTML disabled elements do not fire pointer events like `hover`.
**Action:** When a button will be disabled, instead of wrapping the button directly with `TooltipTrigger asChild`, wrap the `PromptInputSubmit` button with a `span` with `tabIndex={isDisabled ? 0 : -1}` and apply layout classes (`absolute right-1 bottom-1`) to the span wrapper, and use `TooltipTrigger asChild` on the `span`.
