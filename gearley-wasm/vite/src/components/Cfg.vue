<template>
    <Header title="Grammar Info" :level="2" :id="'cfg' + op">
        <DataTable :value="cfgInfo">
            <Column field="op" header="After"></Column>
            <Column field="numSyms" header="Number of symbols"></Column>
            <Column field="numRules" header="Number of rules"></Column>
        </DataTable>
    </Header>
    <Header title="Rules" :level="2" :id="'rules' + op">
        <DataTable :value="rules">
            <Column field="dot" header="dot">
                <template #body="{ data }">
                    {{ data.dot }}
                </template>
            </Column>
            <Column field="lhs" header="LHS">
                <template #body="{ data }">
                    <Symbol :sym="data.lhs" :name="nameOf(data.lhs)" :active="active" />
                </template>
            </Column>
            <Column field="rhs" header="RHS">
                <template #body="{ data }">
                    <Symbol v-for="(sym, i) of data.rhs" :sym="sym" :name="nameOf(sym)" :active="active" :separate="i !== 0" />
                </template>
            </Column>
        </DataTable>
    </Header>
</template>

<script setup>
import { computed } from 'vue'

import DataTable from 'primevue/datatable';
import Column from 'primevue/column';

import Header from './Header.vue'
import Symbol from './Symbol.vue'

const props = defineProps(['op', 'content'])

const cfgInfo = computed(() => {
    return [{
        op: props.op,
        numSyms: props.content.sym_source.next_symbol.n - 1,
        numRules: props.content.rules.length,
    }]
})

const rules = computed(() => {
    return props.content.rules.map((val, index) => ({ dot: index, ...val }))
})

function nameOf(sym) {
    let name = props.content.sym_source && props.content.sym_source.names[sym.n - 1]
    if (name === undefined || name == null) {
        return `g(${sym.n - 1})`
    } else {
        return `${name.name} (${sym.n - 1})`
    } 
}

const active = computed(() => {
    return props.op === 'remap_symbols' || props.op === 'sort_rules_by_lhs'
})
</script>
