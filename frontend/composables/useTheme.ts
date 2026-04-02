import { ref } from 'vue'

export const theme = ref<'light'|'dark'>('light')
export const accent = ref('#5c6bc0')

export function toggleTheme() {
  theme.value = theme.value === 'light' ? 'dark' : 'light'
  document.documentElement.setAttribute('data-theme', theme.value)
}