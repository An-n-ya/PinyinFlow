import { describe, it, expect, vi } from "vitest";
import { OpenAIService, ReviseInput } from "./LLM";


describe("OpenAIService.revise - 集成测试", () => {
    vi.unmock("openai");
    it(
      "连真实服务器并返回修订后的文本",
      async () => {
        //const apiKey = process.env.LONGCAT_API_KEY;
        const apiKey = import.meta.env.VITE_LONGCAT_API_KEY;
        if (!apiKey) {
          // 没有配置 key 时直接跳过，不要让测试失败
          console.warn("跳过集成测试：未设置 LONGCAT_API_KEY");
          return;
        }
  
        // 不再 mock openai，这里用真正的 OpenAIService
        const service = new OpenAIService(
          apiKey,
          "LongCat-Flash-Chat",          // 或你想测的模型
          "https://api.longcat.chat/openai", // 可以不传，使用默认
        );
  
        const original = "这是一段有错别字的中文內容";
        const expect_text =   "这是一段有错别字的中文内容";
        const input = new ReviseInput(original);
  
        const result = await service.revise(input);
  
        // 基本断言：返回非空、能解析
        expect(result.text).toBeTypeOf("string");
        expect(result.text.length).toBeGreaterThan(0);
  
        // 你可以加一些更强的断言，比如不完全等于原始文本
        expect(result.text).toBe(expect_text)
      },
      20000, // 超时时间 20s，避免网络慢时测试直接挂掉
    );
  });

//// 先 mock 掉 openai SDK，避免真实网络请求
//const createMock = vi.fn();

// describe("OpenAIService.revise", () => {
//     vi.mock("openai", () => {
//     class OpenAI {
//         chat = {
//         completions: {
//             create: createMock,
//         },
//         };

//         constructor(_options: any) {
//         }
//     }

//     return {
//         __esModule: true,
//         default: OpenAI,
//     };
//     });
//   it("应当返回模型响应中的修订文本", async () => {
//     // 准备 mock 返回值
//     createMock.mockResolvedValue({
//       choices: [
//         {
//           message: {
//             // revise 中会把这个当成字符串传给 get_content
//             content: JSON.stringify({ revisted: "修正后的文本" }),
//           },
//         },
//       ],
//     } as any);

//     const service = new OpenAIService(
//       "fake-api-key",
//       "LongCat-Flash-Chat",
//       "https://api.longcat.chat/openai",
//     );

//     const input = new ReviseInput("原始文本");
//     const result = await service.revise(input);

//     expect(result.text).toBe("修正后的文本");
//     expect(createMock).toHaveBeenCalledTimes(1);
//   });
// });
