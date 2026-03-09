## 2025-03-09 - Tooltips on Disabled Elements
**Learning:** Natively disabled HTML elements (like `disabled={true}` on a button) do not fire pointer events. This prevents hover-based tooltips (such as those from Radix UI) from appearing, reducing accessibility as users receive no context for *why* an action is unavailable.
**Action:** Wrap disabled buttons inside an event-receiving wrapper (like a `<span tabIndex={0}>`) and apply the `TooltipTrigger` to the wrapper rather than the disabled element itself, while maintaining layout constraints like `absolute` positioning on the wrapper.
