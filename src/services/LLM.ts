import { invoke } from '@tauri-apps/api/core';
import { ReviseStrategy } from '../ai/LLMStrategy/ReviseStrategy';
import { BaseService } from './base';

export type OpenAIModel = string;
export type LLMJsonResponse = {
    name: string;
    description?: string;
    schema: Record<string, unknown>;
};

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
    abstract complete(input: LLMTextInput): Promise<LLMTextOutput>;
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

    expand(_input: LLMTextInput): Promise<LLMTextOutput> {
        throw new Error('Method not implemented.');
    }

    complete(_input: LLMTextInput): Promise<LLMTextOutput> {
        throw new Error('Method not implemented.');
    }

    async revise(input: LLMTextInput, strategy: ReviseStrategy): Promise<LLMTextOutput> {
        try {
            const result = await invoke<string>('revise', {
                apiKey: this.openaiApiKey,
                baseUrl: this.base_url,
                model: this.model,
                prompt: input.prompt,
                systemPrompt: strategy.systemPrompt,
                jsonSchema: strategy.jsonSchema,
            });

            if (!result) {
                throw new Error('Content is empty');
            }

            const text = strategy.parseResponse(result);

            const text_response: LLMTextOutput = {
                text,
            };
            return text_response;
        } catch (error) {
            console.error('LLM request failed:', error);
            throw error;
        }
    }
}
