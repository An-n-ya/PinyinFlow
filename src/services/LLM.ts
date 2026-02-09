import OpenAI from 'openai';
import {
    ChatCompletion,
    ChatCompletionContentPart,
    ChatCompletionMessageParam,
} from 'openai/resources';
import { BaseService } from './base';
export type OpenAIModel = 'LongCat-Flash-Lite' | 'LongCat-Flash-Chat';
export type LLMJsonResponse = {
    name: string;
    description?: string;
    schema: Record<string, unknown>;
};
export interface LLMTaskStrategy {
    systemPrompt: string;
    jsonSchema: LLMJsonResponse;
    parseResponse(response: string): string;
}

export class ReviseStrategy implements LLMTaskStrategy {
    systemPrompt = '你是一个专业的校对工具，擅长纠正错别字';
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
export class LLMTextInput {
    prompt: string;
    constructor(prompt: string) {
        this.prompt = prompt;
    }
}
export type LLMMetadata = {
    completion_token_cost: number;
    prompt_token_cost: number;
    response_time: number;
};
export type LLMTextOutput = {
    text: string;
    metadata?: LLMMetadata;
};

export abstract class BaseLLMService extends BaseService {
    abstract revise(input: LLMTextInput, strategy: ReviseStrategy): Promise<LLMTextOutput>;
    abstract expand(input: LLMTextInput): Promise<LLMTextOutput>;
}

export class OpenAIService extends BaseLLMService {
    private openaiApiKey: string;
    private model: OpenAIModel;
    private base_url: string;

    constructor(apiKey: string, model: string | null, url: string | null) {
        super();
        this.openaiApiKey = apiKey;
        this.model = (model as OpenAIModel) ?? 'LongCat-Flash-Chat';
        this.base_url = url ?? 'https://api.longcat.chat/openai';
    }

    expand(input: LLMTextInput): Promise<LLMTextOutput> {
        throw new Error('Method not implemented.');
    }

    async revise(input: LLMTextInput, strategy: ReviseStrategy): Promise<LLMTextOutput> {
        const client = new OpenAI({
            apiKey: this.openaiApiKey.trim(),
            baseURL: this.base_url,
            dangerouslyAllowBrowser: true,
        });
        const messages: ChatCompletionMessageParam[] = [];
        messages.push({ role: 'system', content: strategy.systemPrompt });

        const userParts: ChatCompletionContentPart[] = [];
        userParts.push({ type: 'text', text: input.prompt });
        messages.push({ role: 'user', content: userParts });

        const response: ChatCompletion = await client.chat.completions.create({
            messages,
            model: this.model,
            temperature: 1,
            max_completion_tokens: 512,
            top_p: 1,
            response_format: {
                type: 'json_schema',
                json_schema: strategy.jsonSchema,
            },
        });
        if (!response.choices || response.choices.length == 0) {
            throw new Error(`No response from ${this.model}`);
        }

        const result = response.choices[0].message.content;
        console.log(result);
        if (!result) {
            throw new Error('Content is empty');
        }

        const text = strategy.parseResponse(result);

        const text_response: LLMTextOutput = {
            text,
        };
        return text_response;
    }
}
