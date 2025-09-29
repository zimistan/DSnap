import {
  defineConfig,
  presetIcons,
  presetWind3,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss'

export default defineConfig({
  presets: [
    presetWind3(),
    presetIcons({
      warn: true,
      extraProperties: {
        display: 'inline-block',
      },
    }),
  ],
  theme: {
    width: {
      base: 'var(--baseWidth)',
    },
    fontFamily: {
      dt: 'DingTalk',
      ds: 'DS-Digital',
    },
    colors: {
      // TODO: 这里定义主题色
    },
  },
  transformers: [
    transformerDirectives(),
    transformerVariantGroup(),
  ],
  shortcuts: [
    {
      center: 'flex items-center justify-center',
    },
  ],
})
