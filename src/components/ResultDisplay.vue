<script setup lang="ts">
import { NCard, NTag, NButton, NSpace, NText, NCollapse, NCollapseItem, useMessage } from 'naive-ui'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { useProcessingStore } from '@/stores/processing'

const processing = useProcessingStore()
const message = useMessage()

async function copyCopywriting() {
  if (!processing.result) return
  try {
    await writeText(processing.result.copywriting)
    message.success('文案已复制到剪贴板')
  } catch {
    message.error('复制失败')
  }
}

async function copyTags() {
  if (!processing.result) return
  try {
    const text = processing.result.tags.map(t => `#${t}`).join(' ')
    await writeText(text)
    message.success('标签已复制到剪贴板')
  } catch {
    message.error('复制失败')
  }
}

async function copyAll() {
  if (!processing.result) return
  try {
    const text = `${processing.result.copywriting}\n\n${processing.result.tags.map(t => `#${t}`).join(' ')}`
    await writeText(text)
    message.success('全部内容已复制到剪贴板')
  } catch {
    message.error('复制失败')
  }
}
</script>

<template>
  <div v-if="processing.result" class="result-display">
    <n-card title="文案" size="small">
      <template #header-extra>
        <n-button text type="primary" size="small" @click="copyCopywriting">
          复制文案
        </n-button>
      </template>
      <div class="copywriting-content">
        {{ processing.result.copywriting }}
      </div>
    </n-card>

    <n-card title="标签" size="small" style="margin-top: 12px;">
      <template #header-extra>
        <n-button text type="primary" size="small" @click="copyTags">
          复制标签
        </n-button>
      </template>
      <n-space>
        <n-tag
          v-for="tag in processing.result.tags"
          :key="tag"
          type="success"
          round
        >
          #{{ tag }}
        </n-tag>
        <n-text v-if="processing.result.tags.length === 0" depth="3">
          未识别到标签
        </n-text>
      </n-space>
    </n-card>

    <n-space style="margin-top: 12px;" justify="end">
      <n-button type="primary" @click="copyAll">
        复制全部
      </n-button>
    </n-space>

    <n-collapse style="margin-top: 12px;">
      <n-collapse-item title="查看原始响应" name="raw">
        <pre class="raw-response">{{ processing.result.raw_response }}</pre>
      </n-collapse-item>
    </n-collapse>
  </div>
</template>

<style scoped>
.result-display {
  margin-top: 16px;
}

.copywriting-content {
  white-space: pre-wrap;
  line-height: 1.8;
}

.raw-response {
  white-space: pre-wrap;
  word-break: break-all;
  font-size: 12px;
  background: #f5f5f5;
  padding: 12px;
  border-radius: 4px;
  max-height: 200px;
  overflow-y: auto;
}
</style>
