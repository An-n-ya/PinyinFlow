## 2024-05-20 - Screen Reader Double-Announcements
**Learning:** Combining an `aria-label` attribute with a visually hidden `<span className="sr-only">` on a single button causes screen readers to double-announce the label.
**Action:** When adding accessibility labels to icon-only buttons, use either `aria-label` OR a hidden `sr-only` span, but never both simultaneously. Also, if adding a Tooltip, ensure the `aria-label` matches or is sufficient.
