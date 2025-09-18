import init, { parse } from "@/assets/pkg/gearley_wasm.js";

const loadModes = [
    'advanced',
    'basic',
    'c-lexer'
]

function parseWithWasm(input, grammar, mode) {
    if (loadModes.find((m) => m === mode)) {
        return parse(input, grammar, mode)
    } else {
        return 'Unknown mode'
    }
}

async function init_wasm_in_worker() {
  console.log('Initializing worker');

  // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
  await init();
  console.log('Worker intialized');

  // Set callback to handle messages passed to the worker.
  self.onmessage = async ({ data }) => {
    let result = ''
    try {
        result = parseWithWasm(data[0], data[1], data[2]);
    } catch (e) {
        console.error(e)
        result = "Caught\n" + e.message;
    }
    postMessage(result)
}
};

init_wasm_in_worker();