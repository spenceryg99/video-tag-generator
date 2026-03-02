<script setup lang="ts">
import { ref } from 'vue'
import {
  NModal,
  NCard,
  NForm,
  NFormItem,
  NInput,
  NSelect,
  NSlider,
  NButton,
  NSpace,
  NText,
  useMessage,
} from 'naive-ui'
import { useSettingsStore } from '@/stores/settings'
import { validateApiKey } from '@/utils/commands'
import { DEFAULT_MODELS } from '@/types'

const props = defineProps<{ show: boolean }>()
const emit = defineEmits<{ (e: 'update:show', value: boolean): void }>()

const settings = useSettingsStore()
const message = useMessage()

const apiKeyInput = ref('')
const isValidating = ref(false)

function onOpen() {
  apiKeyInput.value = settings.apiKey
}

async function onValidate() {
  if (!apiKeyInput.value) {
    message.warning('请输入 API Key')
    return
  }
  isValidating.value = true
  try {
    const valid = await validateApiKey(apiKeyInput.value)
    if (valid) {
      message.success('API Key 验证通过')
    } else {
      message.error('API Key 无效，请检查')
    }
  } catch (e: any) {
    message.error(`验证失败: ${e}`)
  } finally {
    isValidating.value = false
  }
}

async function onSave() {
  await settings.saveApiKey(apiKeyInput.value)
  message.success('设置已保存')
  emit('update:show', false)
}

function onModelChange(val: string) {
  settings.saveModel(val)
}

function onFpsChange(val: number) {
  settings.saveFps(val)
}
</script>

<template>
  <n-modal
    :show="props.show"
    @update:show="emit('update:show', $event)"
    @after-enter="onOpen"
  >
    <n-card
      title="设置"
      style="width: 500px;"
      :bordered="false"
      role="dialog"
      closable
      @close="emit('update:show', false)"
    >
      <n-form label-placement="left" label-width="90">
        <n-form-item label="API Key">
          <n-space vertical style="width: 100%;">
            <n-input
              v-model:value="apiKeyInput"
              type="password"
              show-password-on="click"
              placeholder="输入阿里云百炼 API Key (sk-xxx)"
            />
            <n-space>
              <n-button size="small" :loading="isValidating" @click="onValidate">
                验证
              </n-button>
              <n-text v-if="settings.apiKeyMasked" depth="3" style="font-size: 12px;">
                当前: {{ settings.apiKeyMasked }}
              </n-text>
            </n-space>
          </n-space>
        </n-form-item>

        <n-form-item label="模型">
          <n-select
            :value="settings.model"
            :options="DEFAULT_MODELS"
            @update:value="onModelChange"
          />
        </n-form-item>

        <n-form-item label="视频 FPS">
          <n-slider
            :value="settings.fps"
            :min="0.5"
            :max="5"
            :step="0.5"
            :marks="{ 1: '1', 2: '2', 3: '3', 5: '5' }"
            @update:value="onFpsChange"
          />
        </n-form-item>

        <n-form-item label=" ">
          <n-text depth="3" style="font-size: 12px;">
            FPS 越高分析越精细，但消耗 Token 越多。建议短视频用 2，长视频用 1。
          </n-text>
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="emit('update:show', false)">取消</n-button>
          <n-button type="primary" @click="onSave">保存</n-button>
        </n-space>
      </template>
    </n-card>
  </n-modal>
</template>
