<template>
    <Tabs v-model:value="tabs.tab" scrollable class="my-tabs">
        <TabList class="my-tab-list">
            <Tab v-for="(_list, op) of parseStore.logs" :key="op" :value="op"> {{ op }} </Tab>
        </TabList>
        <TabPanels class="panel-box">
            <TabPanel v-for="(list, key, index) of parseStore.logs" :key="key" :value="key" ref="printable">
                <Header v-for="([op, kind, content, logs], indexJ) of list" :title="op" :id="op" :help-button="indexJ === 0" :print-button="indexJ === 0" @print="print(index)" @help="showHelp">
                    <component :is="children[kind]" :op="op" :content="content" />
                    <Header title="logs" :level="2" :defaultCollapse="true">
                        <pre class="logs">
                            {{ logs }}
                        </pre>
                    </Header>
                </Header>
            </TabPanel>
        </TabPanels>
    </Tabs>
    <Dialog v-model:visible="helpVisible" modal header="Help" :style="{ width: '30rem' }">
        (TODO)
    </Dialog>
</template>

<script setup>
import { ref } from 'vue'

import { useVueToPrint } from "vue-to-print";

import Cfg from '@/components/Cfg.vue'
import Vec from '@/components/Vec.vue'
import BitSubMatrix from '@/components/BitSubMatrix.vue'
import Scan from '@/components/Scan.vue'
import DefaultGrammar from '@/components/DefaultGrammar.vue'
import DefaultGrammarSize from '@/components/DefaultGrammarSize.vue'
import Mapping from '@/components/Mapping.vue'
import Header from '@/components/Header.vue'
import SymInfo from '@/components/SymInfo.vue'
import SymbolBitSet from '@/components/SymbolBitSet.vue'
import Complete from '@/components/Complete.vue'
import Item from '@/components/Item.vue'

import Dialog from 'primevue/dialog'
import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';
import { useTabs } from '@/stores/tabs'
import { useParse } from '@/stores/parse'

const parseStore = useParse()
const tabs = useTabs()

const helpVisible = ref(false)

const children = {
    Cfg,
    Vec,
    BitSubMatrix,
    Scan,
    DefaultGrammar,
    DefaultGrammarSize,
    mapping: Mapping,
    SymInfo,
    SymbolBitSet,
    Complete,
    Item
}

const printable = ref([])

function print(key) {
    console.log('print')
    const { handlePrint } = useVueToPrint({
        content: printable.value[key],
        documentTitle: "Gearley Tab",
    })
    handlePrint()
}

function showHelp() {
    console.log('help')
    helpVisible.value = true
}
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
