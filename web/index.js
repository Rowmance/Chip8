import * as wasm from 'chip8';

console.log('hello', wasm.add(4, 4));
wasm.main();

function tick() {
    const x = wasm.tick();
    console.log(x.op_code);
    x.free();
}

let paused = false;
let speed = 1;

const frameCallback = () => {
    if (!paused) {
        tickN(speed * 1);
    }
    window.requestAnimationFrame(frameCallback);
}
window.requestAnimationFrame(frameCallback);

function tickN(count) {
    while (count-- > 0) {
        wasm.tick()
    }
}