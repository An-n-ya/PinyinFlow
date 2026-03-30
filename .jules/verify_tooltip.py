import time
from playwright.sync_api import sync_playwright

def run(playwright):
    browser = playwright.chromium.launch(headless=True)
    # Important: mock tauri internal window APIs for the test to avoid crash
    context = browser.new_context(viewport={'width': 800, 'height': 600})
    context.add_init_script("""
        window.__TAURI_INTERNALS__ = {
            invoke: (cmd, args) => {
                if (cmd === 'plugin:window|internal_get_current_window') {
                    return Promise.resolve({ label: 'main' });
                }
                if (cmd === 'plugin:window|get_all_windows') {
                    return Promise.resolve([{ label: 'main' }]);
                }
                if (cmd === 'get_messages') {
                    return Promise.resolve([]);
                }
                if (cmd === 'fetch_user_models' || cmd === 'get_models') {
                    return Promise.resolve([]);
                }
                if (cmd === 'fetch_user_profiles') {
                    return Promise.resolve({ userName: 'test', id: '1' });
                }
                if (cmd === 'fetch_user_preferences') {
                    return Promise.resolve({ isSidebarOpen: true, enableCompleteInput: true });
                }
                return Promise.resolve();
            },
            metadata: {
                currentWindow: { label: 'main' },
                currentWebview: { label: 'main' }
            }
        };
    """)
    page = context.new_page()
    page.goto("http://localhost:1420")

    # Wait for the textarea to be visible
    textarea = page.wait_for_selector("textarea")

    # The button starts disabled because input is empty
    # Submit button is the one with aria-label="发送"
    submit_btn_wrapper = page.locator('span[aria-disabled="true"]').filter(has=page.get_by_label("发送"))

    # Use focus instead of hover for the span to reliably trigger tooltip
    submit_btn_wrapper.focus()
    time.sleep(1)
    page.screenshot(path=".jules/tooltip_submit_disabled.png")

    # Check if tooltip content is visible
    if page.get_by_text("发送", exact=True).is_visible():
        print("SUCCESS: Submit tooltip visible on disabled button")
    else:
        print("ERROR: Submit tooltip not visible on disabled button")

    browser.close()

with sync_playwright() as playwright:
    run(playwright)
