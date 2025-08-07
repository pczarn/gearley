<template>
    <h1>{{ props.op }}</h1>
    <h2>Mapping</h2>
    <ul>
        <ol start="0">
            <li v-for="sym in props.content">
                <template v-if="sym.sym && sym.name">
                    {{ sym.name ? sym.name.name : 'None' }}
                    ({{ sym.sym && sym.sym.n ? sym.sym.n : '?' }})
                </template>
                <Symbol v-else-if="sym.n" :sym="sym" />
            </li>
        </ol>
    </ul>
</template>

<script setup>
import Symbol from './Symbol.vue';

const props = defineProps(['op', 'content'])
</script>
