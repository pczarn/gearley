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
            <Column field="lhs" header="LHS">
                <template #body="{ data }">
                    <Symbol :sym="data.lhs" />
                </template>
            </Column>
            <Column field="rhs" header="RHS">
                <template #body="{ data }">
                    <Symbol v-for="sym in data.rhs" :sym="sym" />
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
    return props.content.rules
})
</script>
