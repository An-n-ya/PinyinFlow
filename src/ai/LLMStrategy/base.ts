import { LLMJsonResponse } from '../../services/LLM';

export interface LLMTaskStrategy {
    systemPrompt: string;
    jsonSchema: LLMJsonResponse;
    parseResponse(response: string): string;
}
