use super::types::*;

const API_BASE: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";

#[derive(Clone, Copy)]
struct PricingTier {
    max_input_tokens: u64,
    input_price_per_million: f64,
    output_price_per_million: f64,
    tier_name: &'static str,
}

// 价格单位：人民币 / 百万 tokens（参考 DashScope 模型价格页，按输入 tokens 所在阶梯估算）。
const QWEN35_PLUS_TIERS: [PricingTier; 2] = [
    PricingTier {
        max_input_tokens: 256_000,
        input_price_per_million: 2.936,
        output_price_per_million: 17.614,
        tier_name: "0~256K",
    },
    PricingTier {
        max_input_tokens: 1_000_000,
        input_price_per_million: 3.67,
        output_price_per_million: 22.018,
        tier_name: "256K~1M",
    },
];

const QWEN35_FLASH_TIERS: [PricingTier; 3] = [
    PricingTier {
        max_input_tokens: 128_000,
        input_price_per_million: 0.2,
        output_price_per_million: 2.0,
        tier_name: "0~128K",
    },
    PricingTier {
        max_input_tokens: 256_000,
        input_price_per_million: 0.8,
        output_price_per_million: 8.0,
        tier_name: "128K~256K",
    },
    PricingTier {
        max_input_tokens: 1_000_000,
        input_price_per_million: 1.2,
        output_price_per_million: 12.0,
        tier_name: "256K~1M",
    },
];

const QWEN3_VL_PLUS_TIERS: [PricingTier; 3] = [
    PricingTier {
        max_input_tokens: 32_000,
        input_price_per_million: 1.468,
        output_price_per_million: 11.743,
        tier_name: "0~32K",
    },
    PricingTier {
        max_input_tokens: 128_000,
        input_price_per_million: 2.202,
        output_price_per_million: 17.614,
        tier_name: "32K~128K",
    },
    PricingTier {
        max_input_tokens: 256_000,
        input_price_per_million: 4.404,
        output_price_per_million: 35.228,
        tier_name: "128K~256K",
    },
];

const QWEN3_VL_FLASH_TIERS: [PricingTier; 3] = [
    PricingTier {
        max_input_tokens: 32_000,
        input_price_per_million: 0.367,
        output_price_per_million: 2.936,
        tier_name: "0~32K",
    },
    PricingTier {
        max_input_tokens: 128_000,
        input_price_per_million: 0.55,
        output_price_per_million: 4.404,
        tier_name: "32K~128K",
    },
    PricingTier {
        max_input_tokens: 256_000,
        input_price_per_million: 0.881,
        output_price_per_million: 7.046,
        tier_name: "128K~256K",
    },
];

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

    if let Some(err) = chat_resp.error.as_ref() {
        return Err(format!(
            "API错误 [{}]: {}",
            err.code.clone().unwrap_or_default(),
            err.message
        ));
    }

    let raw_content = chat_resp
        .choices
        .as_ref()
        .and_then(|c| c.first())
        .map(|c| c.message.content.clone())
        .unwrap_or_default();

    let usage = normalize_usage(chat_resp.usage.as_ref());
    let result = parse_result(&raw_content, usage, model);
    Ok(result)
}

fn parse_result(raw: &str, usage: Option<TokenUsage>, model: &str) -> AnalysisResult {
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

    let cost_estimate = usage
        .as_ref()
        .and_then(|u| estimate_cost(model, u.input_tokens, u.output_tokens));

    AnalysisResult {
        copywriting,
        tags,
        raw_response: raw.to_string(),
        usage,
        cost_estimate,
    }
}

fn normalize_usage(raw_usage: Option<&ChatUsage>) -> Option<TokenUsage> {
    let usage = raw_usage?;
    let input_tokens = usage.prompt_tokens.or(usage.input_tokens).unwrap_or(0);
    let output_tokens = usage
        .completion_tokens
        .or(usage.output_tokens)
        .unwrap_or(0);
    let total_tokens = usage
        .total_tokens
        .unwrap_or(input_tokens.saturating_add(output_tokens));

    if input_tokens == 0 && output_tokens == 0 && total_tokens == 0 {
        return None;
    }

    Some(TokenUsage {
        input_tokens,
        output_tokens,
        total_tokens,
    })
}

fn estimate_cost(model: &str, input_tokens: u64, output_tokens: u64) -> Option<CostEstimate> {
    let tier = select_pricing_tier(model, input_tokens)?;
    let input_cost = (input_tokens as f64 / 1_000_000.0) * tier.input_price_per_million;
    let output_cost = (output_tokens as f64 / 1_000_000.0) * tier.output_price_per_million;
    let estimated_cost = input_cost + output_cost;

    Some(CostEstimate {
        currency: "CNY".to_string(),
        tier: tier.tier_name.to_string(),
        input_price_per_million: tier.input_price_per_million,
        output_price_per_million: tier.output_price_per_million,
        input_cost: round6(input_cost),
        output_cost: round6(output_cost),
        estimated_cost: round6(estimated_cost),
    })
}

fn select_pricing_tier(model: &str, input_tokens: u64) -> Option<PricingTier> {
    let model_lower = model.to_ascii_lowercase();
    let tiers: &[PricingTier] = if model_lower.contains("qwen3.5-plus")
        || model_lower == "qwen-plus"
        || model_lower.starts_with("qwen-plus-")
    {
        &QWEN35_PLUS_TIERS
    } else if model_lower.contains("qwen3.5-flash")
        || model_lower == "qwen-flash"
        || model_lower.starts_with("qwen-flash-")
    {
        &QWEN35_FLASH_TIERS
    } else if model_lower.contains("qwen3-vl-plus") {
        &QWEN3_VL_PLUS_TIERS
    } else if model_lower.contains("qwen3-vl-flash") {
        &QWEN3_VL_FLASH_TIERS
    } else {
        return None;
    };

    for tier in tiers {
        if input_tokens <= tier.max_input_tokens {
            return Some(*tier);
        }
    }
    tiers.last().copied()
}

fn round6(v: f64) -> f64 {
    (v * 1_000_000.0).round() / 1_000_000.0
}
