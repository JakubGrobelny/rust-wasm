import * as fractal from "fractal";
import { memory } from "fractal/fractal_bg"
import { FractalAnimation } from "fractal";

let canvas = document.getElementById("fractal-canvas");

const size = 1024;
canvas.height = size;
canvas.width = size;

const context = canvas.getContext('2d');

const animation = fractal.AnimationState.new(size, "newton");

const updateImageData = () => {
    let picture = new Uint8ClampedArray(
        memory.buffer,
        animation.get_data(),
        size * size * 4
    );

    let imageData = new ImageData(picture, size, size);
    context.putImageData(imageData, 0, 0);
}
// for mandelbrot set
// let zoom = 200.0;
// let maxZoom = 300;
// let minZoom = 50;
// let deltaZoom = 1;

// for newton fractal:
let zoom = 1.0;
let maxZoom = 100.0;
let minZoom = 0.1;
let deltaZoom = 0.1;


let shift_x = 600;
let shift_y = 500;

const updateAnimation = () => {
    if (zoom < minZoom || zoom > maxZoom) {
        deltaZoom *= -1;
    }
    
    zoom += deltaZoom;

    animation.set_zoom(zoom);
}

const renderLoop = () => {
    updateAnimation();
    updateImageData();
    requestAnimationFrame(renderLoop);
};

animation.set_zoom(zoom);
animation.set_shift(shift_x, shift_y);
renderLoop();