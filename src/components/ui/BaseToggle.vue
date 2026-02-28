<script setup lang="ts">
const model = defineModel<boolean>()

defineProps<{
  disabled?: boolean
}>()
</script>

<template>
  <button
    class="toggle"
    :class="{ 'toggle--on': model, 'toggle--disabled': disabled }"
    role="switch"
    :aria-checked="model"
    :disabled="disabled"
    @click="model = !model"
  >
    <span class="toggle__thumb" />
  </button>
</template>

<style scoped>
.toggle {
  position: relative;
  width: 32px;
  height: 18px;
  border-radius: 9px;
  /* 必须显式写 background，防止被全局 button { background: none } 覆盖 */
  background: var(--color-border-2) !important;
  border: 1px solid var(--color-border-2) !important;
  transition: background var(--transition-fast), border-color var(--transition-fast);
  flex-shrink: 0;
  padding: 0;
}

.toggle--on {
  background: var(--color-accent) !important;
  border-color: var(--color-accent) !important;
}

.toggle--disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.toggle__thumb {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  /* 未开启时：用 bg 颜色的对比色，深色模式=浅灰，浅色模式=中灰 */
  background: var(--color-text-secondary);
  transition: transform var(--transition-fast), background var(--transition-fast);
}

.toggle--on .toggle__thumb {
  transform: translateX(14px);
  /* 开启时 thumb 颜色与 accent 对比：使用背景色（深色=深色，浅色=浅色）*/
  background: var(--color-bg);
}
</style>
