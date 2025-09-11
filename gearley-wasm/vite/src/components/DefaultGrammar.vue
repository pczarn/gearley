<template>
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
                <tr>
                    <th></th>
                    <th v-for="index in content.prediction_matrix.row_bits">{{ index - 1 }}</th>
                </tr>
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
                <th style="min-width: 80px;"></th>
                <th v-for="index in content.lr_sets.row_bits">{{ index - 1 }}</th>
            </thead>
            <tbody>
                <tr v-for="(val, index) in content.lr_sets.bit_vec.storage.trim().split(' ')">
                    <td>{{ index % 2 === 0 ? `LL (${index / 2})` : `LR (${(index - 1) / 2})` }}</td>
                    <td v-for="(bit, index2) in val.split('').slice(0, content.size.syms + content.size.gensyms)">
                        <template v-if="bit === '0' && !diagonal(index, index2)">
                            0
                        </template>
                        <span v-else :class="{ one: bit === '1', diagonal: diagonal(index, index2) }">{{ bit }}</span>
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
                <Symbol v-if="gen[0] !== null" :sym="gen[0].symbol" />
                <span v-else>null</span>
                ,
                <Symbol v-if="gen[1] !== null" :sym="gen[1].symbol" />
                <span v-else>null</span>
                <!-- <Rule :dot="gen.dot" /> -->
            </li>
        </ol>
    </Header>
</template>

<script>
import { getTransitionRawChildren } from 'vue';
import Header from './Header.vue';
import Symbol from './Symbol.vue';

export default {
    props: ['op', 'content'],
    components: {
        Header,
        Symbol,
    },
    methods: {
        eachCons(array, num) {
            return Array.from({ length: array.length - num + 1 },
                              (_, i) => array.slice(i, i + num))
        },
        diagonal(i, j) {
            return i % 2 === 0 && (i / 2) === j
        }
    }
}
</script>
