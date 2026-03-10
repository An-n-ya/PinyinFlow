from playwright.sync_api import sync_playwright

def test_chat_render(page):
    # Setup Tauri mocks for Playwright tests based on project memory
    page.add_init_script("""
        window.__TAURI_INTERNALS__ = {
            invoke: (cmd, args) => {
                if (cmd === 'fetch_user_preferences') return Promise.resolve({ isSidebarOpen: true, enableCompleteInput: true });
                if (cmd === 'fetch_user_profiles') return Promise.resolve([]);
                return Promise.resolve();
            },
            metadata: {
                currentWindow: { label: 'main' },
                currentWebview: { label: 'main' }
            },
            plugins: {
                window: {
                    getCurrentWindow: () => ({ label: 'main', hide: () => Promise.resolve(), close: () => Promise.resolve() }),
                    getAllWindows: () => []
                }
            }
        };
    """)
    page.goto("http://localhost:1420")
    page.wait_for_selector("textarea")
    page.screenshot(path="verification.png")

if __name__ == "__main__":
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        try:
            test_chat_render(page)
        finally:
            browser.close()
