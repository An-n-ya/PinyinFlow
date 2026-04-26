from playwright.sync_api import sync_playwright

def run_cuj(page):
    # Setup Tauri mocks via initialization script
    page.add_init_script("""
        window.__TAURI_INTERNALS__ = {
            metadata: {
                currentWindow: { label: 'main' },
                currentWebview: { label: 'main' }
            },
            invoke: async (cmd, args) => {
                if (cmd === 'plugin:window|internal_get_current_window') {
                    return { label: 'main' };
                }
                if (cmd === 'plugin:window|get_all_windows') {
                    return [{ label: 'main' }];
                }
                if (cmd === 'get_messages') return [];
                if (cmd === 'fetch_user_models') return [];
                if (cmd === 'get_models') return [];
                if (cmd === 'fetch_user_profiles') return { userName: 'test', id: '1' };
                if (cmd === 'fetch_user_preferences') return { isSidebarOpen: true, enableCompleteInput: true };
                return null;
            }
        };
    """)
    page.goto("http://localhost:1420")
    page.wait_for_timeout(2000)

    span_locator = page.locator('span:has(> button[aria-label="发送"][disabled])')
    span_locator.wait_for(state="visible", timeout=10000)

    span_locator.hover()
    page.wait_for_timeout(1000)

    page.screenshot(path="/home/jules/verification/screenshots/verification.png")
    page.wait_for_timeout(1000)

if __name__ == "__main__":
    import os
    os.makedirs("/home/jules/verification/videos", exist_ok=True)
    os.makedirs("/home/jules/verification/screenshots", exist_ok=True)

    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(
            record_video_dir="/home/jules/verification/videos"
        )
        page = context.new_page()
        try:
            run_cuj(page)
        finally:
            context.close()
            browser.close()
