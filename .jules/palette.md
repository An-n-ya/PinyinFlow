## 2024-04-02 - Fix tooltip on disabled buttons in InputArea
**Learning:** Tooltips do not display on natively disabled HTML buttons because disabled elements do not fire pointer events.
**Action:** Wrap disabled buttons in an event-receiving element (e.g., `<span tabIndex={isDisabled ? 0 : -1} aria-disabled={isDisabled ? 'true' : undefined}>`) and attach the `TooltipTrigger` to the wrapper, transferring layout classes to it.
