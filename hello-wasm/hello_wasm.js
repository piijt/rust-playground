const wasmModule = require('./pkg/wasm.js');

// console.log({wasmModule});
const hello = wasmModule.greet("WebAssembly");
console.log({hello})
