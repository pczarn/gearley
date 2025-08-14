<template>
    <p>origin={{ props.content.origin }}, <Symbol :sym="rule.lhs" /> ::= <Symbol v-for="(sym, i) of rule.rhs" :sym="sym" :separate="i !== 0" /></p>
</template>

<script setup>
import Symbol from './Symbol.vue'
import { useParse } from '@/stores/parse'
import { computed } from 'vue'

const props = defineProps(['content', 'op'])
const parseStore = useParse()

const rule = computed(() => {
    return parseStore.rules[props.content.dot]
})
</script>

<style>
</style>
