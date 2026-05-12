1. **Explore codebase:**
   - Look at `src/components/chat/InputArea.tsx` where the submit button is rendered.
   - Verify that the disabled state currently prevents the tooltip from showing because pointer events are swallowed.

2. **Implement conditional tooltip wrapper for disabled button:**
   - Modify `src/components/chat/InputArea.tsx` to conditionally wrap `PromptInputSubmit` in a `<span tabIndex={0} className="absolute right-1 bottom-1">` when disabled.
   - Transfer the absolute positioning classes to the span when disabled, leaving the inner button without them.
   - Update `TooltipContent` to dynamically show "请输入内容" (Please enter content) when disabled, or "发送" (Send) when enabled.

3. **Verify Implementation:**
   - Use `pnpm format:check` to ensure the formatting passes.
   - Build using `pnpm build` or test using `pnpm test`.

4. **Add Journal Entry:**
   - Create or append to `.jules/palette.md` noting the pattern of wrapping disabled buttons in a focusable span and transferring absolute positioning classes.

5. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**

6. **Submit PR:**
   - Commit the changes and submit the PR.
