use anyhow::Result;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage},
    Credentials,
};

use crate::service::llm::domain::{Message, RawLlmResponse};

#[derive(Debug, Clone)]
pub struct GenConfig {
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: Option<u32>,
}

impl Default for GenConfig {
    fn default() -> Self {
        Self {
            temperature: 1.0,
            top_p: 1.0,
            max_tokens: None,
        }
    }
}

pub trait LlmProvider: Send + Sync {
    async fn generate(&self, messages: &[Message], config: &GenConfig) -> Result<RawLlmResponse>;
}

#[derive(Debug, Clone)]
pub struct OpenAiProvider {
    api_key: String,
    model: String,
    entrypoint: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: String, entrypoint: String) -> Self {
        Self {
            api_key,
            model,
            entrypoint,
        }
    }
}

impl LlmProvider for OpenAiProvider {
    async fn generate(&self, messages: &[Message], config: &GenConfig) -> Result<RawLlmResponse> {
        let messages: Vec<ChatCompletionMessage> =
            messages.to_vec().into_iter().map(|v| v.into()).collect();
        let credential = Credentials::new(self.api_key.clone(), self.entrypoint.clone());
        // TODO: add retry mechanism
        let chat_completion = ChatCompletion::builder(&self.model, messages)
            .credentials(credential)
            .temperature(config.temperature)
            .top_p(config.top_p)
            .create()
            .await
            .unwrap();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();
        Ok(RawLlmResponse::from(returned_message))
    }
}

pub struct LocalProvider {
    endpoint: String,
}

impl LlmProvider for LocalProvider {
    async fn generate(&self, messages: &[Message], _config: &GenConfig) -> Result<RawLlmResponse> {
        unimplemented!("LocalProvider::generate")
    }
}

pub enum LlmBackend {
    OpenAi(OpenAiProvider),
    Local(LocalProvider),
}

impl LlmBackend {
    pub async fn generate(
        &self,
        messages: &[Message],
        config: &GenConfig,
    ) -> Result<RawLlmResponse> {
        match self {
            LlmBackend::OpenAi(provider) => provider.generate(messages, config).await,
            LlmBackend::Local(provider) => provider.generate(messages, config).await,
        }
    }
}
