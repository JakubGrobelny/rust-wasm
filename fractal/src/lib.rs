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
    up_to_date: bool,
    fractal: fn(x: f64, y: f64) -> Color,
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

fn clamp_u8(val: usize) -> u8 {
    if val > 255 {
        255
    } else {
        val as u8
    }
}

fn invalid_fractal(_: f64, _: f64) -> Color {
    Color {r: 0, g: 0, b: 0}
}

fn mandelbrot_set(x: f64, y: f64) -> Color {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0f64, 0f64);
    let max_iter : usize = 100;

    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() > 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }

    Color {r: (max_iter - i) as u8, g: i as u8, b: clamp_u8(i*i) as u8}
}

fn newton_fractal(x: f64, y: f64) -> Color {
    // based on https://en.wikipedia.org/wiki/Newton_fractal
    fn function(z: Complex<f64>) -> Complex<f64> {
        let mut z3 = z * z * z;
        z3.re -= 1.0;
        z3
    }

    fn derivative(z: Complex<f64>) -> Complex<f64> {
        3.0 * z * z
    }

    fn newton(x: f64, y: f64) -> Color {
        let mut iteration : usize = 1;
        let max_iter : usize = 128;
        let tolerance : f64 = 0.005;

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
    pub fn new(size: usize, fractal: &str) -> AnimationState {
        utils::set_panic_hook();
        let mut data : Vec<u8> = vec![255; size * size * 4];

        AnimationState {
            shift: (0.0, 0.0), 
            zoom: 1.0,
            size: size,
            data: data,
            up_to_date: false,
            fractal: match fractal {
                "newton" => newton_fractal,
                "mandelbrot" => mandelbrot_set,
                _ => invalid_fractal,
            }
        }
    }

    fn update_pixel(&mut self, col: usize, row: usize) {
        let (x,y) = rescale((col, row), self.shift, self.zoom);
        let f = self.fractal;
        let color = f(x, y);

        let index = translate_coordinates(row, col, self.size);
        self.data[index + R] = color.r;
        self.data[index + G] = color.g;
        self.data[index + B] = color.b;
    }

    fn update_frame(&mut self) {
        let _timer = Timer::new("Animation::update_frame");

        for row in 0..self.size / 2 {
            for col in 0..self.size / 2 {
                self.update_pixel(col, row);
                let col_mirror = self.size - col - 1;
                let row_mirror = self.size - row - 1;
                self.update_pixel(col_mirror, row_mirror);
                self.update_pixel(col_mirror, row);
                self.update_pixel(col, row_mirror);
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


