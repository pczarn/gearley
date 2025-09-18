<template>
    <div id="container" class="container">
        <div class="bar">
            <div class="item">
                <label for="load-mode">
                    Mode
                </label>
                <select name="load-mode" id="load-mode" v-model="selectedMode">
                    <option v-for="val in loadModes" :value="val">{{ val }}</option>
                </select>
                <label for="load-example">
                    Example
                </label>
                <select name="load-example" id="load-example" v-model="selectedExample">
                    <option v-for="val in loadExamples" :value="val.id">{{ val.name }}</option>
                </select>
                <label for="limit">
                    Limit lines
                </label>
                <input type="number" v-model="parseStore.limit" class="num" />
            </div>
            <div class="item">
                <a href="https://github.com/pczarn/cfg/" target="_blank">
                    <img src="/images/github-128.png" alt="See the source on GitHub" class="github-icon" />
                    cfg v0.9
                </a>
                <a href="https://github.com/pczarn/gearley/" target="_blank">
                    <img src="/images/github-128.png" alt="See the source on GitHub" class="github-icon" />
                    gearley v0.0.1
                </a>
                <a href="https://github.com/pczarn/gearley/tree/a107af27fd538b2162da120c69eafede7cc19b1f/gearley-wasm/vite" target="_blank">
                    <img src="/images/github-128.png" alt="See the source on GitHub" class="github-icon" />
                    gearley-wasm v0.1.1
                </a>
                <Button label="Help" @click="helpVisible = true" />
            </div>
        </div>
        <div class="main-row">
            <div class="left-box">
                <div id="full-editor"></div>
            </div>
            <div class="right-box">
                <ProgressSpinner v-show="isLoading"></ProgressSpinner>
                <Result />
            </div>
        </div>
    </div>
    <Dialog v-model:visible="helpVisible" modal header="Help" class="help-dialog">
        <p>
            With the two dropdowns in the top bar, you can load any of the
            built-in examples. Each example is available only for the lexer
            selected as <i>mode</i>. The <i>basic</i> lexer accepts input as a
            space-separated list of terminal symbol names. The <i>advanced</i>
            lexer supports regular expressions and strings, allowing the grammar
            to run in scannerless mode.
        </p>

        <p>
            Line limits are used to prevent excessive processing and rendering
            for very large inputs. The default limit is 10,000 lines.
        </p>

        <p>
            On the left, you’ll see an editor. This editor provides the string
            to be parsed inside the HTML-like input tag (note that whitespace is
            not trimmed). Similarly, the grammar is defined inside the grammar
            tag. See the examples for more details.
        </p>

        <p>
            On the right, several tabs are available. Each tab corresponds to a
            grammar transformation or parsing step and displays the information
            logged at that stage. This makes it possible to explore the internal
            workings of Gearley.
        </p>

        <p>
            Finally, the top bar also contains links to our GitHub repositories.
        </p>
    </Dialog>
</template>

<script setup>
import Result from './Result.vue'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import ProgressSpinner from 'primevue/progressspinner'
import '@/assets/ace-builds'
import { ref, computed, onMounted, watch } from 'vue'
import { getGrammars, getExamples } from "@/assets/pkg/gearley_wasm.js";
import { useParse } from '@/stores/parse'
import Worker from '@/worker/parse?worker'

const parseStore = useParse()
const ParseWorker = new Worker()
ace.config.set('basePath', 'ace-builds/src-noconflict/')

let editor = null
const selectedMode = ref('advanced')
const typingTimer = ref(null)
const typing = ref(false)
const isLoading = ref(false)
const loadModes = [
    'advanced',
    'basic',
    'c-lexer'
]
const selectedExample = ref(null)
const allLoadExamples = ref([])
const helpVisible = ref(false)

const loadExamples = computed(() => {
    return allLoadExamples.value.filter((val) => val.mode === selectedMode.value)
})

onMounted(() => {
    if (typeof window.log_error === 'undefined') {
        window.log_error = function(text) {
            parseStore.setResult("Logged:\n" + text)
        }
    }

    editor = ace.edit("full-editor");
    editor.setTheme("ace/theme/monokai");
    editor.setOptions({
        fontSize: "11pt"
    });
    editor.session.setMode("ace/mode/javascript");
    function each_slice(ary, size) {
        let result = []
        for (var i = 0, l = ary.length; i < l; i += size){
            const [id, name, mode, content] = ary.slice(i, i + size)
            result.push({ id, name, mode, content });
        }
        return result
    };
    allLoadExamples.value = each_slice(getGrammars(), 4)
    selectedExample.value = allLoadExamples.value.find((val) => val.mode === selectedMode.value).id
    editor.getSession().on('change', update);
})

function update() {
    if (typing.value) {
        clearTimeout(typingTimer.value);
        typingTimer.value = setTimeout(() => {
            typing.value = false;
            update();
        }, 200);
    } else {
        typing.value = true;
        typingTimer.value = setTimeout(() => {
            typing.value = false;
        }, 200);
        const input = editor.getValue();
        processInput(input)
    }
}

function processInput(input) {
    const matchedInput = input.match(/<input>([\s\S]+)<\/input>/m)
    const matchedGrammar = input.match(/<grammar>([\s\S]+)<\/grammar>/m)
    if (!matchedInput || !matchedGrammar || typeof(matchedInput[1]) !== 'string' || typeof(matchedGrammar[1]) !== 'string') {
        parseStore.setResult('Error: could not find <input> or <grammar>')
        return
    }

    isLoading.value = true
    ParseWorker.postMessage([matchedInput[1], matchedGrammar[1], selectedMode.value])
}

ParseWorker.onmessage = (event) => {
    isLoading.value = false
    parseStore.setResult(event.data)
}

watch(selectedExample, () => {
    const info = loadExamples.value.find((val) => val.id === selectedExample.value)
    const samples = getExamples(info.id)
    const sfg = `<input>${samples[0]}</input>\n<grammar>${info.content}</grammar>`
    editor.setValue(sfg)
    processInput(sfg)
})
</script>

<style>
.container {
    display: flex;
    flex-direction: column;
    height: 100vh;
}

.bar {
    display: flex;
    align-items: center;

    font-size: 1.1em;

    flex: 0 0 40px;
    padding: 5px;
    border-bottom: gray solid 4px;
}

.bar .item select {
    margin: 7px 7px;
    padding: 3px 3px;
    width: 150px;
}

.bar .item a {
    display: flex;
    align-items: center;
    flex: 1 1;
}

.bar .item {
    flex: 0.5 1;
    margin: 0 7px;
    display: flex;
    align-items: center;
}

input.num {
    width: 90px;
}

#full-editor {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
}

.main-row {
    display: flex;
    flex: 1;
    overflow: auto;
}

.left-box {
    flex: 1;
    overflow: auto;
    padding: 20px;
    border-right: gray solid 2px;
    position: relative;
}

.right-box {
    flex: 1;
    background-color: #eee;
    overflow: auto;
    border-left: gray solid 2px;
    font-family: 'Courier New', Courier, monospace;
}

img.github-icon {
    width: 22px;
    height: 22px;
    margin: 5px 5px;
}

.help-dialog {
    width: 30rem;
    font-size: 13pt;
}
</style>