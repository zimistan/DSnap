<script setup lang="ts">
import { onMounted } from 'vue'

onMounted(async () => {
})
</script>

<template>
  <div class="p-4 space-y-4">
    <h1 class="text-xl font-bold">
      Home
    </h1>
    <p>欢迎使用 DSnap。点击进入截图页：</p>
    <RouterLink to="/screenshot" class="text-blue-500 underline">
      前往截图
    </RouterLink>
  </div>
</template>
