import { describe, expect, it, vi } from 'vitest';
import { ReviseStrategy } from '../../ai/LLMStrategy/ReviseStrategy';
import { LLMTextInput, OpenAIService } from '../LLM';

// Mock @tauri-apps/api/core
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

describe('OpenAIService.revise', () => {
    it('应当返回模型响应中的修订文本', async () => {
        // Prepare mock return value
        (invoke as any).mockResolvedValue(JSON.stringify({ revisted: '修正后的文本' }));

        const service = new OpenAIService(
            'fake-api-key',
            'LongCat-Flash-Chat',
            'https://api.longcat.chat/openai'
        );

        const input = new LLMTextInput('原始文本');
        const result = await service.revise(input, new ReviseStrategy());

        expect(result.text).toBe('修正后的文本');
        expect(invoke).toHaveBeenCalledWith('revise', {
            apiKey: 'fake-api-key',
            baseUrl: 'https://api.longcat.chat/openai',
            model: 'LongCat-Flash-Chat',
            prompt: '原始文本',
            systemPrompt: expect.any(String),
            jsonSchema: expect.any(Object),
        });
    });
});
