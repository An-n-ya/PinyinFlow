import { beforeEach, describe, expect, it, vi } from 'vitest';
import { play } from './Chat';

// Mock the invoke function from tauri
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn(),
    listen: vi.fn().mockResolvedValue(() => {}), // mock listen to return a promise resolving to unlisten fn
}));

import { invoke } from '@tauri-apps/api/core';

describe('play function', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    it('should call proofread then play with correct arguments', async () => {
        const id = 'msg-123';
        const input = 'test input';
        const revisedInput = 'revised input';

        // Mock invoke implementations
        (invoke as any).mockImplementation((cmd: string, _args: any) => {
            if (cmd === 'proofread') {
                return Promise.resolve(revisedInput);
            }
            if (cmd === 'play') {
                return Promise.resolve();
            }
            return Promise.reject(new Error(`Unknown command: ${cmd}`));
        });

        await play(id, input);

        expect(invoke).toHaveBeenCalledTimes(2);
        expect(invoke).toHaveBeenNthCalledWith(1, 'proofread', { id, input });
        expect(invoke).toHaveBeenNthCalledWith(2, 'play', { id, input: revisedInput });
    });

    it('should handle errors gracefully', async () => {
        const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
        (invoke as any).mockRejectedValue(new Error('Network error'));

        await play('id', 'input');

        expect(consoleSpy).toHaveBeenCalled();
        consoleSpy.mockRestore();
    });
});
