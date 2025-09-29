import { setupLayouts } from 'virtual:generated-layouts'
import { createRouter, createWebHashHistory } from 'vue-router'
import generatedRoutes from '~pages'

const routes = setupLayouts(generatedRoutes)

console.warn(routes)

const router = createRouter({
  // Hash history 更适合 Tauri file:// & devUrl 混合场景，若要 history 需自定义协议或处理刷新
  history: createWebHashHistory(),
  routes,
})

router.beforeResolve((to) => {
  if (to.meta.title) {
    document.title = to.meta.title as string
  }
})

export default router
