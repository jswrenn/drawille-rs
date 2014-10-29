//! Terminal graphics using Braille characters
//!
//! This module provides an interface for utilising Braille characters to draw a picture to a
//! terminal, allowing for much smaller pixels but losing proper colour support.

use std::char;
use std::cmp;
use std::fmt::{Show, Formatter, FormatError};
static PIXEL_MAP: [[int, ..2], ..4] = [[0x01, 0x08],
                                       [0x02, 0x10],
                                       [0x04, 0x20],
                                       [0x40, 0x80]];

/// A canvas object that can be used to draw to the terminal using Braille characters.
#[deriving(Clone, PartialEq, Eq)]
pub struct Canvas {
    chars: Vec<int>,
    width:  uint,
    height: uint,
}

impl Canvas {
    /// Creates a new `Canvas` with the given width and height.
    ///
    /// Note that the `Canvas` can still draw outside the given dimensions (expanding the canvas)
    /// if a pixel is set outside the dimensions.
    pub fn new(width: uint, height: uint) -> Canvas {
        Canvas {
            chars: Vec::new(),
            width: width / 2,
            height: height / 4,
        }
    }

    /// Clears the canvas.
    pub fn clear(&mut self) {
        self.chars.clear();
    }

    /// Sets a pixel at the specified coordinates.
    pub fn set(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        let index = row*self.width + col;
        *self.chars.get_mut(index) |= PIXEL_MAP[y % 4][x % 2];
    }

    /// Deletes a pixel at the specified coordinates.
    pub fn unset(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        let index = row*self.width + col;
        *self.chars.get_mut(index) &= PIXEL_MAP[y % 4][x % 2];
    }

    /// Toggles a pixel at the specified coordinates.
    pub fn toggle(&mut self, x: uint, y: uint) {
        let (row, col) = (x / 2, y / 4);
        let index = row*self.width + col;
        *self.chars.get_mut(index) ^= PIXEL_MAP[y % 4][x % 2];
    }

    /// Detects whether the pixel at the given coordinates is set.
    pub fn get(&self, x: uint, y: uint) -> bool {
        let dot_index = PIXEL_MAP[y % 4][x % 2];
        let (row, col) = (x / 2, y / 4);
        let index = row*self.width + col;
        let c = self.chars[index];
        return c & dot_index != 0;
    }
}

//printf("%c[2J",27);
impl Show for Canvas {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        for (i, c) in self.chars.iter().enumerate() {
            if i % self.width == 0 {
                try!(write!(fmt,"\n"));
            }
            try!(write!(fmt, "{}", *c as int));
        }
        Ok(())
    }
}
