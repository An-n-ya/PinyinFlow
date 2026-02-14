use crate::service::llm::{
    domain::{LlmResponse, Message, Role},
    provider::GenConfig,
};
use anyhow::Result;

pub(crate) mod complete;
pub(crate) mod proofread;
pub(crate) mod reply;

pub trait TaskContext: Send + Sync {}
pub trait TaskStrategy: Send + Sync {
    type Input: TaskContext;
    type Output;

    fn system_prompt(&self) -> String;

    fn build_messages(&self, input: &Self::Input) -> Vec<Message> {
        let mut messages = vec![Message::new(Role::System, self.system_prompt())];
        messages.extend(self.user_messages(input));
        messages
    }

    fn config(&self) -> GenConfig {
        GenConfig::default()
    }

    fn user_messages(&self, input: &Self::Input) -> Vec<Message>;

    fn parse_response(&self, raw: LlmResponse) -> Result<Self::Output>;
}
