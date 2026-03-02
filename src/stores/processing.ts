import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AnalysisResult } from '@/types'
import { analyzeVideo } from '@/utils/commands'
import { useSettingsStore } from './settings'

export const useProcessingStore = defineStore('processing', () => {
  const isProcessing = ref(false)
  const error = ref<string | null>(null)
  const result = ref<AnalysisResult | null>(null)
  const videoPath = ref<string | null>(null)
  const videoName = ref<string | null>(null)

  function setVideo(path: string) {
    videoPath.value = path
    videoName.value = path.split(/[/\\]/).pop() || path
    result.value = null
    error.value = null
  }

  function clearVideo() {
    videoPath.value = null
    videoName.value = null
    result.value = null
    error.value = null
  }

  async function startAnalysis(prompt: string) {
    if (!videoPath.value) {
      error.value = '请先选择视频文件'
      return
    }

    const settings = useSettingsStore()
    if (!settings.apiKey) {
      error.value = '请先设置 API Key'
      return
    }

    isProcessing.value = true
    error.value = null
    result.value = null

    try {
      result.value = await analyzeVideo({
        video_path: videoPath.value,
        prompt,
        model: settings.model,
      })
    } catch (e: any) {
      error.value = typeof e === 'string' ? e : e.message || '分析失败'
    } finally {
      isProcessing.value = false
    }
  }

  return {
    isProcessing,
    error,
    result,
    videoPath,
    videoName,
    setVideo,
    clearVideo,
    startAnalysis,
  }
})
