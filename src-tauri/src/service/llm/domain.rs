use openai::chat::{
    ChatCompletionChoiceDelta, ChatCompletionGeneric, ChatCompletionMessage,
    ChatCompletionMessageRole,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum TaskType {
    Proofread, // 校对
    Continue,  // 续写
    Reply,     // 回复
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMetadata {
    pub usage: Option<TokenUsage>,
    pub latency: usize,
    pub model_id: String,
    pub finish_reason: Option<String>,
}

type ChatComletionDelta = ChatCompletionGeneric<ChatCompletionChoiceDelta>;

#[derive(Debug)]
pub enum LlmResponse {
    Raw(RawLlmResponse),
    Stream(tokio::sync::mpsc::Receiver<ChatComletionDelta>),
}

#[derive(Debug, Clone)]
pub struct RawLlmResponse {
    pub content: String, // string or JSON
    pub meta: Option<LlmMetadata>,
}

pub trait LlmRes {
    fn content(&self) -> String;
}
impl LlmRes for ChatCompletionMessage {
    fn content(&self) -> String {
        self.content.clone().unwrap_or("".to_string())
    }
}

// Adapter
impl Into<ChatCompletionMessageRole> for Role {
    fn into(self) -> ChatCompletionMessageRole {
        match self {
            Role::System => ChatCompletionMessageRole::System,
            Role::User => ChatCompletionMessageRole::User,
            Role::Assistant => ChatCompletionMessageRole::Assistant,
        }
    }
}
impl Into<ChatCompletionMessage> for Message {
    fn into(self) -> ChatCompletionMessage {
        ChatCompletionMessage {
            role: self.role.into(),
            content: Some(self.content),
            name: None,
            function_call: None,
            tool_call_id: None,
            tool_calls: None,
        }
    }
}

impl From<ChatCompletionMessage> for RawLlmResponse {
    fn from(value: ChatCompletionMessage) -> Self {
        Self {
            content: value.content.unwrap_or("".to_string()),
            meta: None,
        }
    }
}
