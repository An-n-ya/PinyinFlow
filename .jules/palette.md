## 2024-05-24 - Tooltip on disabled buttons
**Learning:** Tooltips do not display on natively disabled HTML buttons because disabled elements do not fire pointer events.
**Action:** When using Radix UI TooltipTrigger asChild, conditionally render: when disabled, wrap the disabled button in a focusable <span> (tabIndex={0}) inside the TooltipTrigger; when active, TooltipTrigger asChild must wrap the button directly. NEVER add role="button" to the wrapper span.
