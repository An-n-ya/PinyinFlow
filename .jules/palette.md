## 2024-05-24 - Tooltips on Disabled Buttons
**Learning:** Tooltips do not naturally appear on disabled buttons because they do not emit pointer events. The Radix `TooltipTrigger asChild` pattern requires a focusable wrapper for disabled elements to ensure both hover and keyboard accessibility.
**Action:** Wrap disabled buttons in a `<span tabIndex={0} className="inline-flex">` conditionally to capture events without breaking layouts or overriding focus behavior when the button is active. Also, ensure the tooltip content explains *why* the button is disabled.
