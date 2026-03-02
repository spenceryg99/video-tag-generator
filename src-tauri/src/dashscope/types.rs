use serde::{Deserialize, Serialize};

// ========== 上传相关 ==========

#[derive(Debug, Deserialize)]
pub struct UploadPolicyResponse {
    pub data: UploadPolicyData,
}

#[derive(Debug, Deserialize)]
pub struct UploadPolicyData {
    pub policy: String,
    pub signature: String,
    pub upload_dir: String,
    pub upload_host: String,
    pub oss_access_key_id: String,
    pub x_oss_object_acl: String,
    pub x_oss_forbid_overwrite: String,
}

// ========== OpenAI 兼容接口 ==========

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: Vec<ContentPart>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "video_url")]
    VideoUrl { video_url: VideoUrlContent },
}

#[derive(Debug, Serialize)]
pub struct VideoUrlContent {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Option<Vec<ChatChoice>>,
    pub error: Option<ApiError>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatChoiceMessage,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoiceMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: Option<String>,
}

// ========== 应用层结果 ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub copywriting: String,
    pub tags: Vec<String>,
    pub raw_response: String,
}
