<template>
    <Header :level="2" title="predicted symbols">
        <ul>
            <template v-for="(ch, index) of content.row.trim()">
                <li v-if="ch === '1'">
                    <Symbol :sym="{ 'n': index + 1 }" />
                </li>
            </template>
        </ul>
    </Header>
</template>

<script setup>
import Header from './Header.vue'
import Symbol from './Symbol.vue'
// import { useParse } from '@/stores/parse'

const props = defineProps(['op', 'content'])
</script>

<style>
</style>
