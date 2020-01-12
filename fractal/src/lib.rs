mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
struct AnimationState {
    shift: f64,
    zoom: f64,
    size: usize,
    data: *const u8,
    up_to_date: bool
}

struct Color {
    r: u8,
    g: u8,
    b: u8
}

const R : usize = 0;
const G : usize = 1;
const B : usize = 2;
const A : usize = 3;
const COLOR_SIZE : usize = 4;

fn find_point_color(x: f64, y: f64) -> Color {
    // TODO:
    Color {r: 64, g: 32, b: 128}
}

fn rescale(point: usize, shift: f64, zoom: f64) -> f64 {
    // TODO:
    point as f64
}

fn translate_coordinates(row: usize, col: usize, size: usize) -> usize {
    (row * size + col) * COLOR_SIZE
}

fn generate_picture(shift: f64, zoom: f64, size: usize) -> *const u8 {
    let mut data : Vec<u8> = vec![255; size * size * 4];

    for row in 0..size {
        for col in 0..size {
            let y = rescale(row, shift, zoom);
            let x = rescale(col, shift, zoom);
            let index = translate_coordinates(row, col, size);
            let color = find_point_color(x, y);
            data[index + R] = color.r;
            data[index + G] = color.g;
            data[index + B] = color.b;
        }
    }

    data.as_ptr()
}

#[wasm_bindgen]
impl AnimationState {
    pub fn new(shift: f64, zoom: f64, size: usize) -> AnimationState {
        AnimationState {
            shift: shift, 
            zoom: zoom,
            size: size,
            data: generate_picture(shift, zoom, size),
            up_to_date: true
        }
    }

    pub fn get_data(&mut self) -> *const u8 {
        if !self.up_to_date {
            self.data = generate_picture(
                self.shift,
                self.zoom,
                self.size
            );
            self.up_to_date = true;
        }

        self.data
    }

    pub fn zoom(&mut self, amount: f64) {
        self.zoom += amount;
        self.up_to_date = false;
    }

    pub fn shift(&mut self, amount: f64) {
        self.shift += amount;
        self.up_to_date = false;
    }
}


