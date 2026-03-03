export interface TokenUsage {
  input_tokens: number
  output_tokens: number
  total_tokens: number
}

export interface CostEstimate {
  currency: string
  tier: string
  input_price_per_million: number
  output_price_per_million: number
  input_cost: number
  output_cost: number
  estimated_cost: number
}

export interface AnalysisResult {
  copywriting: string
  tags: string[]
  raw_response: string
  usage?: TokenUsage | null
  cost_estimate?: CostEstimate | null
}

export interface AnalyzeParams {
  video_path: string
  prompt: string
  model?: string
}

export interface AppSettings {
  apiKey: string
  model: string
  fps: number
  promptTemplate: string
}

export const DEFAULT_MODELS = [
  { label: '千问3.5 Plus（推荐）', value: 'qwen3.5-plus' },
  { label: '千问3.5 Flash（快速）', value: 'qwen3.5-flash' },
  { label: '千问3-VL Plus', value: 'qwen3-vl-plus' },
  { label: '千问3-VL Flash', value: 'qwen3-vl-flash' },
]

export const DEFAULT_PROMPT = `请分析这个视频的内容，并按照以下格式输出：

## 文案
根据视频内容，撰写一段适合社交媒体发布的中文文案（100-200字），要求生动有趣、有吸引力。

## 标签
列出5-10个与视频内容相关的标签关键词，用逗号分隔。`

export const PROMPT_TEMPLATES = [
  {
    label: '通用文案+标签',
    value: DEFAULT_PROMPT,
  },
  {
    label: '短视频带货文案',
    value: `请分析这个视频中展示的产品/服务，并按以下格式输出：

## 文案
撰写一段种草带货风格的文案（150-250字），突出产品亮点和使用场景，语气亲切自然，适合抖音/小红书发布。

## 标签
列出8-12个相关标签，包含产品类目、使用场景、用户群体等维度，用逗号分隔。`,
  },
  {
    label: '知识科普文案',
    value: `请分析这个视频的知识内容，并按以下格式输出：

## 文案
撰写一段知识科普类文案（100-200字），简洁明了地总结核心知识点，引发读者思考。

## 标签
列出5-8个相关标签关键词，用逗号分隔。`,
  },
  {
    label: 'Vlog 生活记录',
    value: `请分析这个 Vlog 视频内容，并按以下格式输出：

## 文案
用轻松、治愈的语气撰写一段生活记录类文案（80-150字），分享日常感受。

## 标签
列出5-8个相关标签关键词，用逗号分隔。`,
  },
]
