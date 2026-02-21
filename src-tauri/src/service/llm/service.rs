use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::service::llm::domain::TaskType;
use crate::service::llm::provider::{LlmBackend, OpenAiProvider};
use crate::service::llm::strategy::TaskStrategy;

const LLM_PROVIDER: [(&str, &str, &str); 3] = [
    (
        "VITE_LONGCAT_API_KEY",
        "LongCat-Flash-Chat",
        "https://api.longcat.chat/openai",
    ),
    (
        "VITE_SILICONFLOW_API_KEY",
        "Qwen/Qwen3-8B",
        "https://api.siliconflow.cn/v1/",
    ),
    (
        "LOCAL_KEY",
        "Qwen/Qwen3-1.7B-GGUF",
        "http://127.0.0.1:8033/v1/",
    ),
];

pub struct LlmService {
    providers: HashMap<TaskType, LlmBackend>,
    default_provider: LlmBackend,
}

impl LlmService {
    pub fn init() -> Self {
        // TODO: load api_key from Sqlite database
        let openai_providers: Vec<_> = LLM_PROVIDER
            .iter()
            .map(|(api, model, url)| {
                OpenAiProvider::new(env::var(api).unwrap(), model.to_string(), url.to_string())
            })
            .collect();
        LlmService {
            providers: HashMap::new(),
            default_provider: LlmBackend::OpenAi(openai_providers[0].clone()),
        }
    }

    pub(crate) fn service_for_test() -> Self {
        dotenvy::from_path(Path::new("../.env.test.local")).unwrap();
        let openai_providers: Vec<_> = LLM_PROVIDER
            .iter()
            .map(|(api, model, url)| {
                OpenAiProvider::new(env::var(api).unwrap(), model.to_string(), url.to_string())
            })
            .collect();
        LlmService {
            providers: HashMap::new(),
            default_provider: LlmBackend::OpenAi(openai_providers[1].clone()),
        }
    }

    pub fn register_provider(&mut self, task: TaskType, provider: LlmBackend) {
        self.providers.insert(task, provider);
    }

    fn get_provider(&self, task: TaskType) -> &LlmBackend {
        self.providers.get(&task).unwrap_or(&self.default_provider)
    }

    pub async fn execute_task<S>(
        &self,
        task_type: TaskType,
        strategy: S,
        input: S::Input,
    ) -> Result<S::Output>
    where
        S: TaskStrategy,
    {
        let provider = self.get_provider(task_type);
        let messages = strategy.build_messages(&input);
        let raw_responsee = provider
            .generate(&messages, &strategy.config())
            .await
            .unwrap();

        strategy.parse_response(raw_responsee)
    }
}
