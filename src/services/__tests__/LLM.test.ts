import { describe, it } from 'vitest';

describe('OpenAIService.revise - 集成测试', () => {
    it.skip('集成测试已移动到 Rust 后端 (src-tauri/src/llm.rs)', () => {
        // Since logic moved to Rust backend, frontend integration tests cannot run directly in Node environment without mocking 'invoke'.
        // See src-tauri/src/llm.rs for backend tests.
    });
});
