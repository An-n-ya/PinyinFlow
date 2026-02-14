use crate::service::llm::{
    domain::{LlmMetadata, LlmResponse, Message, Role},
    provider::GenConfig,
    strategy::{TaskContext, TaskStrategy},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub struct ProofreadContext {
    pub text: String,
}
impl TaskContext for ProofreadContext {}
#[derive(Default)]
pub struct ProofreadBuilder {
    custom_instruction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofreadResponse {
    pub corrected_text: String,
    pub error_list: Vec<String>,
    #[serde(skip)]
    pub meta: Option<LlmMetadata>,
}

impl TaskStrategy for ProofreadBuilder {
    type Input = ProofreadContext;
    type Output = String;

    fn system_prompt(&self) -> String {
        self.custom_instruction.clone().unwrap_or_else(|| {
            r#"你是一个专业的校对工具，擅长纠正错别字。你的回答只包含修正后的文本，不要有描述性文字，不要有任何多余的文字。

【输入格式】
用户输入就是需要校对的全部文字，如果没有错误就原样返回。

【输出格式要求】
输出格式就是校对后的文字

【参考示例】
用户打字的时候有很多种错误，比如说有多次键入的错误，也有拼写错误，语法错误，还有打字速度太快导致的错误。比如：

(选字错误示例)
Q: 一部小心选到了错误的方向
A: 一不小心选到了错误的方向

（漏键错误：liao -> lao）
Q: 你真是老师入神
A: 你真是料事如神

（串键，按错成键盘上的相邻键： pinyin -> pinyun）
Q: 我喜欢用拼运打字
A: 我喜欢用拼音打字

请严格按照上述格式和规则提取信息并输出。"#.to_string()
        })
    }

    fn config(&self) -> GenConfig {
        GenConfig {
            max_tokens: None,
            temperature: 0.2,
            top_p: 0.8,
            stream: false,
        }
    }

    fn user_messages(&self, input: &Self::Input) -> Vec<Message> {
        vec![Message::new(Role::User, input.text.clone())]
    }
    fn parse_response(&self, raw: LlmResponse) -> Result<Self::Output> {
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
        strategy::proofread::{ProofreadBuilder, ProofreadContext},
    };

    #[tokio::test]
    async fn test_proofread() {
        let service = LlmService::service_for_test();
        let input = ProofreadContext {
            text: "Rust预言真好用！".into(),
        };
        let res = service
            .execute_task(TaskType::Proofread, ProofreadBuilder::default(), input)
            .await
            .unwrap();
        println!("{:?}", res);
    }
}
