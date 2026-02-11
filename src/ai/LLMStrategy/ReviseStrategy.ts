import { LLMTaskStrategy } from "./base";

export class ReviseStrategy implements LLMTaskStrategy {
    systemPrompt = `
你是一个专业的校对工具，擅长纠正错别字。你的回答只包含修正后的文本，不要有描述性文字，不要有任何多余的文字。
用户的输入是需要校对的全部文字，校对完成后按照指定的JSON Schema格式输出：

【输出格式要求】
输出格式必须是JSON，如下所示：
{"revised": "字符串类型，必需字段，修正后的文本"}

【输入】
用户的输入即是所需要校对的文本，如果没有错误就原样返回。

【参考示例】
用户打字的时候有很多种错误，比如说有多次键入的错误，也有拼写错误，语法错误，还有打字速度太快导致的错误。比如：
Q: 一部小心选到了错误的方向（选字错误）
A: {"revised": "一不小心选到了错误的方向"}

Q: 你真是老师入神（漏键错误：liao -> lao）
A: {"revised": "你真是料事如神"}

Q: 我喜欢用拼运打字（串键，按错成键盘上的相邻键： pinyin -> pinyun）
A: {"revised": "我喜欢用拼音打字"}

请严格按照上述格式和规则提取信息并输出JSON。
`;
    jsonSchema = {
        name: 'revision_tool',
        schema: {
            type: 'object',
            properties: {
                revisted: { type: 'string' },
            },
            required: ['revisted'],
            additionalProperties: false,
        },
    };
    parseResponse(response: string) {
        try {
            const obj = JSON.parse(response);
            if (typeof obj.revisted === 'string') {
                return obj.revisted;
            }
            throw new Error('Missing "revisted" field in response');
        } catch (e) {
            console.error('Failed to parse revise response:', e);
            throw e;
        }
    }
}
