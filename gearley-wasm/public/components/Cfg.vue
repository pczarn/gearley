<template>
    <Header title="Grammar Info" :level="2" :id="'cfg' + op">
        <table>
            <tbody>
                <tr>
                    <td>After</td>
                    <td>{{ op }}</td>
                </tr>
                <tr>
                    <td>Num syms</td>
                    <td>{{ content.sym_source.next_symbol.n - 1 }}</td>
                </tr>
                <tr>
                    <td>Num rules</td>
                    <td>{{ content.rules.length }}</td>
                </tr>
            </tbody>
        </table>
    </Header>
    <Header title="Rules" :level="2" :id="'rules' + op">
        <table>
            <thead>
                <th>LHS</th>
                <th>RHS</th>
            </thead>
            <tbody>
                <tr v-for="rule in content.rules">
                    <td>{{ name_of(rule.lhs) }}</td>
                    <td>
                        <span v-for="sym in rule.rhs">
                            {{ name_of(sym) }},
                        </span>
                    </td>
                </tr>
            </tbody>
        </table>
    </Header>
</template>

<script>
import Header from './Header.vue'

export default {
    props: ['op', 'content', 'names'],
    components: {
        Header
    },
    methods: {
        name_of(sym) {
            let name = this.content.sym_source && this.content.sym_source.names[sym.n - 1]
            if (name === undefined || name == null) {
                return `g(${sym.n - 1})`
            } else {
                return `${name.name} (${sym.n - 1})`
            } 
        },
    }
}

</script>
