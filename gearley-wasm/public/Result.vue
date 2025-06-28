<template>
    <div>
        logs:
        <label class="switch">
            <input type="checkbox" @click="toggleRaw">
            <div class="slider round"></div>
        </label>
        <p>
            <span v-if="raw">Raw output</span>
            <span v-else>Rich output</span>
        </p>
        <div v-if="raw" v-for="[op, kind, content] in logs">
            {{ op }}, {{ kind }};
        </div>
        <pre v-if="raw">{{ rawLogs }}</pre>
    </div>
    <div v-if="!raw">
        bocage: {{ bocage }}
    </div>
    <div v-if="!raw" v-for="[op, kind, content] in logs">
        <Cfg v-if="kind == 'Cfg'" :content="content" :op="op" />
        <div v-if="kind == 'DefaultGrammar'">
            <h1>{{ op }}</h1>
            <h2>DefaultGrammar</h2>
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
            <h2>prediction_matrix</h2>
            <table>
                <thead>
                    <th></th>
                    <th v-for="index in content.prediction_matrix.row_bits">{{ index - 1 }}</th>
                </thead>
                <tbody>
                    <tr v-for="(val, index) in content.prediction_matrix.bit_vec.storage.split(' ')">
                        <td>{{ index }}</td>
                        <td v-for="(bit, index2) in val.split('').slice(0, size.syms)">
                            <span :class="{ one: bit === '1', diagonal: index === index2 }">
                                {{ bit }}
                            </span>
                        </td>
                    </tr>
                </tbody>
            </table>
            <h2>lr_sets</h2>
            <table>
                <thead>
                    <th></th>
                    <th v-for="index in content.lr_sets.row_bits">{{ index - 1 }}</th>
                </thead>
                <tbody>
                    <tr v-for="(val, index) in content.lr_sets.bit_vec.storage.trim().split(' ')">
                        <td>{{ index }}</td>
                        <td v-for="(bit, index2) in val.split('')">
                            <span :class="{ one: bit === '1', diagonal: index === index2 }">{{ bit }}</span>
                        </td>
                    </tr>
                </tbody>
            </table>
            <h2>completion_matrix</h2>
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
        </div>
        <div v-if="kind == 'DefaultGrammarSize'">
            <h1>{{ op }}</h1>
            <h2>DefaultGrammarSize</h2>
            <ul>
                <li v-for="(val, key) in content">{{ key }}: {{ val }}</li>
            </ul>
        </div>
        <div v-if="kind == 'mapping' && op == 'to_external'">
            <h1>{{ op }}</h1>
            <h2>Mapping</h2>
            <ul>
                <ol start="0">
                    <li v-for="sym_with_name in content">{{ sym_with_name.name ? sym_with_name.name.name : 'None' }} ({{ sym_with_name.sym.n }})</li>
                </ol>
            </ul>
        </div>
        <BitSubMatrix v-if="kind === 'BitSubMatrix'" :content="content" :names="names" />
        <Vec v-if="op === 'medial_sort_and_remove_unary_medial_items'" :content="content" :names="names" />
    </div>
</template>

<script>
import Cfg from 'components/Cfg.vue'
import Vec from 'components/Vec.vue'
import BitSubMatrix from 'components/BitSubMatrix.vue'

export default {
    components: {
        Cfg,
        Vec,
        BitSubMatrix,
    },
    data() {
        return {
            input_editor: null,
            grammar_editor: null,
            result: '',
            raw: false,
        }
    },
    mounted() {
        if (typeof window.log_error === 'undefined') {
            window.log_error = function(text) {
                this.result = text;
            }
        }
        this.input_editor = ace.edit("input_editor");
        this.input_editor.setTheme("ace/theme/monokai");
        this.input_editor.session.setMode("ace/mode/javascript");
        this.input_editor.getSession().on('change', this.update);
        this.grammar_editor = ace.edit("grammar_editor");
        this.grammar_editor.setTheme("ace/theme/monokai");
        this.grammar_editor.session.setMode("ace/mode/javascript");
        this.grammar_editor.getSession().on('change', this.update);
        this.update()
    },
    methods: {
        toggleRaw() {
            this.raw = !this.raw
        },
        update() {
            const input = this.input_editor.getValue();
            const grammar = this.grammar_editor.getValue();

            try {
                this.result = window._parse(input, grammar);
            } catch (e) {
                this.result = e.message;
            }
            console.log(this.result.length)
        },
        name_of(content, sym) {
            let name = content.sym_source.names[sym.n - 1]
            if (name === undefined || name == null) {
                return `g(${sym.n - 1})`
            } else {
                return `${name.name} (${sym.n - 1})`
            } 
        },
        eachCons(array, num) {
            return Array.from({ length: array.length - num + 1 },
                              (_, i) => array.slice(i, i + num))
        }
    },
    computed: {
        rawLogs() {
            return this.result
            const maybeError = this.result.split('\n', 1)[0]
            if (maybeError === 'unreachable') {
                return this.result
            } else {
                return maybeError
            }
        },
        logs() {
            function wrappedEval(textExpression, contextData){
                let fn = Function(`"use strict";
                var Some = function(x) { return x };
                var None = null;
                var NoOp = {"root_node": "no_op"};
                var Root = function(x) { return x };
                var NodeHandle = function(x) { return x };
                return (${textExpression})`)
                return fn.bind(contextData)();
            }
            const matches = this.result.matchAll(/^\[TRACE\] - ([\w]+): (\w+) (.+)$/gm)
            const logs = []
            for (const [all, path, kind, content] of matches) {
                function replacer(s, captured) {
                    return `"${captured}":`
                }
                const replaced = content
                    .replace(/\w+ {/g, "{")
                    .replace(/(\w+):/g, replacer)
                const evaled = wrappedEval(replaced, {})
                logs.push([path, kind, evaled])
            }
            return logs
        },
        names() {
            let mapping = this.logs.find(([op, kind, content]) => { return kind == 'mapping' && op == 'to_external' })
            mapping = mapping && mapping[2]
            if (!mapping) {
                return []
            }
            return mapping.map(sym_with_name => sym_with_name.name && sym_with_name.name.name)
        },
        size() {
            const result = this.logs && this.logs.find(([op, kind, content]) => kind == 'DefaultGrammarSize')
            return result && result[2]
        },
        bocage() {
            const result = this.logs && this.logs.find(([op, kind, content]) => kind == 'Bocage')
            return result && result[2]
        },
        finishedNode() {
            const result = this.logs && this.logs.find(([op, kind, content]) => kind == 'NodeHandle')
            return result && result[2].handle
        },
        graph() {
            if(typeof this.bocage !== 'Object') {
                return null
            }
            if(!this.finishedNode) {
                return null
            }

        }
    }
}
</script>

<style>
.switch {
  position: relative;
  display: inline-block;
  width: 60px;
  height: 34px;
}

.switch input {
  display: none;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  -webkit-transition: 0.4s;
  transition: 0.4s;
}

.slider:before {
  position: absolute;
  content: "";
  height: 26px;
  width: 26px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  -webkit-transition: 0.4s;
  transition: 0.4s;
}

input:checked + .slider {
  background-color: #101010;
}

input:focus + .slider {
  box-shadow: 0 0 1px #101010;
}

input:checked + .slider:before {
  -webkit-transform: translateX(26px);
  -ms-transform: translateX(26px);
  transform: translateX(26px);
}

.slider.round {
  border-radius: 34px;
}

.slider.round:before {
  border-radius: 50%;
}

span.one {
    background-color: green;
}

span.diagonal {
    color: white;
}
</style>
