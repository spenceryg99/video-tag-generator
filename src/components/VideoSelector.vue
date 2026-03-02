<script setup lang="ts">
import { ref } from 'vue'
import { NButton, NText } from 'naive-ui'
import { open } from '@tauri-apps/plugin-dialog'
import { useProcessingStore } from '@/stores/processing'

const processing = useProcessingStore()
const isDragOver = ref(false)

async function selectVideo() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: '视频文件',
        extensions: ['mp4', 'avi', 'mov', 'mkv', 'flv', 'wmv', 'webm'],
      },
    ],
  })

  if (selected) {
    processing.setVideo(selected as string)
  }
}

function handleDrop(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = false
  const files = e.dataTransfer?.files
  if (files && files.length > 0) {
    const file = files[0]
    if (file.type.startsWith('video/') || /\.(mp4|avi|mov|mkv|flv|wmv|webm)$/i.test(file.name)) {
      // Tauri drag-drop 会传递文件路径
      processing.setVideo((file as any).path || file.name)
    }
  }
}

function handleDragOver(e: DragEvent) {
  e.preventDefault()
  isDragOver.value = true
}

function handleDragLeave() {
  isDragOver.value = false
}
</script>

<template>
  <div
    class="video-selector"
    :class="{ 'drag-over': isDragOver, 'has-video': processing.videoPath }"
    @drop="handleDrop"
    @dragover="handleDragOver"
    @dragleave="handleDragLeave"
    @click="selectVideo"
  >
    <template v-if="processing.videoPath">
      <div class="selected-info">
        <span class="file-icon">🎬</span>
        <n-text strong>{{ processing.videoName }}</n-text>
        <n-button
          text
          type="error"
          size="small"
          @click.stop="processing.clearVideo()"
          style="margin-left: 8px;"
        >
          移除
        </n-button>
      </div>
    </template>
    <template v-else>
      <div class="upload-placeholder">
        <span class="upload-icon">📁</span>
        <n-text depth="3" style="font-size: 14px;">
          点击选择视频文件，或拖拽到此处
        </n-text>
        <n-text depth="3" style="font-size: 12px; margin-top: 4px;">
          支持 MP4、AVI、MOV、MKV 等格式
        </n-text>
      </div>
    </template>
  </div>
</template>

<style scoped>
.video-selector {
  border: 2px dashed #d9d9d9;
  border-radius: 8px;
  padding: 24px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s;
  min-height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.video-selector:hover {
  border-color: #18a058;
}

.video-selector.drag-over {
  border-color: #18a058;
  background-color: rgba(24, 160, 88, 0.06);
}

.video-selector.has-video {
  border-style: solid;
  border-color: #18a058;
  background-color: rgba(24, 160, 88, 0.04);
}

.upload-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.upload-icon {
  font-size: 32px;
  margin-bottom: 8px;
}

.selected-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.file-icon {
  font-size: 24px;
}
</style>
