## 2024-04-10 - Radix UI Tooltips on Disabled Buttons

**Learning:** When using Radix UI `TooltipTrigger asChild` with natively disabled HTML buttons, tooltips fail to display because disabled elements do not fire pointer events.
**Action:** Unconditionally wrapping the button in a `span` breaks accessibility by applying `aria-describedby` to the span rather than the button. Instead, conditionally render the `TooltipTrigger`: when disabled, wrap the disabled button in a focusable `<span>` (`tabIndex={0}`) to capture pointer events while keeping the button disabled and pointer-events-none; when active, `TooltipTrigger asChild` must wrap the button directly. Never add `role="button"` to the wrapper span.
