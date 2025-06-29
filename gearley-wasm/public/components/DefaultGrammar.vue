<template>
    <Header id="d_g_op" :title="op">
        <Header id="d_g" title="DefaultGrammar" :level="2">
            <table>
                <tbody>
                    <tr v-for="(val, key) in content">
                        <template v-if="typeof val.n !== 'undefined'">
                            <td>{{ key }}</td>
                            <td>{{ val.n }}</td>
                        </template>
                        <template v-if="typeof val === 'boolean'">
                            <td>{{ key }}</td>
                            <td>{{ val }}</td>
                        </template>
                    </tr>
                    <tr><td>dot_before_eof</td><td>{{ content.dot_before_eof }}</td></tr>
                </tbody>
            </table>
        </Header>
        <Header id="d_g_p_m" title="prediction_matrix" :level="2">
            <table>
                <thead>
                    <th></th>
                    <th v-for="index in content.prediction_matrix.row_bits">{{ index - 1 }}</th>
                </thead>
                <tbody>
                    <tr v-for="(val, index) in content.prediction_matrix.bit_vec.storage.split(' ')">
                        <td>{{ index }}</td>
                        <td v-for="(bit, index2) in val.split('').slice(0, content.size.syms)">
                            <span :class="{ one: bit === '1', diagonal: index === index2 }">
                                {{ bit }}
                            </span>
                        </td>
                    </tr>
                </tbody>
            </table>
        </Header>
        <Header id="ll_lr" title="ll, lr_sets" :level="2">
            <table>
                <thead>
                    <th></th>
                    <th v-for="index in content.lr_sets.row_bits">{{ index - 1 }}</th>
                </thead>
                <tbody>
                    <tr v-for="(val, index) in content.lr_sets.bit_vec.storage.trim().split(' ')">
                        <td>{{ index % 2 === 0 ? `LL (${index / 2})` : `LR (${(index - 1) / 2})` }}</td>
                        <td v-for="(bit, index2) in val.split('').slice(0, content.size.syms + content.size.gensyms)">
                            <span :class="{ one: bit === '1', diagonal: index % 2 === 0 && (index / 2) === index2 }">{{ bit }}</span>
                        </td>
                    </tr>
                </tbody>
            </table>
        </Header>
        <Header id="c_m" title="completion_matrix" :level="2">
            <table>
                <tbody>
                    <tr v-for="([i1, i2], index) in eachCons(content.completions.indices, 2)">
                        <td>{{ index }}</td>
                        <td v-for="prediction_transition in content.completions.chart.slice(i1, i2)">
                            symbol: {{ prediction_transition.symbol.n }}, dot: {{ prediction_transition.dot }}, is_unary: {{ prediction_transition.is_unary }}
                        </td>
                    </tr>
                </tbody>
            </table>
        </Header>
        <Header id="d_g_g_c" title="gen_completions" :level="2">
            <ol :start="content.size.syms">
                <li v-for="gen in content.gen_completions">
                    <Symbol :sym="gen.symbol" :names="names" />
                    <Rule :dot="gen.dot" />
                </li>
            </ol>
        </Header>
    </Header>
</template>

<script>
import { getTransitionRawChildren } from 'vue';
import Header from './Header.vue';

export default {
    props: ['op', 'content', 'names'],
    components: {
        Header,
    },
    methods: {
        eachCons(array, num) {
            return Array.from({ length: array.length - num + 1 },
                              (_, i) => array.slice(i, i + num))
        }
    }
}
</script>
