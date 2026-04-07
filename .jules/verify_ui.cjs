import { expect, test } from '@playwright/test';
import { Page, sync_playwright } from 'playwright';

function verify_ui(page: Page) {
    page.goto('http://localhost:1420/chat');

    // Check if the chat page close button has the aria-label
    const chatCloseButton = page.locator('header button[aria-label="关闭"]');
    expect(chatCloseButton).toBeVisible();

    page.screenshot({ path: '/home/jules/verification/chat-close-button.png' });

    page.goto('http://localhost:1420/settings');

    // Check if the settings page close button has the aria-label
    const settingsCloseButton = page.locator('header button[aria-label="关闭"]');
    expect(settingsCloseButton).toBeVisible();

    page.screenshot({ path: '/home/jules/verification/settings-close-button.png' });
}

if (require.main === module) {
    const { sync_playwright } = require('playwright');
    with sync_playwright() as p:
        const browser = p.chromium.launch({ headless: true });
        const page = browser.newPage();
        try {
            verify_ui(page);
        } finally {
            browser.close();
        }
}
