use crate::config::app_config;
use anyhow::{Context, Result, anyhow};
use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct OllamaChat {
    http: Client,
    base_url: String,
    model: String,
    system_prompt: String,
}

/// Ollama チャットクライアントの実装
/// # メソッド
/// * `new` - 新しいチャットクライアントを作成する
/// * `chat_once` - 履歴なしでチャットを行う
impl OllamaChat {
    /// 新しい OllamaChat クライアントを作成する関数
    /// # 戻り値
    /// * `OllamaChat` - 新しいチャットクライアント
    pub fn new() -> Self {
        let base_url = std::fs::read_to_string(&app_config().default_ollama_base_url)
            .unwrap_or_else(|_| "http://127.0.0.1:11434".to_string());
        let model = std::fs::read_to_string(&app_config().default_ollama_model)
            .unwrap_or_else(|_| "hf.co/LiquidAI/LFM2.5-1.2B-Instruct-GGUF".to_string());
        let system_prompt = std::fs::read_to_string(&app_config().default_system_prompt_path)
            .unwrap_or_else(|_| "You are a helpful assistant.".to_string());

        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            http: http_client,
            base_url,
            model,
            system_prompt,
        }
    }

    /// 履歴なしでチャットを行う関数
    /// # 引数
    /// * `user_input` - ユーザーからの入力メッセージ
    /// # 戻り値
    /// * `Ok(String)` - チャットの応答メッセージ
    /// * `Err(Error)` - エラーが発生した場合
    pub async fn chat_once(&self, user_input: &str) -> Result<String> {
        let messages = vec![
            Message {
                role: "system".to_string(),
                content: self.system_prompt.clone(),
            },
            Message {
                role: "user".to_string(),
                content: user_input.to_string(),
            },
        ];

        self.chat(messages).await
    }

    /// チャットを行う関数
    /// # 引数
    /// * `messages` - チャットのメッセージ履歴
    /// # 戻り値
    /// * `Ok(String)` - チャットの応答メッセージ
    /// * `Err(Error)` - エラーが発生した場合
    async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        // Ollama: POST /api/chat { model, messages, stream }
        let req = ChatRequest {
            model: self.model.clone(),
            messages,
            stream: false, // 非ストリーミングにする
        };

        let url = format!("{}/api/chat", self.base_url);

        debug!(
            "ollama request: url={} model={} stream={}",
            url, self.model, req.stream
        );

        let resp = self
            .http
            .post(url)
            .json(&req) // reqwestのJSON送信
            .send()
            .await
            .context("failed to send request to ollama")?;

        let status = resp.status();
        let body = resp
            .text()
            .await
            .context("failed to read ollama response body")?;

        if !status.is_success() {
            return Err(anyhow!(
                "ollama returned non-2xx: status={} body={}",
                status,
                body
            ));
        }

        // 1) 通常のストリーミングでないレスポンス
        if let Ok(parsed) = serde_json::from_str::<ChatResponse>(&body) {
            return Ok(parsed.message.content);
        }

        // 2) エラーレスポンスの確認 ex. {"error": "..."}
        if let Ok(err) = serde_json::from_str::<ErrorResponse>(&body) {
            return Err(anyhow!("ollama error: {}", err.error));
        }

        // 3) 一部の設定では stream=false でも NDJSON ストリーミングチャンクを返す場合がある。
        let mut combined = String::new();
        let mut saw_any_chunk = false;
        for line in body.lines().map(str::trim).filter(|l| !l.is_empty()) {
            saw_any_chunk = true;

            if let Ok(err) = serde_json::from_str::<ErrorResponse>(line) {
                return Err(anyhow!("ollama error: {}", err.error));
            }

            let chunk: ChatChunk = serde_json::from_str(line).map_err(|e| {
                anyhow!(
                    "ollama returned unexpected json line: {} (status={})",
                    e,
                    status
                )
            })?;
            combined.push_str(&chunk.message.content);
            if chunk.done {
                break;
            }
        }
        if saw_any_chunk {
            return Ok(combined);
        }

        Err(anyhow!(
            "ollama returned unexpected response: status={} body={}",
            status,
            body
        ))
    }
}

/// チャットメッセージを表す構造体
/// # フィールド
/// * `role` - メッセージの役割 ("system", "user", "assistant")
/// * `content` - メッセージの内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String, // "system" | "user" | "assistant"
    pub content: String,
}

/// チャットリクエストを表す構造体
/// # フィールド
/// * `model` - 使用するモデル名
/// * `messages` - チャットのメッセージ履歴
/// * `stream` - ストリーミングモードの有無
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

/// チャットレスポンスを表す構造体
/// # フィールド
/// * `message` - チャットの応答メッセージ
/// * `_done` - 処理完了フラグ
#[derive(Debug, Deserialize)]
struct ChatResponse {
    message: Message,
    #[serde(rename = "done")]
    _done: bool,
}

/// チャットチャンクを表す構造体
/// # フィールド
/// * `message` - チャットの応答メッセージ
/// * `done` - 処理完了フラグ
#[derive(Debug, Deserialize)]
struct ChatChunk {
    message: Message,
    done: bool,
}

/// エラーレスポンスを表す構造体
/// # フィールド
/// * `error` - エラーメッセージ
#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn message_serde_roundtrip() {
        let msg = Message {
            role: "user".to_string(),
            content: "hello".to_string(),
        };

        let serialized = serde_json::to_string(&msg).expect("serialize Message");
        let deserialized: Message = serde_json::from_str(&serialized).expect("deserialize Message");

        assert_eq!(deserialized.role, "user");
        assert_eq!(deserialized.content, "hello");
    }

    #[test]
    fn chat_request_serializes_expected_shape() {
        let req = ChatRequest {
            model: "my-model".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are helpful".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: "Hi".to_string(),
                },
            ],
            stream: false,
        };

        let value = serde_json::to_value(&req).expect("serialize ChatRequest");
        let expected = json!({
            "model": "my-model",
            "messages": [
                {"role": "system", "content": "You are helpful"},
                {"role": "user", "content": "Hi"}
            ],
            "stream": false
        });

        assert_eq!(value, expected);
    }

    #[test]
    fn chat_response_deserializes() {
        let raw = r#"{
            "message": {"role": "assistant", "content": "Hello!"},
            "done": true
        }"#;

        let resp: ChatResponse = serde_json::from_str(raw).expect("deserialize ChatResponse");
        assert_eq!(resp.message.role, "assistant");
        assert_eq!(resp.message.content, "Hello!");
        assert!(resp._done);
    }

    #[test]
    fn chat_chunk_deserializes() {
        let raw = r#"{
            "message": {"role": "assistant", "content": "partial"},
            "done": false
        }"#;

        let chunk: ChatChunk = serde_json::from_str(raw).expect("deserialize ChatChunk");
        assert_eq!(chunk.message.role, "assistant");
        assert_eq!(chunk.message.content, "partial");
        assert!(!chunk.done);
    }

    #[test]
    fn error_response_deserializes() {
        let raw = r#"{"error": "model not found"}"#;
        let err: ErrorResponse = serde_json::from_str(raw).expect("deserialize ErrorResponse");
        assert_eq!(err.error, "model not found");
    }
}
