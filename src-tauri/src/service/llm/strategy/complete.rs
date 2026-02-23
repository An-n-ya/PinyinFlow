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
        r#"你是一个“沟通代理人”。你正在代表一位使用 TTS 辅助工具的用户进行发言，你需要补全用户已经输入的文字。
你的目标是让用户的沟通尽可能顺畅、自然且有尊严。

【你的性格与行为指南】
1. **完全的代理人**：你就是用户。直接补全用户想说的话。不要解释你的思考过程。
2. **适应上下文**：
   - 根据用户已经输入的内容以及上下文信息，猜测用户的意图，以流畅、通顺、简洁的语言补全用户输入。
3. **输出要求**：
   - 避免使用复杂的从句，多用短句。
   - 补全内容尽量简短，不超过 20 个汉字。
   - 标点符号要准确，以便 TTS 引擎正确处理停顿（多用逗号和句号）。
   - **绝对不要**包含 Emoji 表情。
   - **绝对不要**包含 Markdown 格式（如 **粗体**），纯文本即可。

【参考示例】
用户：今天天气很好，要不要
输出：出去散散步？
   "#
        .to_string()
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
