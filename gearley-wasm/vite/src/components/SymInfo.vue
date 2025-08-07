<template>
    <span :id="'sym' + (content.internal.n - 1)">{{ name }}</span>
    <div>
        external: {{ content.external.n - 1 }}
    </div>
    <Header title="Appears on LHS" :level="2" :id="'rules_lhs' + (content.internal.n - 1)">
        <table>
            <thead>
                <th>LHS</th>
                <th>RHS</th>
            </thead>
            <tbody>
                <tr v-for="rule in content.rules_lhs">
                    <td>{{ name_of(rule.lhs) }}</td>
                    <td>{{ name_of(rule.rhs0) }}</td>
                    <td v-if="rule.rhs1 && rule.rhs1.n">{{ name_of(rule.rhs1) }}</td>
                </tr>
            </tbody>
        </table>
    </Header>
    <Header title="Appears on RHS" :level="2" :id="'rules_rhs' + (content.internal.n - 1)">
        <table>
            <thead>
                <th>LHS</th>
                <th>RHS</th>
            </thead>
            <tbody>
                <tr v-for="rule in content.rules_rhs">
                    <td>{{ name_of(rule.lhs) }}</td>
                    <td>{{ name_of(rule.rhs0) }}</td>
                    <td v-if="rule.rhs1 && rule.rhs1.n">{{ name_of(rule.rhs1) }}</td>
                </tr>
            </tbody>
        </table>
    </Header>
    <Header title="Predicts" :level="2" :id="'pred' + (content.internal.n - 1)">
        <ul>
            <li v-for="sym in content.predicts"><Symbol :sym="sym" /></li>
        </ul>
    </Header>
    <Header title="Predicted by" :level="2" :id="'pred_by' + (content.internal.n - 1)">
        <ul>
            <li v-for="sym in content.predicts_by"><Symbol :sym="sym" /></li>
        </ul>
    </Header>
</template>

<script setup>
import { computed } from 'vue'
import { useParse } from '@/stores/parse'
import Header from './Header.vue'
import Symbol from './Symbol.vue'

const props = defineProps(['op', 'content'])
const parseStore = useParse()

const name = computed(() => {
    if (props.content.name && props.content.name.name) {
        return `${props.content.name.name} (${props.content.internal.n - 1})`
    } else {
        return `(unnamed) (${props.content.internal.n - 1})`
    }
})

function name_of(sym) {
    let name = parseStore.names && sym.n && parseStore.names[sym.n - 1]
    if (name === undefined || name === null) {
        return `(unnamed) (${sym.n - 1})`
    } else {
        return `${name} (${sym.n - 1})`
    } 
}
</script>
