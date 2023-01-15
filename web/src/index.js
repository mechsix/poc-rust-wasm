import * as wasm from "wasm-hello";


window.addEventListener('load', function () {
    console.log("page loaded")
    wasm.greet()
}, false);

