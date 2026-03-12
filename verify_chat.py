from playwright.sync_api import sync_playwright

def verify_chat_rendering():
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page()

        # Inject mock Tauri API
        page.add_init_script("""
            window.__TAURI_INTERNALS__ = {
                invoke: async (cmd, args) => {
                    if (cmd === 'fetch_user_preferences') return { isSidebarOpen: true };
                    if (cmd === 'fetch_user_profiles') return [];
                    if (cmd === 'proofread') return args.input;
                    if (cmd === 'play') return;
                    console.log('Mock invoked:', cmd, args);
                },
                metadata: {
                    currentWindow: { label: 'main' },
                    currentWebview: { label: 'main' }
                },
                plugins: {
                    window: {
                        getCurrentWindow: () => ({ label: 'main' }),
                        getAllWindows: () => []
                    }
                }
            };
        """)

        # Start Vite dev server in background and wait for it
        import subprocess
        import time
        server = subprocess.Popen(["pnpm", "dev"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        time.sleep(5) # Wait for Vite to start

        try:
            page.goto("http://localhost:1420")
            page.wait_for_selector("text='你好，这是一个测试'")
            page.screenshot(path="chat_verification.png")
            print("Verification successful!")
        finally:
            server.terminate()
            browser.close()

if __name__ == "__main__":
    verify_chat_rendering()
