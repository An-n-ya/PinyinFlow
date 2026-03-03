import time
from playwright.sync_api import sync_playwright

def run(playwright):
    browser = playwright.chromium.launch(headless=True)
    context = browser.new_context(viewport={'width': 800, 'height': 600})
    page = context.new_page()
    page.goto("http://localhost:1420")

    # Wait for the textarea to be visible
    textarea = page.wait_for_selector("textarea")

    # 1. Verify Placeholder
    placeholder = textarea.get_attribute("placeholder")
    print(f"Placeholder: {placeholder}")
    if placeholder != "请输入拼音...":
        print("ERROR: Placeholder is incorrect!")

    # 2. Verify Space key works (doesn't submit)
    textarea.fill("test")
    textarea.press("Space")
    textarea.type("space")
    content = textarea.input_value()
    print(f"Content after space: '{content}'")
    if content != "test space":
        print("ERROR: Space key did not work as expected!")

    # 3. Verify Tooltip on Model Button
    # Model button contains text "模型"
    model_btn = page.get_by_role("button").filter(has_text="模型")
    model_btn.hover()
    time.sleep(1) # Wait for tooltip
    page.screenshot(path="tooltip_model.png")

    # Check if tooltip content is visible
    if page.get_by_text("选择模型").is_visible():
        print("SUCCESS: Model tooltip visible")
    else:
        print("ERROR: Model tooltip not visible")

    # Move mouse away to close tooltip
    page.mouse.move(0, 0)
    time.sleep(0.5)

    # 4. Verify Tooltip on Submit Button
    # Submit button is the one with aria-label="发送"
    submit_btn = page.get_by_label("发送")
    submit_btn.hover()
    time.sleep(1)
    page.screenshot(path="tooltip_submit.png")

    # The tooltip text is "发送". The button aria-label is also "发送".
    # We want to check if the visible text "发送" appears (tooltip content).
    # Since the button itself only has an icon (no text), get_by_text("发送") should find the tooltip.
    if page.get_by_text("发送", exact=True).is_visible():
        print("SUCCESS: Submit tooltip visible")
    else:
        print("ERROR: Submit tooltip not visible")

    # 5. Verify Enter key sends message
    textarea.fill("message 1")
    textarea.press("Enter")
    time.sleep(1)
    # Check if message appears in history
    # The Chat updates messages list.
    if page.get_by_text("message 1").count() > 0:
         print("SUCCESS: Enter key sent message")
    else:
         print("ERROR: Enter key did not send message")

    # 6. Verify Shift+Enter inserts newline
    textarea.fill("line 1")
    textarea.press("Shift+Enter")
    textarea.type("line 2")
    content = textarea.input_value()
    if "line 1\nline 2" in content:
        print("SUCCESS: Shift+Enter inserted newline")
    else:
        print(f"ERROR: Shift+Enter failed. Content: {content!r}")

    browser.close()

with sync_playwright() as playwright:
    run(playwright)
