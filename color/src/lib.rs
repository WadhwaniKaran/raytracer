use std::fs::File;
use std::io::{BufWriter, Write};

pub struct OColor {
    e: [u8; 3],
}

impl OColor {
    pub fn new() -> Self {
        Self {
            e: [0, 0, 0],
        }
    }

    pub fn with(r: u8, g: u8, b: u8) -> Self {
        Self {
            e: [r, g, b],
        }
    }

    pub fn add(&self, other: OColor) -> OColor {
        let r = match u8::checked_add(self.e[0], other.e[0]) {
            Some(c) => c,
            None => 255_u8,
        };
        let g = match u8::checked_add(self.e[1], other.e[1]) {
            Some(c) => c,
            None => 255_u8,
        };
        let b = match u8::checked_add(self.e[2], other.e[2]) {
            Some(c) => c,
            None => 255_u8,
        };
        OColor::with(r, g, b)
    }

    pub fn sub(&self, other: OColor) -> OColor {
        let r = match u8::checked_sub(self.e[0], other.e[0]) {
            Some(c) => c,
            None => 0_u8,
        };
        let g = match u8::checked_sub(self.e[1], other.e[1]) {
            Some(c) => c,
            None => 0_u8,
        };
        let b = match u8::checked_sub(self.e[2], other.e[2]) {
            Some(c) => c,
            None => 0_u8,
        };
        OColor::with(r, g, b)
    }

    pub fn mul(&self, other: u8) -> OColor {
        let r = match u8::checked_mul(self.e[0], other) {
            Some(c) => c,
            None => 255_u8,
        };
        let g = match u8::checked_mul(self.e[1], other) {
            Some(c) => c,
            None => 255_u8,
        };
        let b = match u8::checked_mul(self.e[2], other) {
            Some(c) => c,
            None => 255_u8,
        };
        OColor::with(r, g, b)
    }

    pub fn div(&self, other: u8)-> OColor {
        let r = match u8::checked_mul(self.e[0], other) {
            Some(c) => c,
            None => 0_u8,
        };
        let g = match u8::checked_div(self.e[1], other) {
            Some(c) => c,
            None => 0_u8,
        };
        let b = match u8::checked_div(self.e[2], other) {
            Some(c) => c,
            None => 0_u8,
        };
        OColor::with(r, g, b)
    }

    pub fn write_color(&self, stream: &mut BufWriter<File>) {
        let content = format!("{} {} {}\n", self.e[0], self.e[1], self.e[2]);
        stream.write(content.as_bytes()).unwrap();
    }
}