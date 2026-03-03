<script setup lang="ts">
import { NCard, NTag, NButton, NSpace, NText, NCollapse, NCollapseItem, useMessage } from 'naive-ui'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import { useProcessingStore } from '@/stores/processing'

const processing = useProcessingStore()
const message = useMessage()

function formatTokens(value?: number) {
  if (typeof value !== 'number') return '-'
  return value.toLocaleString('zh-CN')
}

function formatCost(value?: number) {
  if (typeof value !== 'number') return '-'
  return `¥${value < 0.01 ? value.toFixed(6) : value.toFixed(4)}`
}

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

    <n-card title="本次消耗" size="small" style="margin-top: 12px;">
      <div v-if="processing.result.usage" class="stats-grid">
        <div class="stats-item">
          <n-text depth="3">输入 Token</n-text>
          <n-text strong>{{ formatTokens(processing.result.usage.input_tokens) }}</n-text>
        </div>
        <div class="stats-item">
          <n-text depth="3">输出 Token</n-text>
          <n-text strong>{{ formatTokens(processing.result.usage.output_tokens) }}</n-text>
        </div>
        <div class="stats-item">
          <n-text depth="3">总 Token</n-text>
          <n-text strong>{{ formatTokens(processing.result.usage.total_tokens) }}</n-text>
        </div>
        <div class="stats-item">
          <n-text depth="3">预估费用</n-text>
          <n-text strong>
            {{ formatCost(processing.result.cost_estimate?.estimated_cost) }}
          </n-text>
        </div>
      </div>
      <n-text v-else depth="3">
        当前响应未返回 usage，无法统计 Token/费用。
      </n-text>

      <n-text
        v-if="processing.result.cost_estimate"
        depth="3"
        style="display: block; margin-top: 8px; font-size: 12px;"
      >
        阶梯 {{ processing.result.cost_estimate.tier }}，输入单价
        ¥{{ processing.result.cost_estimate.input_price_per_million }}/百万 tokens，
        输出单价 ¥{{ processing.result.cost_estimate.output_price_per_million }}/百万 tokens。
      </n-text>
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

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px;
}

.stats-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 10px;
  border-radius: 6px;
  background: #fafafa;
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
