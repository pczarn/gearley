<template>
    <Header title="Gensyms">
        <ul>
            <template v-for="(bit, i) of props.content.bit_vec.storage.trim().split('')">
                <li v-if="bit === '1'">
                    <Symbol :sym="{ n: i + 1 }" />
                </li>
            </template>
        </ul>
    </Header>
</template>

<script setup>
import Header from './Header.vue';
import Symbol from './Symbol.vue';
import { useParse } from '@/stores/parse';

const props = defineProps(['op', 'content'])
const parseStore = useParse()
</script>
