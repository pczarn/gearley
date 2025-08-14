<template>
    <ul>
        <template v-for="(item, index) of props.content">
            <li>
                origin={{ item.origin }}, <Symbol :sym="parseStore.rules[item.dot].lhs" /> ::= <Symbol v-for="(sym, i) of parseStore.rules[item.dot].rhs" :sym="sym" :separate="i !== 0" />
            </li>
        </template>
    </ul>
</template>

<script setup>
import Symbol from './Symbol.vue'
import { useParse } from '@/stores/parse'

const props = defineProps(['op', 'content'])
const parseStore = useParse()
</script>

<style>
</style>
