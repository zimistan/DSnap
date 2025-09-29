import type { PluginOption } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import UnoCSS from 'unocss/vite'
import Pages from 'vite-plugin-pages'
import Layouts from 'vite-plugin-vue-layouts'

export default function createVitePlugins() {
  return [
    vue(),
    vueJsx(),
    UnoCSS(),
    Pages({
      dirs: 'src/views',
      exclude: ['**/components/*.vue'],
      routeBlockLang: 'yaml',
    }),
    Layouts({
      layoutsDirs: 'src/layouts',
      defaultLayout: 'default/index',
      exclude: ['**/components/*.vue'],
    }),
  ] as (PluginOption | PluginOption[])[]
}
