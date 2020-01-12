import * as fractal from "fractal";
import { memory } from "fractal/fractal_bg"
import { FractalAnimation } from "fractal";

let canvas = document.getElementById("fractal-canvas");

const size = 1024;
canvas.height = size;
canvas.width = size;

const context = canvas.getContext('2d');

const animation = fractal.AnimationState.new(0.0, 1.0, size);

const renderLoop = () => {
    let pixels = new Uint8ClampedArray(
        memory.buffer,
        animation.get_data(),
        size * size * 4
    );
    let imageData = new ImageData(pixels, size, size);
    context.putImageData(imageData, 0, 0);
    requestAnimationFrame(renderLoop);
};

renderLoop();