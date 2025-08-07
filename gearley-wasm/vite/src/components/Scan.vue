<template>
    <p><Symbol :sym="props.content.symbol" /> node = {{ node }}</p>
</template>

<script setup>
import Symbol from './Symbol.vue'
import { useParse } from '@/stores/parse'
import { computed } from 'vue'

const props = defineProps(['content', 'op'])
const parseStore = useParse()

const name = computed(() => {
    return parseStore.names[props.content.symbol.n - 1]
})

const node = computed(() => {
    return props.content.node
})
</script>

<style>
</style>
