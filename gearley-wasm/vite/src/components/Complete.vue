<template>
    <p>@set_id={{ props.content.set_id }} <Symbol :sym="props.content.sym" /></p>
</template>

<script setup>
import Symbol from './Symbol.vue'
import { useParse } from '@/stores/parse'
// import { computed } from 'vue'

const props = defineProps(['content', 'op'])
const parseStore = useParse()
</script>

<style>
</style>
