import { describe, expect, it } from 'vitest';
import { ReviseStrategy } from '../../ai/LLMStrategy/ReviseStrategy';
import { LLMTextInput, OpenAIService } from '../LLM';

const MODELS = [
    {
        apiKey: import.meta.env.VITE_LONGCAT_API_KEY, //~780ms
        model: 'LongCat-Flash-Chat',
        url: 'https://api.longcat.chat/openai/v1/',
    },
    {
        apiKey: import.meta.env.VITE_SILICONFLOW_API_KEY, //~1800ms
        model: 'Qwen/Qwen3-8B',
        url: 'https://api.siliconflow.cn/v1/',
    },
    {
        apiKey: 'local', //~250ms
        model: 'Qwen/Qwen3-1.7B-GGUF',
        url: 'http://127.0.0.1:8033/v1/',
    },
];

describe('OpenAIService.revise - 集成测试', () => {
    it('连真实服务器并返回修订后的文本', async () => {
        const model = MODELS[1];

        const apiKey = model.apiKey;
        if (!apiKey) {
            console.warn('跳过集成测试：未设置 VITE_SILICONFLOW_API_KEY');
            return;
        }

        const service = new OpenAIService(apiKey, model.model, model.url);

        const original = '一部小心选到了错误的方向';
        const expect_text = '一不小心选到了错误的方向';
        const input = new LLMTextInput(original);

        const result = await service.revise(input, new ReviseStrategy());

        // 基本断言：返回非空、能解析
        expect(result.text).toBeTypeOf('string');
        expect(result.text.length).toBeGreaterThan(0);

        // 你可以加一些更强的断言，比如不完全等于原始文本
        expect(result.text).toBe(expect_text);
    }, 20000); // 超时时间 20s，避免网络慢时测试直接挂掉
});
