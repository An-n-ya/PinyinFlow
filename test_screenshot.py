import time
from playwright.sync_api import sync_playwright

def run(playwright):
    browser = playwright.chromium.launch(headless=True)
    context = browser.new_context()
    context.add_init_script("""
        window.__TAURI_INTERNALS__ = {
            invoke: (cmd, args) => {
                if (cmd === 'fetch_user_preferences') {
                    return Promise.resolve({
                        userId: 'mock',
                        isSidebarOpen: true,
                        enableCompleteInput: false
                    });
                }
                if (cmd === 'fetch_user_profiles') {
                    return Promise.resolve({
                        id: 'mock',
                        name: 'mock',
                        avatar: ''
                    });
                }
                return Promise.resolve();
            },
            metadata: {
                currentWindow: { label: "main" },
                currentWebview: { label: "main" }
            },
            plugins: {
                window: {
                    getCurrentWindow: () => ({ label: 'main', listen: () => Promise.resolve(), onCloseRequested: () => {}, destroy: () => Promise.resolve() }),
                    getCurrentWebviewWindow: () => ({ label: 'main', listen: () => Promise.resolve() }),
                    getAllWindows: () => []
                },
                webview: {
                    getCurrentWebview: () => ({ listen: () => Promise.resolve() })
                },
                core: {
                    invoke: (cmd, args) => {
                        if (cmd === 'fetch_user_preferences') {
                            return Promise.resolve({
                                userId: 'mock',
                                isSidebarOpen: true,
                                enableCompleteInput: false
                            });
                        }
                        if (cmd === 'fetch_user_profiles') {
                            return Promise.resolve({
                                id: 'mock',
                                name: 'mock',
                                avatar: ''
                            });
                        }
                        return Promise.resolve();
                    }
                },
                event: { listen: () => Promise.resolve() }
            }
        };
        window.__TAURI__ = window.__TAURI_INTERNALS__.plugins;
    """)
    page = context.new_page()
    page.goto("http://localhost:1420")

    # Wait for page to load fully
    page.wait_for_load_state("networkidle")
    time.sleep(2)

    # Take a full page screenshot to see what's rendering
    page.screenshot(path="screenshot.png", full_page=True)

    browser.close()

with sync_playwright() as playwright:
    run(playwright)
