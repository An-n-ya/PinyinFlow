use serde::{Deserialize, Serialize};

use crate::service::llm::{
    domain::{LlmMetadata, LlmResponse, Message, Role},
    strategy::{TaskContext, TaskStrategy},
};

enum MessageRole {
    Me,
    Opposite(String),
}

struct HistoryMessage {
    role: MessageRole,
    content: String,
}

pub struct ReplyContext {
    pub history: Vec<HistoryMessage>,
    pub outline: String,
}
impl TaskContext for ReplyContext {}
impl ReplyContext {
    fn context(&self) -> String {
        let mut history = vec!["【当前上下文】".to_string()];
        for message in &self.history {
            match &message.role {
                MessageRole::Me => history.push(format!("我：{}", message.content)),
                MessageRole::Opposite(name) => {
                    history.push(format!("{}：{}", name, message.content))
                }
            }
        }
        history.join("\n")
    }
}

#[derive(Debug, Default)]
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
    type Output = String;

    fn system_prompt(&self) -> String {
        r#"你是一个“沟通代理人”。你正在代表一位使用 TTS 辅助工具的用户进行发言。
你的目标是让用户的沟通尽可能顺畅、自然且有尊严。

【你的性格与行为指南】
1. **完全的代理人**：你就是用户。直接输出用户想说的话。不要解释你的思考过程。
2. **适应上下文**：
   - 如果对方在开玩笑，你的回复应该轻松幽默。
   - 如果是在医院或银行，你的回复应该清晰、准确、严肃。
3. **处理不完整输入**：
   - 如果用户输入 "厕所 哪"，你应该输出 "请问洗手间在哪里？"
   - 如果用户输入 "不同意"，你应该根据上下文委婉或直接地表达反对。
4. **输出要求**：
   - 避免使用复杂的从句，多用短句。
   - 标点符号要准确，以便 TTS 引擎正确处理停顿（多用逗号和句号）。
   - **绝对不要**包含 Emoji 表情。
   - **绝对不要**包含 Markdown 格式（如 **粗体**），纯文本即可。

【参考示例】
用户：厕所 哪
输出：请问洗手间在哪里？
   "#
        .to_string()
    }

    fn user_messages(&self, input: &Self::Input) -> Vec<Message> {
        println!("context: {}", input.context());
        vec![
            Message::new(Role::User, format!("{}", input.outline)),
            Message::new(Role::System, input.context()),
        ]
    }

    fn config(&self) -> crate::service::llm::provider::GenConfig {
        crate::service::llm::provider::GenConfig {
            max_tokens: None,
            temperature: 0.4,
            top_p: 0.7,
            stream: false,
        }
    }

    fn parse_response(&self, raw: LlmResponse) -> anyhow::Result<Self::Output> {
        if let LlmResponse::Raw(raw) = raw {
            Ok(raw.content)
        } else {
            anyhow::bail!("Unexpected response type")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::llm::{
        domain::TaskType,
        service::LlmService,
        strategy::reply::{HistoryMessage, MessageRole, ReplyBuilder, ReplyContext},
    };

    #[tokio::test]
    async fn test_reply() {
        let service = LlmService::service_for_test();
        let input = ReplyContext {
            history: vec![
                HistoryMessage {
                    content: "你好，有什么可以帮助你的么？".into(),
                    role: MessageRole::Opposite("咖啡店员".into()),
                },
                HistoryMessage {
                    content: "一杯生椰拿铁，不加糖".into(),
                    role: MessageRole::Me,
                },
                HistoryMessage {
                    content: "打包么？".into(),
                    role: MessageRole::Opposite("咖啡店员".into()),
                },
            ],
            outline: "不".into(),
        };
        let res = service
            .execute_task(TaskType::Reply, ReplyBuilder::default(), input)
            .await
            .unwrap();
        println!("{:?}", res);
    }
}
