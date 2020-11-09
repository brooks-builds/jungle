import * as wasm from 'jungle';
import config from './config.json';

// setup
let canvas = document.querySelector('canvas');
const context = canvas.getContext('2d');
const width = canvas.width;
const height = canvas.height;
const image = new Image(512, 512);
image.src = "./KawaiiIcons_NoBG037.png";

// // game loop
// requestAnimationFrame(loop);

// function loop() {
//     run
//     requestAnimationFrame(loop);
// }

// function draw() {
//     context.drawImage(image, 0, 0, 10, 10);
// }

wasm.run();