use tauri::ipc::Channel;

use crate::{
    commands::ReplyCompleteEvent,
    service::llm::{
        domain::{LlmResponse, Message, Role},
        provider::GenConfig,
        strategy::{TaskContext, TaskStrategy},
    },
};

pub struct CompleteContext {
    pub history: Vec<Message>,
    pub current_input: String,
}
impl CompleteContext {
    pub fn new(current_input: String) -> Self {
        // TODO: load history from db
        Self {
            history: vec![],
            current_input,
        }
    }
}
impl TaskContext for CompleteContext {}

pub struct CompleteBuilder {
    channel: Channel<ReplyCompleteEvent>,
}

impl CompleteBuilder {
    pub fn new(channel: Channel<ReplyCompleteEvent>) -> Self {
        Self { channel }
    }
}
impl TaskStrategy for CompleteBuilder {
    type Input = CompleteContext;

    type Output = ();

    fn system_prompt(&self) -> String {
        "".to_string()
    }

    fn config(&self) -> GenConfig {
        GenConfig {
            max_tokens: None,
            temperature: 0.8,
            top_p: 0.8,
            stream: true,
        }
    }

    fn user_messages(&self, input: &Self::Input) -> Vec<Message> {
        vec![Message::new(Role::User, format!("{}", input.current_input))]
    }

    fn parse_response(&self, raw: LlmResponse) -> anyhow::Result<Self::Output> {
        if let LlmResponse::Stream(mut receiver) = raw {
            let channel = self.channel.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(msg) = receiver.recv().await {
                        println!("complete receive message: {msg:?}");
                        if let Some(reason) = &msg.choices[0].finish_reason {
                            if reason == "stop" {
                                channel
                                    .send(ReplyCompleteEvent::Finished)
                                    .expect("Failed to send finished message");
                                break;
                            }
                        }
                        channel
                            .send(ReplyCompleteEvent::Content(
                                msg.choices[0].delta.content.clone().unwrap(),
                            ))
                            .expect("Failed to send stream message");
                    }
                }
            });
            Ok(())
        } else {
            anyhow::bail!("Unexpected response type")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::sync::{mpsc, oneshot};

    use crate::service::llm::domain::TaskType;
    use crate::service::llm::{
        service::LlmService,
        strategy::complete::{CompleteBuilder, CompleteContext},
    };

    #[tokio::test]
    async fn test_complete() {
        let service = LlmService::service_for_test();
        let input = CompleteContext::new("你是谁".to_string());

        let channel = tauri::ipc::Channel::new(move |e| {
            println!("{:?}", e);
            Ok(())
        });
        let res = service
            .execute_task(TaskType::Continue, CompleteBuilder::new(channel), input)
            .await
            .unwrap();

        // TODO: adopt to a better way to wait for the completion
        tokio::time::sleep(Duration::from_secs(10)).await;
        println!("{:?}", res);
    }
}
