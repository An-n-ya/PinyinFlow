import { describe, expect, it, vi } from 'vitest';
import { LLMTextInput, OpenAIService } from '../LLM';
import { ReviseStrategy } from '../../ai/LLMStrategy/ReviseStrategy';
// 先 mock 掉 openai SDK，避免真实网络请求
const createMock = vi.fn();

describe('OpenAIService.revise', () => {
    vi.mock('openai', () => {
        class OpenAI {
            chat = {
                completions: {
                    create: createMock,
                },
            };

            constructor(_options: any) {}
        }

        return {
            __esModule: true,
            default: OpenAI,
        };
    });
    it('应当返回模型响应中的修订文本', async () => {
        // 准备 mock 返回值
        createMock.mockResolvedValue({
            choices: [
                {
                    message: {
                        // revise 中会把这个当成字符串传给 get_content
                        content: JSON.stringify({ revisted: '修正后的文本' }),
                    },
                },
            ],
        } as any);

        const service = new OpenAIService(
            'fake-api-key',
            'LongCat-Flash-Chat',
            'https://api.longcat.chat/openai'
        );

        const input = new LLMTextInput('原始文本');
        const result = await service.revise(input, new ReviseStrategy());

        expect(result.text).toBe('修正后的文本');
        expect(createMock).toHaveBeenCalledTimes(1);
    });
});
