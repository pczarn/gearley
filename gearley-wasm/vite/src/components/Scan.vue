<template>
    <p>{{ parseStore.names[props.content.symbol.n - 1] }}, {{ props.content.node }}</p>
</template>

<script setup>
import { useParse } from '@/stores/parse'

const props = defineProps(['content', 'op'])
const parseStore = useParse()
</script>

<style>
</style>
