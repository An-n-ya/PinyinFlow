## 2024-03-12 - Tooltips on Disabled Buttons
**Learning:** In this application's design system using Radix UI tooltips, tooltips do not appear on natively disabled HTML buttons (like `PromptInputSubmit`) because disabled elements stop firing pointer events.
**Action:** Always wrap conditionally disabled buttons in an event-receiving wrapper (like `<span tabIndex={isDisabled ? 0 : -1}>`) and attach the `TooltipTrigger` to the wrapper. Also, transfer layout classes like `absolute` positioning to the wrapper to maintain layout.
