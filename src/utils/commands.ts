import { invoke } from '@tauri-apps/api/core'
import type { AnalysisResult, AnalyzeParams } from '@/types'

export async function analyzeVideo(params: AnalyzeParams): Promise<AnalysisResult> {
  return invoke<AnalysisResult>('analyze_video', { params })
}

export async function setApiKey(apiKey: string): Promise<void> {
  return invoke('set_api_key', { apiKey })
}

export async function getApiKeyMasked(): Promise<string> {
  return invoke<string>('get_api_key_masked')
}

export async function validateApiKey(apiKey: string): Promise<boolean> {
  return invoke<boolean>('validate_api_key', { apiKey })
}
