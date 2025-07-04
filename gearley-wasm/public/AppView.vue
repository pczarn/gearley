<template>
    <div class="container" id="container">
        <div class="bar">
            <div class="item">
                <select name="load-mode" id="load-mode" v-model="selectedMode">
                    <option v-for="val in loadModes" :value="val">{{ val }}</option>
                </select>
                <select name="load-example" id="load-example">
                    <option v-for="val in loadExamples" :value="val.id">{{ val.name }}</option>
                </select>
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
            </div>
        </div>
        <div class="main-row">
            <div class="left-box">
                <div id="full-editor"></div>
            </div>
            <div class="right-box">
                <div id="result">
                    <Result :result="result"></Result>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import Result from './Result.vue'

export default {
    components: {
        Result
    },
    data() {
        return {
            editor: null,
            result: '',
            selectedMode: 'basic',
            loadModes: [
                'advanced',
                'basic'
            ],
            allLoadExamples: [
                {
                    id: 'test',
                    name: 'Test v0.1',
                    mode: 'basic',
                    content: `<input>a b c d</input>\n<grammar>test ::= a b c d;</grammar>`
                },
                {
                    id: 'adv',
                    name: 'Tokenizing v0.1',
                    mode: 'advanced',
                    content: `<input>a b c d</input>\n\n<grammar>test ::= "a b c d";</grammar>`
                }
            ]
        }
    },
    computed: {
        loadExamples() {
            return this.allLoadExamples.filter((val) => val.mode === this.selectedMode)
        }
    },
    mounted() {
        if (typeof window.log_error === 'undefined') {
            window.log_error = function(text) {
                this.result = text;
            }
        }

        this.editor = ace.edit("full-editor");
        this.editor.setTheme("ace/theme/monokai");
        this.editor.session.setMode("ace/mode/javascript");
        const input = this.loadExamples[0].content
        this.editor.setValue(input)
        this.editor.getSession().on('change', this.update);
        this.processInput(input)
    },
    methods: {
        update() {
            const input = this.editor.getValue();
            this.processInput(input)
        },
        processInput(input) {
            const matchedInput = input.match(/<input>([\s\S]+)<\/input>/m)
            const matchedGrammar = input.match(/<grammar>([\s\S]+)<\/grammar>/m)
            if (!matchedInput || !matchedGrammar || typeof(matchedInput[1]) !== 'string' || typeof(matchedGrammar[1]) !== 'string') {
                this.result = 'Error: could not find <input> or <grammar>'
                return
            }

            function parseWithWasm(input, grammar, mode) {
                if (mode === 'basic' || mode === 'advanced') {
                    return window._parse(input, grammar, mode)
                } else {
                    return 'Unknown mode'
                }
            }

            try {
                this.result = parseWithWasm(matchedInput[1], matchedGrammar[1], this.selectedMode);
            } catch (e) {
                this.result = e.message;
            }
        }
    }
}
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

    flex: 0 0 40px;
    padding: 5px;
    border-bottom: gray solid 4px;
}

.bar .item select {
    margin: 7px 7px;
    padding: 3px 3px;
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
}

.left-box, .right-box {
    flex: 1;
    overflow: auto;
    padding: 20px;
}

.left-box {
    border-right: gray solid 2px;
    position: relative;
}

.right-box {
    background-color: #eee;
    border-left: gray solid 2px;
    font-family: 'Courier New', Courier, monospace;
}

img.github-icon {
    width: 22px;
    height: 22px;
    margin: 5px 5px;
}
</style>