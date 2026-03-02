<script setup lang="ts">
import { ref, computed } from 'vue'
import { NInput, NSelect, NText } from 'naive-ui'
import { useSettingsStore } from '@/stores/settings'
import { PROMPT_TEMPLATES } from '@/types'

const settings = useSettingsStore()

const templateOptions = computed(() =>
  PROMPT_TEMPLATES.map((t, i) => ({
    label: t.label,
    value: i.toString(),
  }))
)

const selectedTemplate = ref<string | null>(null)

function onTemplateChange(val: string) {
  selectedTemplate.value = val
  const idx = parseInt(val)
  if (PROMPT_TEMPLATES[idx]) {
    settings.savePromptTemplate(PROMPT_TEMPLATES[idx].value)
  }
}

function onPromptInput(val: string) {
  settings.savePromptTemplate(val)
  selectedTemplate.value = null
}
</script>

<template>
  <div class="prompt-editor">
    <div class="prompt-header">
      <n-text strong>提示词</n-text>
      <n-select
        v-model:value="selectedTemplate"
        :options="templateOptions"
        placeholder="选择模板"
        size="small"
        style="width: 180px;"
        clearable
        @update:value="onTemplateChange"
      />
    </div>
    <n-input
      :value="settings.promptTemplate"
      type="textarea"
      placeholder="输入你的提示词..."
      :autosize="{ minRows: 4, maxRows: 10 }"
      @update:value="onPromptInput"
    />
  </div>
</template>

<style scoped>
.prompt-editor {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.prompt-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
</style>
