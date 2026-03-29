from playwright.sync_api import sync_playwright

def run_cuj(page):
    page.goto("http://localhost:1420")
    page.wait_for_timeout(2000)

    # Focus the wrapper span around the disabled send button
    page.locator('span[aria-label="发送"], span[aria-disabled="true"]').first.focus()
    page.wait_for_timeout(1000)

    page.screenshot(path=".jules/tooltip_verification.png")

if __name__ == "__main__":
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context()
        page = context.new_page()

        page.add_init_script("""
            window.__TAURI_INTERNALS__ = {
                metadata: { currentWindow: { label: 'main' }, currentWebview: { label: 'main' } },
                invoke: function(cmd, args) {
                    if (cmd === 'plugin:window|internal_get_current_window') return Promise.resolve({ label: 'main' });
                    if (cmd === 'plugin:window|get_all_windows') return Promise.resolve([{ label: 'main' }]);
                    if (cmd === 'get_messages') return Promise.resolve([]);
                    if (cmd === 'fetch_user_models') return Promise.resolve([]);
                    if (cmd === 'get_models') return Promise.resolve([]);
                    if (cmd === 'fetch_user_profiles') return Promise.resolve({ userName: 'test', id: '1' });
                    if (cmd === 'fetch_user_preferences') return Promise.resolve({ isSidebarOpen: true, enableCompleteInput: true });
                    return Promise.resolve(null);
                }
            };
        """)

        try:
            run_cuj(page)
        finally:
            context.close()
            browser.close()
