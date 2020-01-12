mod utils;

use wasm_bindgen::prelude::*;

extern crate num;
use num::complex::Complex;

extern crate web_sys;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
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
        let max_iter : usize = 64;
        let tolerance : f64 = 0.003;

        let roots : [(f64,f64); 3] = [
            (1.0, 0.0),
            (-0.5, 3f64.sqrt() / 2.0),
            (-0.5, -3f64.sqrt() / 2.0)
        ];

        let mut closest : usize = 3;
        let mut z = Complex::new(x, y);

        'outer: while iteration < max_iter {
            z -= function(z) / derivative(z);
            for i in 0..roots.len() {
                let im_diff = (roots[i].1 - z.im).abs();
                let re_diff = (roots[i].0 - z.re).abs();
                if im_diff < tolerance && re_diff < tolerance {
                    closest = i;
                    break 'outer;
                }
            }

            iteration += 1;
        }

        fn clamp_u8(val: usize) -> u8 {
            if val > 255 {
                255
            } else {
                val as u8
            }
        }

        let col0 = 255 - clamp_u8(iteration * iteration / 4);
        match closest {
            0 => Color {r: col0, g: col0, b: 0},
            1 => Color {r: 0, g: col0, b: col0},
            2 => Color {r: col0, g: col0/2, b: col0},
            _ => Color {r: 0, g: 0, b: 0},
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
        utils::set_panic_hook();
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
        let _timer = Timer::new("Animation::update_frame");
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


