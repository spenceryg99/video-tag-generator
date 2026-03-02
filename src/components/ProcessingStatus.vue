<script setup lang="ts">
import { NButton, NAlert, NSpin, NText } from 'naive-ui'
import { useProcessingStore } from '@/stores/processing'
import { useSettingsStore } from '@/stores/settings'

const processing = useProcessingStore()
const settings = useSettingsStore()

function startAnalysis() {
  processing.startAnalysis(settings.promptTemplate)
}
</script>

<template>
  <div class="processing-status">
    <n-button
      type="primary"
      size="large"
      block
      :loading="processing.isProcessing"
      :disabled="!processing.videoPath || !settings.apiKey || processing.isProcessing"
      @click="startAnalysis"
    >
      {{ processing.isProcessing ? '分析中...' : '开始分析' }}
    </n-button>

    <n-alert
      v-if="!settings.apiKey && settings.isInitialized"
      type="warning"
      title="未设置 API Key"
      style="margin-top: 12px;"
    >
      请点击右上角设置按钮配置阿里云百炼 API Key
    </n-alert>

    <div v-if="processing.isProcessing" class="loading-section">
      <n-spin size="small" />
      <n-text depth="3">正在上传视频并分析，请耐心等待...</n-text>
    </div>

    <n-alert
      v-if="processing.error"
      type="error"
      title="分析失败"
      closable
      style="margin-top: 12px;"
    >
      {{ processing.error }}
    </n-alert>
  </div>
</template>

<style scoped>
.loading-section {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 12px;
  padding: 12px;
  background: rgba(24, 160, 88, 0.04);
  border-radius: 6px;
}
</style>
