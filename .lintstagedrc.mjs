/**
 * @filename: lint-staged.config.js
 * @type {import('lint-staged').Configuration}
 */
export default {
  '*.{ts,js,jsx,tsx,vue}': ['eslint --cache --fix', () => 'vue-tsc --noEmit'],
  '*.{css,scss,vue}': 'stylelint --cache --fix NODE_OPTIONS=--no-deprecation',
}
