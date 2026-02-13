use serde::{Deserialize, Serialize};

use crate::service::llm::{
    domain::{LlmMetadata, Message, Role},
    strategy::{TaskContext, TaskStrategy},
};

pub struct ReplyContext {
    pub history: Vec<Message>,
    pub outline: String,
}
impl TaskContext for ReplyContext {}

pub struct ReplyBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyResponse {
    pub text: String,
    // 同时也包含元数据，方便业务层统计
    #[serde(skip)]
    pub meta: Option<LlmMetadata>,
}

impl TaskStrategy for ReplyBuilder {
    type Input = ReplyContext;
    type Output = ReplyResponse;

    fn system_prompt(&self) -> String {
        "你是一个乐于助人的助手。请根据用户提供的提纲和之前的对话历史，生成合适的回复。".to_string()
    }

    fn user_messages(&self, input: &Self::Input) -> Vec<Message> {
        let mut msgs = input.history.clone(); // 包含上下文
        msgs.push(Message::new(
            Role::User,
            format!("请根据提纲回复：{}", input.outline),
        ));
        msgs
    }

    fn parse_response(
        &self,
        raw: crate::service::llm::domain::RawLlmResponse,
    ) -> anyhow::Result<Self::Output> {
        todo!()
    }
}
