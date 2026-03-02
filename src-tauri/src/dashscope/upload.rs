use reqwest::multipart;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use super::types::UploadPolicyResponse;

const UPLOAD_API: &str = "https://dashscope.aliyuncs.com/api/v1/uploads";

fn generate_safe_filename(original_path: &str) -> String {
    let path = Path::new(original_path);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp4");
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("video_{}.{}", ts, ext)
}

pub async fn get_upload_policy(api_key: &str, model: &str) -> Result<super::types::UploadPolicyData, String> {
    let client = reqwest::Client::new();
    eprintln!("[DEBUG] 请求上传凭证, model={}", model);
    let resp = client
        .get(UPLOAD_API)
        .header("Authorization", format!("Bearer {}", api_key))
        .query(&[("action", "getPolicy"), ("model", model)])
        .send()
        .await
        .map_err(|e| format!("获取上传凭证失败: {}", e))?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    eprintln!("[DEBUG] 上传凭证响应 status={}, body={}", status, &body[..body.len().min(500)]);

    if !status.is_success() {
        return Err(format!("获取上传凭证失败 ({}): {}", status, body));
    }

    let policy_resp: UploadPolicyResponse = serde_json::from_str(&body)
        .map_err(|e| format!("解析上传凭证失败: {}", e))?;

    eprintln!("[DEBUG] upload_host={}, upload_dir={}", policy_resp.data.upload_host, policy_resp.data.upload_dir);
    Ok(policy_resp.data)
}

pub async fn upload_file_to_oss(
    policy: &super::types::UploadPolicyData,
    file_path: &str,
) -> Result<String, String> {
    let safe_name = generate_safe_filename(file_path);
    let key = format!("{}/{}", policy.upload_dir, safe_name);

    let file_bytes = tokio::fs::read(file_path)
        .await
        .map_err(|e| format!("读取文件失败: {}", e))?;

    let content_type = match Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp4")
    {
        "mp4" => "video/mp4",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",
        "wmv" => "video/x-ms-wmv",
        "flv" => "video/x-flv",
        "webm" => "video/webm",
        _ => "video/mp4",
    };

    let form = multipart::Form::new()
        .text("OSSAccessKeyId", policy.oss_access_key_id.clone())
        .text("Signature", policy.signature.clone())
        .text("policy", policy.policy.clone())
        .text("x-oss-object-acl", policy.x_oss_object_acl.clone())
        .text("x-oss-forbid-overwrite", policy.x_oss_forbid_overwrite.clone())
        .text("key", key.clone())
        .text("success_action_status", "200".to_string())
        .text("x-oss-content-type", content_type.to_string())
        .part(
            "file",
            multipart::Part::bytes(file_bytes)
                .file_name(safe_name.clone())
                .mime_str(content_type)
                .unwrap(),
        );

    let client = reqwest::Client::new();
    let resp = client
        .post(&policy.upload_host)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("上传文件到OSS失败: {}", e))?;

    let oss_status = resp.status();
    let oss_body = resp.text().await.unwrap_or_default();
    eprintln!("[DEBUG] OSS上传响应 status={}, body={}", oss_status, &oss_body[..oss_body.len().min(500)]);

    if !oss_status.is_success() {
        return Err(format!("上传文件到OSS失败 ({}): {}", oss_status, oss_body));
    }

    let url = format!("oss://{}", key);
    eprintln!("[DEBUG] 生成的OSS URL: {}", url);
    Ok(url)
}

pub async fn upload_video(api_key: &str, model: &str, file_path: &str) -> Result<String, String> {
    let policy = get_upload_policy(api_key, model).await?;
    upload_file_to_oss(&policy, file_path).await
}
