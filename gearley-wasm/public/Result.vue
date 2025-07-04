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
        <DefaultGrammar v-if="kind === 'DefaultGrammar'" :op="op" :content="content" :names="names" />
        <DefaultGrammarSize v-if="kind === 'DefaultGrammarSize'" :op="op" :content="content" />
        <Mapping v-if="kind == 'mapping' && op == 'to_external'" :op="op" :content="content" />
        <BitSubMatrix v-if="kind === 'BitSubMatrix'" :content="content" :names="names" />
        <Vec v-if="op === 'medial_sort_and_remove_unary_medial_items'" :content="content" :names="names" />
        <Scan v-if="kind === 'Scan'" :content="content" :names="names" />
    </div>
</template>

<script>
import Cfg from 'components/Cfg.vue'
import Vec from 'components/Vec.vue'
import BitSubMatrix from 'components/BitSubMatrix.vue'
import Scan from 'components/Scan.vue'
import DefaultGrammar from 'components/DefaultGrammar.vue'
import DefaultGrammarSize from 'components/DefaultGrammarSize.vue'
import Mapping from 'components/Mapping.vue'

export default {
    components: {
        Cfg,
        Vec,
        BitSubMatrix,
        Scan,
        Mapping,
        DefaultGrammar,
        DefaultGrammarSize,
    },
    props: ['result'],
    data() {
        return {
            raw: false,
        }
    },
    methods: {
        toggleRaw() {
            this.raw = !this.raw
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
                var BinaryHeap = function(x) { return x };
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
        rules() {
            let cfg = this.logs.find(([op, kind, content]) => { return op === 'sort_rules_by_lhs' })
            cfg = cfg && cfg[2]
            if (!cfg) {
                return []
            }
            return cfg.rules
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
