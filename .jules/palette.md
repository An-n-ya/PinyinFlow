## 2024-03-20 - Disabled Button Tooltips
**Learning:** In this design system, disabled buttons (like `PromptInputSubmit`) natively block pointer events (`disabled:pointer-events-none`), meaning tooltips attached directly to them will never trigger.
**Action:** When adding tooltips to buttons that can be disabled, wrap the button in a focusable `span` (`<span tabIndex={isDisabled ? 0 : -1}>`) and attach the `TooltipTrigger` and any positioning classes (e.g. `absolute right-1 bottom-1`) to the wrapper instead of the button. Ensure Playwright tests target the wrapper for hover events.
