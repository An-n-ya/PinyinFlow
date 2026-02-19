from playwright.sync_api import sync_playwright

def run():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        page.goto("http://localhost:1420")

        # Wait for the message bubble
        page.wait_for_selector(".flex.h-screen.flex-col")

        # Find the visualizer using role and label
        try:
            visualizer = page.get_by_role("img", name="正在播放音频")
            if visualizer.is_visible():
                print("SUCCESS: Visualizer found and visible.")
                # Verify aria-label
                label = visualizer.get_attribute("aria-label")
                print(f"Aria-label: {label}")
                if label == "正在播放音频":
                    print("SUCCESS: Aria-label is correct.")
                else:
                    print(f"FAILURE: Aria-label is incorrect: {label}")
            else:
                print("FAILURE: Visualizer found but not visible.")
        except Exception as e:
            print(f"FAILURE: Visualizer not found: {e}")

        # Take screenshot
        page.screenshot(path="verification_screenshot.png")
        browser.close()

if __name__ == "__main__":
    run()
