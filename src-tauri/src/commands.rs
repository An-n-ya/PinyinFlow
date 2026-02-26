use anyhow::Result;
use anyhow_tauri::TAResult;
use paste::paste;
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, State};
use tokio::sync::Mutex;

use crate::{
    database::DataBase,
    domain::{preferences::UserPreferences, user_profiles::UserProfiles},
    service::{
        llm::{
            domain::TaskType,
            service::LlmService,
            strategy::{
                complete::{CompleteBuilder, CompleteContext},
                proofread::{ProofreadBuilder, ProofreadContext},
            },
        },
        tts::service::{TTSPlayRequest, TTSService},
    },
};

// 定义 CRUD 命令宏
macro_rules! define_crud_commands {
    ($struct_type:ty, $name:ident) => {
        paste::paste! {
            #[tauri::command]
            pub async fn [<update_ $name>](
                state: State<'_, DataBase>,
                item: $struct_type,
            ) -> TAResult<()> {
                log::info!(concat!("update_", stringify!($name), " {:?}"), item);
                state.[<update_ $name>](&item).await?;
                Ok(())
            }

            #[tauri::command]
            pub async fn [<fetch_ $name>](
                state: State<'_, DataBase>,
                user_id: String,
            ) -> TAResult<Option<$struct_type>> {
                let ret = state.[<fetch_ $name>](&user_id).await?;
                Ok(ret)
            }
        }
    };
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlayRequest {
    pub data: Vec<u8>,
    pub id: String,
}
#[derive(Serialize, Debug)]
pub struct TonePlayRequest {
    input: String,
    id: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PinyinRespond {
    pinyin: String,
    py_styled: String,
    tone: String,
}
#[tauri::command]
pub fn split(input: &str) -> String {
    log::info!("split {input}");
    dollop::split(input)
}
#[tauri::command]
pub async fn tone(input: &str) -> Result<PinyinRespond, String> {
    log::info!("tone {input}");
    let client = reqwest::Client::new();

    let req_body = TonePlayRequest {
        input: input.to_string(),
        id: "1".to_string(),
    };

    let res = client
        .post("http://localhost:8000/tone")
        .json(&req_body)
        .send()
        .await
        .expect("result")
        .text()
        .await
        .unwrap();

    log::info!("tone {res}");
    let v: PinyinRespond = serde_json::from_str(&res).unwrap();

    Ok(v)
}

#[tauri::command]
pub async fn play(state: State<'_, Mutex<TTSService>>, id: String, input: String) -> TAResult<()> {
    state.lock().await.play(TTSPlayRequest { id, input })?;
    Ok(())
}

#[tauri::command]
pub async fn proofread(
    state: State<'_, Mutex<LlmService>>,
    _id: String,
    input: String,
) -> TAResult<String> {
    let input_ = ProofreadContext {
        text: input.clone(),
    };
    let service = state.lock().await;
    let res: String = service
        .execute_task(TaskType::Proofread, ProofreadBuilder::default(), input_)
        .await
        .unwrap();
    log::info!("proofread: origin: {input}, revised: {:?}", res);
    Ok(res)
}

#[derive(Serialize, Debug)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum ReplyCompleteEvent {
    Finished,
    Content(String),
}

#[tauri::command]
pub async fn complete_message(
    state: State<'_, Mutex<LlmService>>,
    input: String,
    on_event: Channel<ReplyCompleteEvent>,
) -> TAResult<()> {
    let input_ = CompleteContext::new(input.clone());
    let service = state.lock().await;
    service
        .execute_task(TaskType::Continue, CompleteBuilder::new(on_event), input_)
        .await
        .unwrap();
    log::info!("complete_message: {input}");
    Ok(())
}

// 使用宏生成 CRUD 命令
define_crud_commands!(UserProfiles, user_profiles);
define_crud_commands!(UserPreferences, user_preferences);
