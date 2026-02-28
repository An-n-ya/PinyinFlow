
## 2024-02-28 - Tooltips for System Controls
**Learning:** Adding `aria-label` along with visual tooltips via `radix-ui` wrapper elements greatly improves accessibility for icon-only system controls like 'Close Window', which previously relied only on visual cues.
**Action:** When working on icon buttons, always ensure an `aria-label` is present. If wrapping in a Tooltip, the `aria-label` on the button itself is sufficient and preferable to adding redundant screen-reader-only `<span>` elements, which can cause duplicate announcements.
