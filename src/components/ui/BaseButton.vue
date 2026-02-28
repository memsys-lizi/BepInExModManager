<script setup lang="ts">
defineProps<{
  variant?: 'primary' | 'ghost' | 'danger'
  size?: 'sm' | 'md'
  disabled?: boolean
  loading?: boolean
}>()
</script>

<template>
  <button
    class="btn"
    :class="[
      `btn--${variant ?? 'primary'}`,
      `btn--${size ?? 'md'}`,
      { 'btn--loading': loading },
    ]"
    :disabled="disabled || loading"
  >
    <slot />
  </button>
</template>

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-weight: 500;
  border-radius: var(--radius-sm);
  border: 1px solid transparent;
  transition: background var(--transition-fast), border-color var(--transition-fast), color var(--transition-fast);
  cursor: pointer;
  white-space: nowrap;
}

.btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* Size */
.btn--sm { padding: 3px 10px; font-size: var(--text-sm); }
.btn--md { padding: 5px 14px; font-size: var(--text-base); }

/* Primary：用 CSS 变量让深色/浅色都正确 */
.btn--primary {
  background: var(--color-accent);
  color: var(--color-bg);          /* 与背景对比：深色模式=深色背景色，浅色模式=浅色背景色 */
  border-color: var(--color-accent);
}
.btn--primary:hover:not(:disabled) {
  background: var(--color-text-secondary);
  border-color: var(--color-text-secondary);
  color: var(--color-bg);
}

/* Ghost */
.btn--ghost {
  background: transparent;
  color: var(--color-text-secondary);
  border-color: var(--color-border);
}
.btn--ghost:hover:not(:disabled) {
  background: var(--color-surface-2);
  color: var(--color-text-primary);
  border-color: var(--color-border-2);
}

/* Danger */
.btn--danger {
  background: transparent;
  color: var(--color-danger);
  border-color: var(--color-danger);
}
.btn--danger:hover:not(:disabled) {
  background: var(--color-danger);
  color: #fff;
}

.btn--loading { opacity: 0.6; cursor: wait; }
</style>
