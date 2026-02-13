use crate::service::llm::{domain::Message, strategy::TaskContext};

pub struct CompleteContext {
    pub history: Vec<Message>,
    pub current_input: String,
}
impl TaskContext for CompleteContext {}
