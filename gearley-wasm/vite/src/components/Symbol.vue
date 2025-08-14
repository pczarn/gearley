<template>
    <span v-if="props.separate">, </span>
    <template v-if="props.active">
        <a v-if="props.sym.n && parseStore.names" href="#" @click.prevent="goToSym(props.sym.n - 1)" class="name">
            {{ nameOrUnnamed }}
        </a>
    </template>
    <template v-else>
        <span v-if="name" class="name">{{ name }}</span>
    </template>
</template>

<script setup>
import { nextTick, computed } from 'vue'
import { useTabs } from '@/stores/tabs'
import { useParse } from '@/stores/parse'

const props = defineProps({
    sym: {
        type: Object,
        required: true
    },
    name: {
        type: String,
        default: null
    },
    active: {
        type: Boolean,
        default: true
    },
    separate: {
        type: Boolean,
        default: false
    }
})
const tabs = useTabs()
const parseStore = useParse()

const name = computed(() => {
    if (props.name !== null) {
        return props.name
    }
    if (!props.sym.n || !parseStore.names) {
        return null
    }
    return parseStore.names[props.sym.n - 1] + `(${props.sym.n - 1})`
})

const nameOrUnnamed = computed(() => {
    return name.value ? name.value : '(unnamed)'
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

<style scoped>
.name {
    white-space: nowrap;
}
</style>