import { defineStore } from 'pinia'

export const useParse = defineStore('parse', {
  state: () => ({
    result: '',
    limit: 10000
  }),
  actions: {
    setResult(result) {
      this.result = result
    }
  },
  getters: {
    logs: (state) => {
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
        let lines = state.result.split("\n")
        if (lines.length > state.limit) {
            lines = lines.slice(0, state.limit)
        }
        let logs = {}
        let lastLog = null
        for (const line of lines) {
            const myMatch = line.match(/^\[TRACE\] - ([\w\.]+): (\w+) (.+)$/)
            if (myMatch === null) {
                if (lastLog) {
                    lastLog[3] += line
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
            const pathList = path.split(".")
            if (typeof logs[pathList[0]] === 'undefined') {
                logs[pathList[0]] = []
            }
            if (pathList.length === 2) {
                let ary = [pathList[1], kind, evaled, line]
                logs[pathList[0]].push(ary)
                lastLog = ary
            } else if (pathList.length === 1) {
                let ary = [pathList[0], kind, evaled, line]
                logs[pathList[0]].push(ary)
                lastLog = ary
            } else {
                console.error("Wrong path list provided by trace")
            }
        }
        return logs
    },
    names() {
        let mapping = this.logs['to_external']
        mapping = mapping && mapping[0] && mapping[0][2]
        if (typeof mapping !== 'object') {
            return []
        }
        return mapping.map(sym_with_name => sym_with_name.name && sym_with_name.name.name)
    },
    rules() {
        let cfg = this.logs['sort_rules_by_lhs']
        cfg = cfg && cfg[0] && cfg[0][2]
        if (typeof cfg !== 'object') {
            return []
        }
        return cfg.rules
    }
  }
})
