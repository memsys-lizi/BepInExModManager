<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { ChevronDown, Check } from 'lucide-vue-next'

interface Option {
  value: string
  label: string
}

const props = defineProps<{
  modelValue: string
  options: Option[]
  placeholder?: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>()

const open = ref(false)
const rootEl = ref<HTMLElement | null>(null)

const selectedLabel = () =>
  props.options.find(o => o.value === props.modelValue)?.label ?? props.placeholder ?? ''

function select(value: string) {
  emit('update:modelValue', value)
  open.value = false
}

function onOutsideClick(e: MouseEvent) {
  if (rootEl.value && !rootEl.value.contains(e.target as Node)) {
    open.value = false
  }
}

onMounted(() => document.addEventListener('mousedown', onOutsideClick))
onUnmounted(() => document.removeEventListener('mousedown', onOutsideClick))
</script>

<template>
  <div ref="rootEl" class="bselect" :class="{ 'bselect--open': open }">
    <button class="bselect__trigger" @click="open = !open">
      <span class="bselect__value">{{ selectedLabel() }}</span>
      <ChevronDown :size="12" class="bselect__arrow" />
    </button>

    <Transition name="dropdown">
      <div v-if="open" class="bselect__menu">
        <button
          v-for="opt in options"
          :key="opt.value"
          class="bselect__option"
          :class="{ 'bselect__option--active': opt.value === modelValue }"
          @click="select(opt.value)"
        >
          <span>{{ opt.label }}</span>
          <Check v-if="opt.value === modelValue" :size="11" class="bselect__check" />
        </button>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.bselect {
  position: relative;
  display: inline-block;
}

.bselect__trigger {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
  background: var(--color-surface-2);
  color: var(--color-text-secondary);
  font-size: var(--text-xs);
  cursor: pointer;
  transition: border-color var(--transition-fast), background var(--transition-fast),
              color var(--transition-fast);
  white-space: nowrap;
}

.bselect__trigger:hover,
.bselect--open .bselect__trigger {
  border-color: var(--color-border-2);
  color: var(--color-text-primary);
  background: var(--color-surface);
}

.bselect__arrow {
  flex-shrink: 0;
  color: var(--color-text-muted);
  transition: transform var(--transition-fast);
}
.bselect--open .bselect__arrow {
  transform: rotate(180deg);
}

.bselect__menu {
  position: absolute;
  bottom: calc(100% + 4px);
  left: 0;
  min-width: 100%;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.18);
  overflow: hidden;
  z-index: 200;
}

.bselect__option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  width: 100%;
  padding: 6px 12px;
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  background: transparent;
  cursor: pointer;
  text-align: left;
  transition: background var(--transition-fast), color var(--transition-fast);
  white-space: nowrap;
}

.bselect__option:hover {
  background: var(--color-surface-2);
  color: var(--color-text-primary);
}

.bselect__option--active {
  color: var(--color-text-primary);
  font-weight: 500;
}

.bselect__check {
  flex-shrink: 0;
  color: var(--color-text-primary);
}

/* Dropdown 动画 */
.dropdown-enter-active,
.dropdown-leave-active {
  transition: opacity 120ms ease, transform 120ms ease;
}
.dropdown-enter-from,
.dropdown-leave-to {
  opacity: 0;
  transform: translateY(4px);
}
</style>
