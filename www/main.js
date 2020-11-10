import * as wasm from 'jungle';
import config from './config.json';

const hello = 'hello world';

// setup
let gameState = wasm.GameState.setup();
let canvas = document.querySelector('#canvas');
const context = canvas.getContext('2d');
const width = canvas.width;
const height = canvas.height;
const image = new Image(512, 512);
image.src = gameState.get_image_url();
console.log(gameState.get_image_url())

// game loop
requestAnimationFrame(loop);

function loop() {
    // run
    draw()
    requestAnimationFrame(loop);
}

function draw() {
    context.drawImage(image, 0, 0, 10, 10);
}