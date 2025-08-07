<template>
    <a v-if="props.sym.n && parseStore.names" href="#" @click.prevent="goToSym(props.sym.n - 1)">
        <template v-if="name">{{ name }}</template>
        <template v-else>(unnamed)</template>
        ({{ props.sym.n - 1 }})
    </a>
</template>

<script setup>
import { nextTick, computed } from 'vue'
import { useTabs } from '@/stores/tabs'
import { useParse } from '@/stores/parse'

const props = defineProps(['sym'])
const tabs = useTabs()
const parseStore = useParse()

const name = computed(() => {
    if (!props.sym.n || !parseStore.names) {
        return null
    }
    return parseStore.names[props.sym.n - 1]
})

function goToSym(id) {
    tabs.send('symbols')
    // emit('tab', 'syms')
    nextTick(() => {
        const el = document.getElementById('sym' + id)
        if (el) {
            el.scrollIntoView({ behavior: 'smooth' })
        }
    })
}
</script>
