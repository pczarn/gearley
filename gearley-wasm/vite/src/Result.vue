<template>
    <Tabs v-model:value="tabs.tab" scrollable class="my-tabs">
        <TabList class="my-tab-list">
            <Tab v-for="[op, _kind, _content, _lines] in logs[0]" :key="op" :value="op"> {{ op }} </Tab>
            <Tab v-if="logs[1].length > 0" value="recognizer">recognizer</Tab>
        </TabList>
        <TabPanels class="panel-box">
            <TabPanel v-for="[op, kind, content, lines] in logs[0]" :key="op" :value="op">
                <Header :title="op" :id="op">
                    <component :is="children[kind]" :op="op" :content="content" :names="names" />
                    <Header title="logs" :level="2" :defaultCollapse="true">
                        <pre class="logs">
                            {{ lines }}
                        </pre>
                    </Header>
                </Header>
            </TabPanel>
            <TabPanel v-if="logs[1].length > 0" value="recognizer">
                <Header v-for="[op, kind, content, lines] in logs[1]" :key="op" :title="op" :id="op">
                    <component :is="children[kind]" :op="op" :content="content" :names="names" />
                    <Header title="logs" :level="2" :defaultCollapse="true">
                        <pre class="logs">
                            {{ lines }}
                        </pre>
                    </Header>
                </Header>
            </TabPanel>
            <!-- <TabPanel v-if="logs[2].length > 0" value="recognizer">
                <Header v-for="[op, kind, content, lines] in logs[1]" :key="op" :title="op" :id="op">
                    <component :is="children[kind]" :op="op" :content="content" :names="names" />
                    <Header title="logs" :level="2" :defaultCollapse="true">
                        <pre class="logs">
                            {{ lines }}
                        </pre>
                    </Header>
                </Header>
            </TabPanel> -->
        </TabPanels>
    </Tabs>
</template>

<script setup>
import Cfg from '@/components/Cfg.vue'
import Vec from '@/components/Vec.vue'
import BitSubMatrix from '@/components/BitSubMatrix.vue'
import Scan from '@/components/Scan.vue'
import DefaultGrammar from '@/components/DefaultGrammar.vue'
import DefaultGrammarSize from '@/components/DefaultGrammarSize.vue'
import Mapping from '@/components/Mapping.vue'
import Header from '@/components/Header.vue'
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';
import { computed } from 'vue'
import { useTabs } from '@/stores/tabs'

const tabs = useTabs()

const children = {
    Cfg,
    Vec,
    BitSubMatrix,
    Scan,
    DefaultGrammar,
    DefaultGrammarSize,
    Mapping,
}

const props = defineProps(['result'])
const limit = 1000

const rawLogs = computed(() => {
    return this.result
    const maybeError = this.result.split('\n', 1)[0]
    if (maybeError === 'unreachable') {
        return this.result
    } else {
        return maybeError
    }
})

const logs = computed(() => {
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
    let lines = props.result.split("\n")
    if (lines.length > limit.value) {
        lines = lines.slice(0, limit.value)
    }
    let logs = []
    let recce = []
    let symbols = []
    for (const line of lines) {
        const myMatch = line.match(/^\[TRACE\] - ([\w]+): (\w+) (.+)$/)
        if (myMatch === null) {
            if (logs.length > 0) {
                logs[logs.length - 1][3] += line
            }
            continue
        }
        let [all, path, kind, content] = myMatch
        function replacer(s, captured) {
            return `"${captured}":`
        }
        const replaced = content
            .replace(/\w+ {/g, "{")
            .replace(/(\w+):/g, replacer)
        const evaled = wrappedEval(replaced, {})
        if (path === 'predicted' || path === 'scan' || path === 'complete' || path === 'medial_sort_and_remove_unary_medial_items' || path === 'was_predicted_will_be_useful') {
            recce.push([path, kind, evaled, line])
        } else {
            logs.push([path, kind, evaled, line])
        }
    }
    return [logs, recce]
})

const names = computed(() => {
    let mapping = logs.value[0].find(([op, kind, content]) => { return kind == 'mapping' && op == 'to_external' })
    mapping = mapping && mapping[2]
    if (!mapping) {
        return []
    }
    return mapping.map(sym_with_name => sym_with_name.name && sym_with_name.name.name)
})

const rules = computed(() => {
    let cfg = logs.value[0].find(([op, kind, content]) => { return op === 'sort_rules_by_lhs' })
    cfg = cfg && cfg[2]
    if (!cfg) {
        return []
    }
    return cfg.rules
})

const size = computed(() => {
    const result = logs.value && logs.value[0] && logs.value[0].find(([op, kind, content]) => kind == 'DefaultGrammarSize')
    return result && result[2]
})

const bocage = computed(() => {
    const result = logs.value && logs.value[0] && logs.value[0].find(([op, kind, content]) => kind == 'Bocage')
    return result && result[2]
})

const finishedNode = computed(() => {
    const result = logs.value && logs.value[0] && logs.value[0].find(([op, kind, content]) => kind == 'NodeHandle')
    return result && result[2].handle
})

const graph = (() => {
    if(typeof bocage.value !== 'Object') {
        return null
    }
    if(!this.finishedNode) {
        return null
    }

})
</script>

<style>
.my-tabs {
    height: 100%;
}

/* .my-tab-list {
    flex: 1 0;
} */

.panel-box {
    overflow: auto;
    flex: 1 1;
}

.logs {
    width: 200%;
    text-wrap: auto;
}
</style>
