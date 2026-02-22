import { act, render } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import Chat from './Chat';

// Polyfill crypto for MessageType
Object.defineProperty(global, 'crypto', {
    value: {
        randomUUID: () => 'test-uuid-' + Math.random(),
    },
});

// Polyfill ResizeObserver
global.ResizeObserver = class ResizeObserver {
    observe() {}
    unobserve() {}
    disconnect() {}
};

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn(),
    Channel: class {
        onmessage = () => {};
    },
}));

vi.mock('@tauri-apps/api/event', () => ({
    listen: vi.fn(() => Promise.resolve(() => {})),
}));

// Hoisted state for render tracking
const mocks = vi.hoisted(() => ({
    inputAreaRenderCount: { count: 0 },
}));

// Mock InputArea
vi.mock('./InputArea', async () => {
    const { memo } = await import('react');
    return {
        InputArea: memo(() => {
            mocks.inputAreaRenderCount.count++;
            return <div data-testid="input-area">InputArea</div>;
        }),
    };
});

describe('Chat Component Performance', () => {
    it('should not re-render InputArea when messages update', async () => {
        mocks.inputAreaRenderCount.count = 0; // Reset count

        render(<Chat />);

        // Initial render: Chat renders -> InputArea renders.
        expect(mocks.inputAreaRenderCount.count).toBe(1);

        // Capture the listen callback
        const { listen } = await import('@tauri-apps/api/event');
        const listenMock = listen as unknown as { mock: { calls: any[] } };

        // Find the call to listen('audio-played', ...)
        const audioPlayedCall = listenMock.mock.calls.find((c: any) => c[0] === 'audio-played');

        if (!audioPlayedCall) {
            throw new Error('audio-played listener not registered');
        }

        const callback = audioPlayedCall[1];

        // Trigger the callback to update state
        await act(async () => {
            callback({
                payload: {
                    AudioPlayed: { id: '1' }, // ID from TEST_DATA in Chat.tsx
                },
            });
        });

        // After update:
        // Without optimization: InputArea re-renders -> count becomes 2.
        // With optimization: InputArea does NOT re-render -> count remains 1.
        expect(mocks.inputAreaRenderCount.count).toBe(1);
    });
});
