use super::types::*;

const API_BASE: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

pub async fn analyze_video_with_prompt(
    api_key: &str,
    model: &str,
    video_oss_url: &str,
    prompt: &str,
) -> Result<AnalysisResult, String> {
    let request = ChatRequest {
        model: model.to_string(),
        messages: vec![ChatMessage {
            role: "user".to_string(),
            content: vec![
                ContentPart::VideoUrl {
                    video_url: VideoUrlContent {
                        url: video_oss_url.to_string(),
                    },
                },
                ContentPart::Text {
                    text: prompt.to_string(),
                },
            ],
        }],
    };

    let request_json = serde_json::to_string_pretty(&request).unwrap_or_default();
    eprintln!("[DEBUG] API请求体:\n{}", request_json);

    let client = reqwest::Client::new();
    let resp = client
        .post(API_BASE)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("X-DashScope-OssResourceResolve", "enable")
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("调用API失败: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API返回错误 ({}): {}", status, text));
    }

    let chat_resp: ChatResponse = resp
        .json()
        .await
        .map_err(|e| format!("解析API响应失败: {}", e))?;

    if let Some(err) = chat_resp.error {
        return Err(format!("API错误 [{}]: {}", err.code.unwrap_or_default(), err.message));
    }

    let raw_content = chat_resp
        .choices
        .and_then(|c| c.into_iter().next())
        .map(|c| c.message.content)
        .unwrap_or_default();

    let result = parse_result(&raw_content);
    Ok(result)
}

fn parse_result(raw: &str) -> AnalysisResult {
    let mut copywriting = String::new();
    let mut tags: Vec<String> = Vec::new();

    let mut in_copy_section = false;
    let mut in_tags_section = false;

    for line in raw.lines() {
        let trimmed = line.trim();

        if trimmed.contains("文案") || trimmed.contains("copywriting") || trimmed.contains("Copy") {
            in_copy_section = true;
            in_tags_section = false;
            continue;
        }
        if trimmed.contains("标签") || trimmed.contains("tags") || trimmed.contains("Tags") || trimmed.contains("关键词") {
            in_copy_section = false;
            in_tags_section = true;
            continue;
        }

        if in_tags_section {
            let cleaned = trimmed
                .trim_start_matches(|c: char| c == '-' || c == '•' || c == '*' || c == '#' || c.is_numeric() || c == '.' || c == ' ');
            let cleaned = cleaned.trim_matches(|c: char| c == '#' || c == ' ');
            if !cleaned.is_empty() {
                // 支持逗号、顿号分隔的标签
                for tag in cleaned.split(|c: char| c == ',' || c == '、' || c == '，' || c == ';' || c == '；') {
                    let t = tag.trim().trim_matches('#').trim();
                    if !t.is_empty() {
                        tags.push(t.to_string());
                    }
                }
            }
        } else if in_copy_section {
            if !trimmed.is_empty() {
                if !copywriting.is_empty() {
                    copywriting.push('\n');
                }
                copywriting.push_str(trimmed);
            }
        }
    }

    // 如果结构化解析失败，回退：整个内容作为文案
    if copywriting.is_empty() && tags.is_empty() {
        copywriting = raw.to_string();
    }

    AnalysisResult {
        copywriting,
        tags,
        raw_response: raw.to_string(),
    }
}
