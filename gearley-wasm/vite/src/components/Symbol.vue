<template>
    <a v-if="props.sym.n && parseStore.names" href="#" @click.prevent="goToSym(props.sym.n - 1)">{{ parseStore.names[props.sym.n - 1] }} ({{ props.sym.n - 1 }})</a>
</template>

<script setup>
import { useTabs } from '@/stores/tabs'
import { useParse } from '@/stores/parse'

const props = defineProps(['sym', 'names'])
// const emit = defineEmits(['tab'])
const tabs = useTabs()
const parseStore = useParse()

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
