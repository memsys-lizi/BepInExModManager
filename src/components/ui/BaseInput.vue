<script setup lang="ts">
defineProps<{
  placeholder?: string
  disabled?: boolean
  readonly?: boolean
}>()

const model = defineModel<string>()
</script>

<template>
  <input
    class="input"
    v-model="model"
    :placeholder="placeholder"
    :disabled="disabled"
    :readonly="readonly"
  />
</template>

<style scoped>
.input {
  width: 100%;
  padding: 5px 10px;
  border-radius: var(--radius-sm);
  font-size: var(--text-base);
}

.input:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
