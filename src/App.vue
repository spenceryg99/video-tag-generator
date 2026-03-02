<script setup lang="ts">
import { ref, onMounted } from 'vue'
import {
  NConfigProvider,
  NLayout,
  NLayoutHeader,
  NLayoutContent,
  NButton,
  NText,
  NMessageProvider,
  zhCN,
  dateZhCN,
} from 'naive-ui'
import VideoSelector from '@/components/VideoSelector.vue'
import PromptEditor from '@/components/PromptEditor.vue'
import ProcessingStatus from '@/components/ProcessingStatus.vue'
import ResultDisplay from '@/components/ResultDisplay.vue'
import SettingsDialog from '@/components/SettingsDialog.vue'
import { useSettingsStore } from '@/stores/settings'

const settings = useSettingsStore()
const showSettings = ref(false)

onMounted(() => {
  settings.init()
})
</script>

<template>
  <n-config-provider :locale="zhCN" :date-locale="dateZhCN">
    <n-message-provider>
      <n-layout style="height: 100vh;">
        <n-layout-header bordered style="padding: 12px 24px; display: flex; align-items: center; justify-content: space-between;">
          <n-text strong style="font-size: 18px;">
            视频文案标签生成器
          </n-text>
          <n-button quaternary circle @click="showSettings = true">
            ⚙️
          </n-button>
        </n-layout-header>

        <n-layout-content style="padding: 24px; overflow-y: auto;">
          <div class="main-content">
            <VideoSelector />

            <div style="margin-top: 16px;">
              <PromptEditor />
            </div>

            <div style="margin-top: 16px;">
              <ProcessingStatus />
            </div>

            <ResultDisplay />
          </div>
        </n-layout-content>
      </n-layout>

      <SettingsDialog v-model:show="showSettings" />
    </n-message-provider>
  </n-config-provider>
</template>

<style>
body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

.main-content {
  max-width: 800px;
  margin: 0 auto;
}
</style>
