import {pi_by_loop} from "wasm-hello";


const dom = document.getElementById("render-root");

let evaluate = 0
let max = 0
let min = Infinity
let avg = 0.0
setInterval(() => {
    evaluate += 1
    const new_value = pi_by_loop()
    max = (new_value > max) ? new_value : max
    min = (new_value < min) ? new_value : min
    avg = (new_value+avg) / 2
    dom.innerHTML = `Evaluate ${evaluate}<br/> max: ${max}, min: ${min} avg: ${avg}<br/>${new_value}`
}, 50)
