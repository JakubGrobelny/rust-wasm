import * as fractal from "fractal";
import { memory } from "fractal/fractal_bg"
import { FractalAnimation } from "fractal";

let canvas = document.getElementById("fractal-canvas");

const size = 1024;
canvas.height = size;
canvas.width = size;

const context = canvas.getContext('2d');

const animation = fractal.AnimationState.new(size);

const updateImageData = () => {
    let picture = new Uint8ClampedArray(
        memory.buffer,
        animation.get_data(),
        size * size * 4
    );

    let imageData = new ImageData(picture, size, size);
    context.putImageData(imageData, 0, 0);
}

const updateAnimation = () => {
    let zoom = animation.get_zoom();
    if (zoom > 5.0) {
        zoom = 1.0;
    } else {
        zoom += 0.1;
    }
    animation.set_zoom(zoom);

    let shift = animation.get_shift_x();
    if (shift > 10000) {
        shift = 0.0;
    } else {
        shift += 50.0;
    }
    animation.set_shift(shift, -shift/2);
}

const renderLoop = () => {    
    // updateAnimation();
    updateImageData();
    requestAnimationFrame(renderLoop);
};

animation.set_zoom(0.0001);
animation.set_shift(0, -750);
renderLoop();