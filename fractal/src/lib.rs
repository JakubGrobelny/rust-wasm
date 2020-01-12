mod utils;

use wasm_bindgen::prelude::*;
extern crate num;
use num::complex::Complex;

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
    shift: (f64, f64),
    zoom: f64,
    size: usize,
    data: Vec<u8>,
    up_to_date: bool
}

#[derive(Clone, Copy)]
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
    // based on https://en.wikipedia.org/wiki/Newton_fractal
    fn function(z: Complex<f64>) -> Complex<f64> {
        z * z * z - Complex::new(1.0, 0.0)
    }

    fn derivative(z: Complex<f64>) -> Complex<f64> {
        3.0 * z * z
    }

    fn newton(x: f64, y: f64) -> Color {
        let mut iteration : usize = 1;
        let max_iter : usize = 70;
        let tolerance : f64 = 0.001;
        let roots : [Complex<f64>; 3] = [
            Complex::new( 1.0,  0.0),
            Complex::new(-0.5,  3f64.sqrt() / 2.0),
            Complex::new(-0.5, -3f64.sqrt() / 2.0)
        ];

        let colors : [Color; 3] = [
            Color {r: 255, g: 0,   b: 0  },
            Color {r: 0,   g: 255, b: 0  },
            Color {r: 0,   g: 0,   b: 255}
        ];

        let mut closest : usize = 4;
        let mut z = Complex::new(x, y);

        'outer: while iteration < max_iter {
            z -= function(z) / derivative(z);
            for i in 0..roots.len() {
                let diff = z - roots[i];
                if diff.re.abs() < tolerance && diff.im.abs() < tolerance {
                    closest = i;
                    break 'outer;
                }
            }

            iteration += 1;
        }

        if closest <= 3 { 
            colors[closest].clone()
            // let color = colors[closest];
            // Color {
            //     r: color.r / (iteration as u8 / 10) * 4, 
            //     g: color.g / (iteration as u8 / 10) * 4, 
            //     b: color.b / (iteration as u8 / 10) * 4,
            // }
        } else {
            Color {r: 0, g: 0, b: 0}
        }
    }

    newton(x, y)
}

fn rescale(point: (usize, usize), shift: (f64, f64), zoom: f64) -> (f64, f64) {
    let (x,y) = point;
    let (shift_x, shift_y) = shift;
    ((x as f64 - shift_x) / zoom, (y as f64 - shift_y) / zoom)
}

fn translate_coordinates(row: usize, col: usize, size: usize) -> usize {
    (row * size + col) * COLOR_SIZE
}

#[wasm_bindgen]
impl AnimationState {
    pub fn new(size: usize) -> AnimationState {
        let mut data : Vec<u8> = vec![255; size * size * 4];

        AnimationState {
            shift: (0.0, 0.0), 
            zoom: 1.0,
            size: size,
            data: data,
            up_to_date: false
        }
    }

    fn update_frame(&mut self) {
        for row in 0..self.size {
            for col in 0..self.size {
                let (x,y) = rescale((col, row), self.shift, self.zoom);
                let color = find_point_color(x, y);

                let index = translate_coordinates(row, col, self.size);
                self.data[index + R] = color.r;
                self.data[index + G] = color.g;
                self.data[index + B] = color.b;
            }
        }
    }

    pub fn get_data(&mut self) -> *const u8 {
        if !self.up_to_date {
            self.update_frame();
            self.up_to_date = true;
        }

        self.data.as_ptr()
    }

    pub fn get_zoom(&self) -> f64 {
        self.zoom
    }

    pub fn get_shift_x(&self) -> f64 {
        self.shift.0
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        if self.zoom != zoom {
            self.zoom = zoom;
            self.up_to_date = false;
        }
    }

    pub fn set_shift_x(&mut self, shift_x: f64) {
        if self.shift.0 != shift_x {
            self.shift.0 = shift_x;
            self.up_to_date = false;
        }
    }

    pub fn set_shift_y(&mut self, shift_y: f64) {
        if self.shift.1 != shift_y {
            self.shift.1 = shift_y;
            self.up_to_date = false;
        }
    }

    pub fn set_shift(&mut self, shift_x: f64, shift_y: f64) {
        self.set_shift_x(shift_x);
        self.set_shift_y(shift_y);
    }

    pub fn get_shift_y(&self) -> f64 {
        self.shift.1
    }

    pub fn zoom_by(&mut self, amount: f64) {
        self.set_zoom(self.zoom + amount)
    }

    pub fn shift_by(&mut self, amount_x: f64, amount_y: f64) {
        self.set_shift(self.shift.0 + amount_x, self.shift.1 + amount_y)
    }
}


