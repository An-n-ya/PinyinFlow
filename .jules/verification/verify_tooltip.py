from playwright.sync_api import sync_playwright
import os

def run_cuj(page):
    page.goto("http://localhost:1420")
    page.wait_for_timeout(2000)

    # Focus the span wrapping the submit button
    submit_span = page.locator("span[aria-disabled='true']")
    submit_span.focus()

    page.wait_for_timeout(1000)
    page.screenshot(path=".jules/verification/screenshots/verification.png")
    page.wait_for_timeout(1000)

if __name__ == "__main__":
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(
            record_video_dir=".jules/verification/videos"
        )

        context.add_init_script("""
            window.__TAURI_INTERNALS__ = {
                metadata: {
                    currentWindow: { label: 'main' },
                    currentWebview: { label: 'main' }
                },
                invoke: function(cmd, args) {
                    if (cmd === 'plugin:window|internal_get_current_window') return { label: 'main' };
                    if (cmd === 'plugin:window|get_all_windows') return [{ label: 'main' }];

                    if (cmd === 'get_messages') return [];
                    if (cmd === 'fetch_user_models') return [];
                    if (cmd === 'get_models') return [];
                    if (cmd === 'fetch_user_profiles') return { userName: 'test', id: '1' };
                    if (cmd === 'fetch_user_preferences') return { isSidebarOpen: true, enableCompleteInput: true };

                    if (cmd === 'proofread') return args.input;
                    if (cmd === 'play') return;

                    console.log('Mock unhandled cmd:', cmd, args);
                    return null;
                }
            };
        """)

        page = context.new_page()
        try:
            run_cuj(page)
        finally:
            context.close()
            browser.close()
