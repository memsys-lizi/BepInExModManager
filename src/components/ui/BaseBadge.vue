<script setup lang="ts">
defineProps<{
  variant?: 'default' | 'success' | 'danger' | 'warning' | 'muted'
}>()
</script>

<template>
  <span class="badge" :class="`badge--${variant ?? 'default'}`">
    <slot />
  </span>
</template>

<style scoped>
.badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 1px 7px;
  border-radius: var(--radius-sm);
  font-size: var(--text-xs);
  font-weight: 500;
  line-height: 18px;
  border: 1px solid transparent;
}

.badge--default { background: var(--color-accent-dim); color: var(--color-text-primary); border-color: var(--color-border-2); }
.badge--success { background: var(--color-success-bg); color: var(--color-success); border-color: var(--color-success); }
.badge--danger  { background: var(--color-danger-bg);  color: var(--color-danger);  border-color: var(--color-danger); }
.badge--warning { background: var(--color-warning-bg); color: var(--color-warning); border-color: var(--color-warning); }
.badge--muted   { background: transparent; color: var(--color-text-muted); border-color: var(--color-border); }
</style>
