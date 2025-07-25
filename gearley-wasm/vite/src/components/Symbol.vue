<template>
    <a v-if="sym.n" href="#" @click.prevent="goToSym(sym.n - 1)">{{ names[sym.n - 1] }} ({{ sym.n - 1 }})</a>
</template>

<script setup>
import { useTabs } from '@/stores/tabs'
const props = defineProps(['sym', 'names'])
// const emit = defineEmits(['tab'])
const tabs = useTabs()

function goToSym(id) {
    tabs.send('syms')
    // emit('tab', 'syms')
    nextTick(() => {
        const el = document.getElementById(id)
        if (el) {
            el.scrollIntoView({ behavior: 'smooth' })
        }
    })
}
</script>
