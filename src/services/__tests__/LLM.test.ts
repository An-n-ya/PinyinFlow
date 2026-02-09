import { describe, expect, it, vi } from 'vitest';
import { LLMTextInput, OpenAIService, ReviseStrategy } from '../LLM';

describe('OpenAIService.revise - 集成测试', () => {
  vi.unmock('openai');
  it('连真实服务器并返回修订后的文本', async () => {
    //const apiKey = process.env.LONGCAT_API_KEY;
    const apiKey = import.meta.env.VITE_LONGCAT_API_KEY;
    if (!apiKey) {
      console.warn('跳过集成测试：未设置 LONGCAT_API_KEY');
      return;
    }

    const service = new OpenAIService(
      apiKey,
      'LongCat-Flash-Chat',
      'https://api.longcat.chat/openai'
    );

    const original = '这是一段有错别字的中文內容';
    const expect_text = '这是一段有错别字的中文内容';
    const input = new LLMTextInput(original);

    const result = await service.revise(input, new ReviseStrategy());

    // 基本断言：返回非空、能解析
    expect(result.text).toBeTypeOf('string');
    expect(result.text.length).toBeGreaterThan(0);

    // 你可以加一些更强的断言，比如不完全等于原始文本
    expect(result.text).toBe(expect_text);
  }, 20000); // 超时时间 20s，避免网络慢时测试直接挂掉
});
