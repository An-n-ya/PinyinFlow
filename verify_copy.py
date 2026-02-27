from playwright.sync_api import sync_playwright

def run():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(
            permissions=['clipboard-read', 'clipboard-write']
        )

        # Enhanced Mock Tauri internals
        # We need to mock window.__TAURI__.window.getCurrentWindow() immediately
        # because RootSideEffects.tsx calls it on mount.

        init_script = """
        // 1. Mock window.__TAURI_INTERNALS__ (used by invoke under the hood sometimes)
        window.__TAURI_INTERNALS__ = {
            invoke: async (cmd, args) => {
                console.log(`[Mock] Invoke: ${cmd}`, args);
                if (cmd === 'get_current_user') return { id: 1, name: 'Test User' };
                if (cmd === 'complete_message') return { event: 'finished', data: {} };
                if (cmd === 'proofread') return args.input;
                if (cmd === 'play') return {};
                return {};
            },
            metadata: {
                currentWindow: { label: 'main' }
            }
        };

        // 2. Mock window.__TAURI__ (v2 API)
        // Ensure structure exists deeply enough
        window.__TAURI__ = {
            core: {
                invoke: async (cmd, args) => {
                     console.log(`[Mock Core] Invoke: ${cmd}`, args);
                     if (cmd === 'get_current_user') return { id: 1, name: 'Test User' };
                     return {};
                }
            },
            event: {
                listen: async () => (() => {}),
                emit: async () => {},
            },
            window: {
                getCurrentWindow: () => {
                    return {
                        label: 'main',
                        listen: async () => (() => {}),
                        emit: async () => {},
                        onCloseRequested: async () => (() => {}),
                        destroy: async () => {},
                    };
                },
                getAllWindows: async () => [],
            },
            webview: {
                getCurrentWebview: () => {
                    return {
                        listen: async () => (() => {}),
                    };
                }
            }
        };
        """

        page = context.new_page()
        page.add_init_script(init_script)

        print("Navigating to app...")
        page.goto("http://localhost:1420")

        print("Waiting for app content...")
        try:
            # Wait for text area or any indicator of success
            page.wait_for_selector("textarea", timeout=10000)
            print("App loaded (textarea found).")
        except Exception as e:
            print(f"App load failed: {e}")
            page.screenshot(path="debug_app_load_fail_4.png")
            browser.close()
            return

        # Check for message
        print("Looking for message '你好，这是一个测试'...")
        if page.get_by_text("你好，这是一个测试").count() == 0:
             print("Message not found. Injecting it manually via input.")
             page.fill("textarea", "你好，这是一个测试")
             page.press("textarea", "Enter")
             page.wait_for_timeout(1000)

        print("Locating copy button...")
        # Try to find the button. Since we have aria-label="复制"
        copy_button = page.get_by_label("复制").first

        # If invisible, hover over the message text first
        if not copy_button.is_visible():
             print("Copy button not visible, attempting hover...")
             page.get_by_text("你好，这是一个测试").first.hover()
             page.wait_for_timeout(500)

        print("Clicking copy button...")
        try:
            copy_button.click()
            page.wait_for_timeout(500)

            page.screenshot(path="verification_copy_success.png")
            print("Screenshot saved to verification_copy_success.png")

            clipboard_text = page.evaluate("navigator.clipboard.readText()")
            if clipboard_text:
                print(f"✅ SUCCESS: Text copied: '{clipboard_text}'")
            else:
                print(f"❌ ERROR: Clipboard empty.")
        except Exception as e:
            print(f"Interaction failed: {e}")
            page.screenshot(path="debug_interaction_fail.png")

        browser.close()

if __name__ == "__main__":
    run()
