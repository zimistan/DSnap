<script setup lang="ts">
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useElementSize } from '@vueuse/core'
import { ref } from 'vue'

// 自动导入已配置：ref, useElementSize 等都已自动导入

const container = ref<HTMLElement | null>(null)
const { width, height } = useElementSize(container)
const currentWindow = WebviewWindow.getCurrent()
function closeWindow() {
  // 关闭当前窗口
  currentWindow.close()
}

// 监听键盘事件，按 ESC 关闭窗口
document.addEventListener('keydown', (event) => {
  if (event.key === 'Escape') {
    closeWindow()
  }
})
</script>

<template>
  <div ref="container" class="size-screen bg-red">
    <button class="close-btn" @click="closeWindow">
      关闭
    </button>
    {{ width }} {{ height }}
    <!-- <img v-if="screenshot" :src="screenshot" class="screenshot" alt="Screenshot"> -->
  </div>
</template>
