use serde::{Deserialize, Serialize};
use tauri::State;

use crate::dashscope;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeParams {
    pub video_path: String,
    pub prompt: String,
    pub model: Option<String>,
}

#[tauri::command]
pub async fn analyze_video(
    state: State<'_, AppState>,
    params: AnalyzeParams,
) -> Result<dashscope::types::AnalysisResult, String> {
    let api_key = state.api_key.lock().await;
    let api_key = api_key.as_ref().ok_or("请先设置 API Key")?;

    let model = params.model.unwrap_or_else(|| "qwen3.5-plus".to_string());

    // 1. 上传视频到 DashScope 临时存储
    eprintln!("[DEBUG] 开始上传视频: {}", params.video_path);
    let oss_url = dashscope::upload::upload_video(api_key, &model, &params.video_path).await?;
    eprintln!("[DEBUG] 上传成功，OSS URL: {}", oss_url);

    // 2. 调用多模态 API 分析视频
    eprintln!("[DEBUG] 开始调用API，模型: {}", model);
    let result = dashscope::client::analyze_video_with_prompt(
        api_key,
        &model,
        &oss_url,
        &params.prompt,
    )
    .await?;
    eprintln!("[DEBUG] API调用成功");

    Ok(result)
}

#[tauri::command]
pub async fn set_api_key(state: State<'_, AppState>, api_key: String) -> Result<(), String> {
    let mut key = state.api_key.lock().await;
    *key = Some(api_key);
    Ok(())
}

#[tauri::command]
pub async fn get_api_key_masked(state: State<'_, AppState>) -> Result<String, String> {
    let key = state.api_key.lock().await;
    match key.as_ref() {
        Some(k) if k.len() > 8 => {
            let prefix = &k[..4];
            let suffix = &k[k.len() - 4..];
            Ok(format!("{}****{}", prefix, suffix))
        }
        Some(_) => Ok("****".to_string()),
        None => Ok(String::new()),
    }
}

#[tauri::command]
pub async fn validate_api_key(api_key: String) -> Result<bool, String> {
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "model": "qwen3.5-plus",
        "messages": [{"role": "user", "content": [{"type": "text", "text": "hi"}]}],
        "max_tokens": 1
    });

    let resp = client
        .post("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("验证请求失败: {}", e))?;

    Ok(resp.status().is_success())
}
