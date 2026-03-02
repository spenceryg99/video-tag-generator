import { defineStore } from 'pinia'
import { ref } from 'vue'
import { load } from '@tauri-apps/plugin-store'
import { setApiKey, getApiKeyMasked } from '@/utils/commands'
import { DEFAULT_PROMPT } from '@/types'

const STORE_DEFAULTS = {
  apiKey: '',
  model: 'qwen3.5-plus',
  fps: 2,
  promptTemplate: DEFAULT_PROMPT,
}

async function getStore() {
  return load('settings.json', { defaults: STORE_DEFAULTS, autoSave: true })
}

export const useSettingsStore = defineStore('settings', () => {
  const apiKey = ref('')
  const apiKeyMasked = ref('')
  const model = ref('qwen3.5-plus')
  const fps = ref(2)
  const promptTemplate = ref(DEFAULT_PROMPT)
  const isInitialized = ref(false)

  async function init() {
    try {
      const store = await getStore()
      const savedKey = await store.get<string>('apiKey')
      const savedModel = await store.get<string>('model')
      const savedFps = await store.get<number>('fps')
      const savedPrompt = await store.get<string>('promptTemplate')

      if (savedKey) {
        apiKey.value = savedKey
        await setApiKey(savedKey)
        apiKeyMasked.value = await getApiKeyMasked()
      }
      if (savedModel) model.value = savedModel
      if (savedFps) fps.value = savedFps
      if (savedPrompt) promptTemplate.value = savedPrompt
    } catch (e) {
      console.error('加载设置失败:', e)
    }
    isInitialized.value = true
  }

  async function saveApiKey(key: string) {
    apiKey.value = key
    await setApiKey(key)
    apiKeyMasked.value = await getApiKeyMasked()
    const store = await getStore()
    await store.set('apiKey', key)
  }

  async function saveModel(m: string) {
    model.value = m
    const store = await getStore()
    await store.set('model', m)
  }

  async function saveFps(f: number) {
    fps.value = f
    const store = await getStore()
    await store.set('fps', f)
  }

  async function savePromptTemplate(p: string) {
    promptTemplate.value = p
    const store = await getStore()
    await store.set('promptTemplate', p)
  }

  return {
    apiKey,
    apiKeyMasked,
    model,
    fps,
    promptTemplate,
    isInitialized,
    init,
    saveApiKey,
    saveModel,
    saveFps,
    savePromptTemplate,
  }
})
